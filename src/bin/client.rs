use std::net::TcpStream;
use std::io::{self, Write, BufRead, Read};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use termigame_pong::game::{Message, Question};
use termigame_pong::renderer::Renderer;

enum Event {
    Net(Message),
    Input(String),
}

#[derive(PartialEq)]
enum Phase { Lobby, Game, Over }

fn send_msg(stream: &mut TcpStream, msg: &Message) -> std::io::Result<()> {
    if let Ok(data) = bincode::serialize(msg) {
        let len = data.len() as u32;
        stream.write_all(&len.to_le_bytes())?;
        stream.write_all(&data)?;
        stream.flush()?;
    }
    Ok(())
}

fn prompt(timer: Option<(&Instant, u32)>) {
    if let Some((start, duration)) = timer {
        let elapsed = start.elapsed().as_secs() as u32;
        let remaining = duration.saturating_sub(elapsed);
        print!("\r[{}s] Answer (1-4) or chat: ", remaining);
    } else {
        print!("> ");
    }
    io::stdout().flush().ok();
}

fn main() -> std::io::Result<()> {
    print!("Server address (default: 127.0.0.1): ");
    io::stdout().flush()?;
    let mut server_addr = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_line(&mut server_addr)?;
    }
    let server_addr = if server_addr.trim().is_empty() {
        "127.0.0.1:80".to_string()
    } else {
        format!("{}:80", server_addr.trim())
    };

    print!("Your name: ");
    io::stdout().flush()?;
    let mut player_name = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_line(&mut player_name)?;
    }
    let player_name = player_name.trim().to_string();

    let mut stream = TcpStream::connect(&server_addr)?;
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;

    send_msg(&mut stream, &Message::Join { name: player_name.clone() })?;

    let (tx, rx) = mpsc::channel::<Event>();

    // Network thread
    let mut stream_net = stream.try_clone()?;
    let tx_net = tx.clone();
    thread::spawn(move || {
        let mut len_buf = [0u8; 4];
        loop {
            match stream_net.read_exact(&mut len_buf) {
                Ok(_) => {
                    let len = u32::from_le_bytes(len_buf) as usize;
                    if len > 65536 { break; }
                    let mut data = vec![0u8; len];
                    if stream_net.read_exact(&mut data).is_err() { break; }
                    if let Ok(msg) = bincode::deserialize::<Message>(&data) {
                        if tx_net.send(Event::Net(msg)).is_err() { break; }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::TimedOut => {}
                Err(_) => break,
            }
        }
    });

    // Input thread
    let tx_input = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        let handle = stdin.lock();
        for line in handle.lines() {
            match line {
                Ok(l) => { if tx_input.send(Event::Input(l)).is_err() { break; } }
                Err(_) => break,
            }
        }
    });

    println!("\n--- Lobby ---");
    println!("'r' = ready   |   type text = chat");

    let mut phase = Phase::Lobby;
    let mut current_question: Option<Question> = None;
    let mut q_timer: Option<(Instant, u32)> = None;

    loop {
        // Refresh timer display while in game
        if phase == Phase::Game && q_timer.is_some() {
            let (ref start, duration) = q_timer.as_ref().unwrap().clone();
            let elapsed = start.elapsed().as_secs() as u32;
            let remaining = duration.saturating_sub(elapsed);
            print!("\r[{}s] Answer (1-4) or chat: ", remaining);
            io::stdout().flush().ok();
        }

        match rx.recv_timeout(Duration::from_millis(250)) {
            Ok(Event::Net(msg)) => match msg {
                Message::LobbyState { players } => {
                    println!("\n--- Lobby ({}/{} players) ---", players.len(), players.len());
                    for (name, ready) in &players {
                        let status = if *ready { "[READY]" } else { "[     ]" };
                        println!("  {} {}", status, name);
                    }
                    println!("'r' = ready   |   type text = chat");
                }
                Message::GameStart => {
                    println!("\n=== Game starting! ===");
                    phase = Phase::Game;
                }
                Message::QuestionMsg { question, num, total, timer } => {
                    current_question = Some(question.clone());
                    q_timer = Some((Instant::now(), timer));
                    println!("\n--- Question {}/{} ---", num, total);
                    Renderer::draw_question(&question, num as u32, total as u32);
                    prompt(q_timer.as_ref().map(|(s, d)| (s, *d)));
                }
                Message::AnswerResult { correct, points, score, correct_answer } => {
                    println!();
                    if correct {
                        println!("Correct! +{} points  (score: {})", points, score);
                    } else {
                        println!("Wrong! Correct answer was {}  (score: {})", correct_answer + 1, score);
                    }
                    current_question = None;
                    q_timer = None;
                }
                Message::WaitingForOthers => {
                    println!("Waiting for other players...");
                }
                Message::RoundEnd { scores } => {
                    println!("\n--- Scores ---");
                    for (name, s) in &scores {
                        println!("  {:20} {}", name, s);
                    }
                    println!("Next question coming...");
                }
                Message::GameOver { scores } => {
                    println!("\n=== Game Over ===");
                    for (i, (name, s)) in scores.iter().enumerate() {
                        println!("  {}. {:20} {}", i + 1, name, s);
                    }
                    println!("--- Back to lobby ---");
                    println!("'r' = ready   |   type text = chat");
                    phase = Phase::Lobby;
                    current_question = None;
                    q_timer = None;
                }
                Message::Chat { player, text } => {
                    println!("\n[{}] {}", player, text);
                    if let Some(ref qt) = q_timer {
                        let (ref start, duration) = qt;
                        prompt(Some((start, *duration)));
                    }
                }
                _ => {}
            },

            Ok(Event::Input(input)) => {
                if input.is_empty() { continue; }
                match phase {
                    Phase::Lobby => {
                        if input == "r" || input == "R" {
                            let _ = send_msg(&mut stream, &Message::Ready);
                        } else {
                            let _ = send_msg(&mut stream, &Message::Chat { player: player_name.clone(), text: input });
                        }
                    }
                    Phase::Game => {
                        if let Ok(n) = input.parse::<usize>() {
                            if n >= 1 && n <= 4 && current_question.is_some() {
                                let _ = send_msg(&mut stream, &Message::Answer(n - 1));
                                q_timer = None;
                            }
                        } else {
                            let _ = send_msg(&mut stream, &Message::Chat { player: player_name.clone(), text: input });
                        }
                    }
                    Phase::Over => {}
                }
            }

            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(_) => break,
        }
    }

    Ok(())
}
