use std::net::UdpSocket;
use std::io::{self, Write, Read};
use std::time::Duration;
use std::thread;
use termigame_pong::game::{Message, Question};
use termigame_pong::renderer::Renderer;

fn main() -> std::io::Result<()> {
    println!("╔════════════════════════════════════════╗");
    println!("║  🎮 TERMIGAME QUIZ - Client  🎮      ║");
    println!("╚════════════════════════════════════════╝\n");

    print!("Adresse du serveur (défaut: 127.0.0.1): ");
    io::stdout().flush()?;
    let mut server_addr = String::new();
    io::stdin().read_line(&mut server_addr)?;
    let server_addr = if server_addr.trim().is_empty() {
        "127.0.0.1:9999".to_string()
    } else {
        format!("{}:9999", server_addr.trim())
    };

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    socket.connect(&server_addr)?;

    println!("\n✓ Connecté au serveur: {}\n", server_addr);
    println!("Appuyez sur Entrée pour commencer...");
    let _ = io::stdin().read_line(&mut String::new());

    let mut current_question: Option<Question> = None;
    let mut current_num = 0;
    let total_questions = 5;

    loop {
        if current_question.is_none() {
            current_num += 1;
            
            // Demande une question
            if let Ok(data) = bincode::serialize(&Message::QuestionRequest) {
                let sent = socket.send(&data);
                eprintln!("📤 QuestionRequest envoyé: {:?}", sent);
            }

            // Attend la réponse du serveur
            let mut buf = [0; 512];
            loop {
                eprintln!("⏳ En attente de réponse...");
                match socket.recv(&mut buf) {
                    Ok(n) => {
                        eprintln!("📥 Réponse reçue ({} bytes)", n);
                        if let Ok(msg) = bincode::deserialize::<Message>(&buf[..n]) {
                            match msg {
                                Message::Question(q) => {
                                    current_question = Some(q);
                                    Renderer::draw_question(
                                        current_question.as_ref().unwrap(),
                                        current_num,
                                        total_questions,
                                    );
                                    break;
                                }
                                Message::GameOver { final_score } => {
                                    Renderer::draw_game_over(final_score);
                                    println!("\n👋 Fin du jeu!");
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                        break;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                        // Timeout - continue à attendre
                        eprintln!("⏱️ Timeout - en attente...");
                        continue;
                    }
                    Err(e) => {
                        eprintln!("❌ Erreur de connexion: {} ({})", e, e.kind());
                        println!("Erreur de connexion au serveur");
                        return Ok(());
                    }
                }
            }
        } else {
            // En attente de réponse du joueur
            if let Ok(answer_str) = read_input_timeout(Duration::from_millis(100)) {
                if let Ok(choice) = answer_str.trim().parse::<usize>() {
                    if choice >= 1 && choice <= 4 {
                        let choice_idx = choice - 1;
                        
                        // Envoie la réponse
                        if let Ok(data) = bincode::serialize(&Message::Answer(choice_idx)) {
                            socket.send(&data).ok();
                        }

                        // Attend le résultat
                        let mut buf = [0; 512];
                        if let Ok(n) = socket.recv(&mut buf) {
                            if let Ok(msg) = bincode::deserialize::<Message>(&buf[..n]) {
                                match msg {
                                    Message::AnswerResult { correct, score } => {
                                        Renderer::draw_result(correct, score);
                                        thread::sleep(Duration::from_secs(2));
                                        
                                        // Demande la prochaine question
                                        if let Ok(data) = bincode::serialize(&Message::NextQuestion) {
                                            socket.send(&data).ok();
                                        }
                                        
                                        current_question = None;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn read_input_timeout(timeout: Duration) -> io::Result<String> {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buffer = [0; 2];
        if let Ok(n) = stdin.read(&mut buffer) {
            if n > 0 {
                let s = std::str::from_utf8(&buffer[..n])
                    .unwrap_or("")
                    .to_string();
                tx.send(s).ok();
            }
        }
    });

    match rx.recv_timeout(timeout) {
        Ok(input) => Ok(input),
        Err(_) => Err(io::Error::new(io::ErrorKind::WouldBlock, "timeout")),
    }
}
