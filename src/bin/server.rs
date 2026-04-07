use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::io;
use termigame_pong::game::{Message, Question};

const QUESTION_TIME: u32 = 20;
const MAX_POINTS: u32 = 10;
const ROUND_PAUSE_SECS: u64 = 3;

fn get_questions() -> Vec<Question> {
    vec![
        Question { id: 0, text: "Quel est le plus grand océan ?".to_string(),
            options: ["Océan Atlantique".to_string(), "Océan Pacifique".to_string(), "Océan Indien".to_string(), "Océan Arctique".to_string()],
            correct: 1 },
        Question { id: 1, text: "En quelle année l'homme a-t-il marché sur la Lune ?".to_string(),
            options: ["1965".to_string(), "1969".to_string(), "1971".to_string(), "1975".to_string()],
            correct: 1 },
        Question { id: 2, text: "Quel est le plus haut sommet du monde ?".to_string(),
            options: ["K2".to_string(), "Kangchenjunga".to_string(), "Mont Everest".to_string(), "Denali".to_string()],
            correct: 2 },
        Question { id: 3, text: "Quelle est la capitale de la France ?".to_string(),
            options: ["Lyon".to_string(), "Marseille".to_string(), "Paris".to_string(), "Bordeaux".to_string()],
            correct: 2 },
        Question { id: 4, text: "Combien de continents existe-t-il ?".to_string(),
            options: ["5".to_string(), "6".to_string(), "7".to_string(), "8".to_string()],
            correct: 2 },
    ]
}

fn points(correct: bool, time_secs: u32) -> u32 {
    if !correct { return 0; }
    let ratio = (time_secs as f32 / QUESTION_TIME as f32).min(1.0);
    ((MAX_POINTS as f32 * (1.0 - ratio)) as u32).max(1)
}

struct Player {
    addr: std::net::SocketAddr,
    name: String,
    score: u32,
    ready: bool,
    answered: bool,
}

#[derive(PartialEq)]
enum Phase { Lobby, RoundPause(Instant), Game }

fn broadcast(socket: &UdpSocket, players: &[Player], msg: &Message) {
    if let Ok(data) = bincode::serialize(msg) {
        for p in players {
            socket.send_to(&data, p.addr).ok();
        }
    }
}

fn broadcast_lobby(socket: &UdpSocket, players: &[Player]) {
    let state: Vec<(String, bool)> = players.iter().map(|p| (p.name.clone(), p.ready)).collect();
    broadcast(socket, players, &Message::LobbyState { players: state });
}

fn start_question(socket: &UdpSocket, players: &[Player], idx: usize, questions: &[Question], phase: &mut Phase, q_start: &mut Option<Instant>) {
    let q = questions[idx].clone();
    broadcast(socket, players, &Message::QuestionMsg {
        question: q,
        num: idx + 1,
        total: questions.len(),
        timer: QUESTION_TIME,
    });
    *phase = Phase::Game;
    *q_start = Some(Instant::now());
    println!("Q{}/{} sent", idx + 1, questions.len());
}

fn show_scores(players: &[Player]) -> Vec<(String, u32)> {
    let mut scores: Vec<(String, u32)> = players.iter().map(|p| (p.name.clone(), p.score)).collect();
    scores.sort_by(|a, b| b.1.cmp(&a.1));
    scores
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:9999")?;
    socket.set_read_timeout(Some(Duration::from_millis(50)))?;

    let local_ip = UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| { s.connect("8.8.8.8:80").ok(); s.local_addr() })
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    println!("GT Server on {}:9999", local_ip);
    println!("Waiting for players...");

    let questions = get_questions();
    let mut players: Vec<Player> = Vec::new();
    let mut phase = Phase::Lobby;
    let mut q_idx: usize = 0;
    let mut q_start: Option<Instant> = None;
    let mut buf = [0u8; 1024];

    loop {
        // Timer expired: force unanswered players and advance
        if let Phase::Game = &phase {
            if let Some(start) = q_start {
                if start.elapsed().as_secs() >= QUESTION_TIME as u64 {
                    let correct_answer = questions[q_idx].correct;
                    for p in players.iter_mut().filter(|p| !p.answered) {
                        let score = p.score;
                        if let Ok(data) = bincode::serialize(&Message::AnswerResult {
                            correct: false, points: 0, score, correct_answer,
                        }) { socket.send_to(&data, p.addr).ok(); }
                        p.answered = true;
                    }
                    // Advance
                    let scores = show_scores(&players);
                    q_idx += 1;
                    for p in players.iter_mut() { p.answered = false; }
                    if q_idx >= questions.len() {
                        broadcast(&socket, &players, &Message::GameOver { scores });
                        println!("Game over. Back to lobby.");
                        q_idx = 0;
                        for p in players.iter_mut() { p.score = 0; p.ready = false; }
                        phase = Phase::Lobby;
                        broadcast_lobby(&socket, &players);
                    } else {
                        broadcast(&socket, &players, &Message::RoundEnd { scores });
                        phase = Phase::RoundPause(Instant::now());
                        q_start = None;
                    }
                    continue;
                }
            }
        }

        // Round pause elapsed: send next question
        if let Phase::RoundPause(paused_at) = &phase {
            if paused_at.elapsed().as_secs() >= ROUND_PAUSE_SECS {
                let paused_at = *paused_at; // copy to avoid borrow issue
                let _ = paused_at;
                start_question(&socket, &players, q_idx, &questions, &mut phase, &mut q_start);
                continue;
            }
        }

        // Receive messages
        match socket.recv_from(&mut buf) {
            Ok((n, addr)) => {
                let pidx = players.iter().position(|p| p.addr == addr);
                if let Ok(msg) = bincode::deserialize::<Message>(&buf[..n]) {
                    match msg {
                        Message::Join { name } => {
                            if pidx.is_none() {
                                if !matches!(phase, Phase::Lobby) {
                                    // Game in progress, reject
                                    continue;
                                }
                                println!("+ {} joined", name);
                                players.push(Player { addr, name, score: 0, ready: false, answered: false });
                                broadcast_lobby(&socket, &players);
                            }
                        }
                        Message::Ready => {
                            if let Some(i) = pidx {
                                if matches!(phase, Phase::Lobby) {
                                    players[i].ready = !players[i].ready;
                                    let status = if players[i].ready { "ready" } else { "not ready" };
                                    println!("* {} is {}", players[i].name, status);
                                    broadcast_lobby(&socket, &players);
                                    // All ready with at least 1 player → start
                                    if !players.is_empty() && players.iter().all(|p| p.ready) {
                                        println!("All ready! Starting...");
                                        broadcast(&socket, &players, &Message::GameStart);
                                        q_idx = 0;
                                        for p in players.iter_mut() { p.score = 0; p.answered = false; }
                                        // Short pause then first question
                                        phase = Phase::RoundPause(Instant::now() - Duration::from_secs(ROUND_PAUSE_SECS));
                                    }
                                }
                            }
                        }
                        Message::Answer(choice) => {
                            if let (Some(i), Phase::Game) = (pidx, &phase) {
                                if !players[i].answered && q_idx < questions.len() {
                                    let q = &questions[q_idx];
                                    let correct = choice == q.correct;
                                    let time_secs = q_start.map(|s| s.elapsed().as_secs() as u32).unwrap_or(QUESTION_TIME);
                                    let pts = points(correct, time_secs);
                                    players[i].score += pts;
                                    players[i].answered = true;
                                    let score = players[i].score;
                                    let correct_answer = q.correct;

                                    if let Ok(data) = bincode::serialize(&Message::AnswerResult {
                                        correct, points: pts, score, correct_answer,
                                    }) { socket.send_to(&data, addr).ok(); }

                                    let all_answered = players.iter().all(|p| p.answered);
                                    if !all_answered {
                                        if let Ok(data) = bincode::serialize(&Message::WaitingForOthers) {
                                            socket.send_to(&data, addr).ok();
                                        }
                                    } else {
                                        let scores = show_scores(&players);
                                        q_idx += 1;
                                        for p in players.iter_mut() { p.answered = false; }
                                        if q_idx >= questions.len() {
                                            broadcast(&socket, &players, &Message::GameOver { scores });
                                            println!("Game over. Back to lobby.");
                                            q_idx = 0;
                                            for p in players.iter_mut() { p.score = 0; p.ready = false; }
                                            phase = Phase::Lobby;
                                            broadcast_lobby(&socket, &players);
                                        } else {
                                            broadcast(&socket, &players, &Message::RoundEnd { scores });
                                            phase = Phase::RoundPause(Instant::now());
                                            q_start = None;
                                        }
                                    }
                                }
                            }
                        }
                        Message::Chat { player, text } => {
                            println!("[{}] {}", player, text);
                            broadcast(&socket, &players, &Message::Chat { player, text });
                        }
                        _ => {}
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock
                || e.kind() == io::ErrorKind::TimedOut => {}
            Err(_) => {}
        }
    }
}
