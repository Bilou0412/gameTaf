use std::net::UdpSocket;
use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use termigame_pong::game::{Message, Question};
use termigame_pong::renderer::Renderer;

fn main() -> std::io::Result<()> {
    print!("Server address (default: 127.0.0.1): ");
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

    let mut current_question: Option<Question> = None;
    let mut current_num = 0;
    let total_questions = 5;

    loop {
        if current_question.is_none() {
            current_num += 1;
            
            // Request question
            if let Ok(data) = bincode::serialize(&Message::QuestionRequest) {
                socket.send(&data).ok();
            }

            // Wait for response
            let mut buf = [0; 512];
            loop {
                match socket.recv(&mut buf) {
                    Ok(n) => {
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
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                        break;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                        continue;
                    }
                    Err(_) => {
                        return Ok(());
                    }
                }
            }
        } else {
            // Wait for player answer
            if let Ok(answer_str) = read_input_timeout(Duration::from_secs(10)) {
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
    use std::io::BufRead;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut line = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        if let Ok(_) = handle.read_line(&mut line) {
            if !line.is_empty() {
                tx.send(line).ok();
            }
        }
    });

    match rx.recv_timeout(timeout) {
        Ok(input) => Ok(input),
        Err(_) => Err(io::Error::new(io::ErrorKind::WouldBlock, "timeout")),
    }
}
