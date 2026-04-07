use std::net::UdpSocket;
use std::time::Duration;
use termigame_pong::game::{Message, Question};

fn get_questions() -> Vec<Question> {
    vec![
        Question {
            id: 0,
            text: "Quel est le plus grand océan ?".to_string(),
            options: [
                "Océan Atlantique".to_string(),
                "Océan Pacifique".to_string(),
                "Océan Indien".to_string(),
                "Océan Arctique".to_string(),
            ],
            correct: 1,
        },
        Question {
            id: 1,
            text: "En quelle année l'homme a-t-il marché sur la Lune ?".to_string(),
            options: [
                "1965".to_string(),
                "1969".to_string(),
                "1971".to_string(),
                "1975".to_string(),
            ],
            correct: 1,
        },
        Question {
            id: 2,
            text: "Quel est le plus haut sommet du monde ?".to_string(),
            options: [
                "K2".to_string(),
                "Kangchenjunga".to_string(),
                "Mont Everest".to_string(),
                "Denali".to_string(),
            ],
            correct: 2,
        },
        Question {
            id: 3,
            text: "Quelle est la capitale de la France ?".to_string(),
            options: [
                "Lyon".to_string(),
                "Marseille".to_string(),
                "Paris".to_string(),
                "Bordeaux".to_string(),
            ],
            correct: 2,
        },
        Question {
            id: 4,
            text: "Combien de continents existe-t-il ?".to_string(),
            options: [
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
            ],
            correct: 2,
        },
    ]
}

fn get_local_ip() -> String {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            match socket.connect("8.8.8.8:80") {
                Ok(_) => {
                    match socket.local_addr() {
                        Ok(addr) => addr.ip().to_string(),
                        Err(_) => "127.0.0.1".to_string(),
                    }
                }
                Err(_) => "127.0.0.1".to_string(),
            }
        }
        Err(_) => "127.0.0.1".to_string(),
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:9999")?;
    socket.set_read_timeout(Some(Duration::from_millis(100)))?;

    // Get local IP
    let local_ip = UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("8.8.8.8:80").ok();
            s.local_addr()
        })
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    
    println!("GT Server on {}:9999", local_ip);

    let questions = get_questions();

    let mut connected_players: Vec<std::net::SocketAddr> = Vec::new();
    let mut player_scores: std::collections::HashMap<std::net::SocketAddr, u32> = std::collections::HashMap::new();
    let mut current_question_idx = 0;
    let mut buf = [0; 512];

    loop {
        // Receive messages from clients
        if let Ok((n, addr)) = socket.recv_from(&mut buf) {
            if !connected_players.contains(&addr) {
                connected_players.push(addr);
                player_scores.insert(addr, 0);
                println!("Player connected: {} (total: {})", addr, connected_players.len());
            }

            if let Ok(msg) = bincode::deserialize::<Message>(&buf[..n]) {
                match msg {
                    Message::QuestionRequest => {
                        if current_question_idx < questions.len() {
                            let question = questions[current_question_idx].clone();
                            if let Ok(data) = bincode::serialize(&Message::Question(question)) {
                                socket.send_to(&data, addr).ok();
                            }
                        } else {
                            let final_score = *player_scores.get(&addr).unwrap_or(&0);
                            if let Ok(data) = bincode::serialize(&Message::GameOver { final_score }) {
                                socket.send_to(&data, addr).ok();
                            }
                        }
                    }
                    Message::Answer(choice) => {
                        if current_question_idx < questions.len() {
                            let question = &questions[current_question_idx];
                            let is_correct = choice == question.correct;
                            
                            let score = player_scores.entry(addr).or_insert(0);
                            if is_correct {
                                *score += 1;
                            }
                            
                            if let Ok(data) = bincode::serialize(&Message::AnswerResult {
                                correct: is_correct,
                                score: *score,
                            }) {
                                socket.send_to(&data, addr).ok();
                            }
                        }
                    }
                    Message::NextQuestion => {
                        current_question_idx += 1;
                    }
                    _ => {}
                }
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
