use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::collections::HashMap;
use termigame_pong::game::{Message, Question};

const QUESTION_TIME: u32 = 20;
const MAX_POINTS: u32 = 10;
const ROUND_PAUSE_SECS: u64 = 3;

enum ClientMsg {
    Message(usize, Message), // (client_id, message)
    Disconnected(usize),
}

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
    id: usize,
    name: String,
    score: u32,
    ready: bool,
    answered: bool,
    stream: Arc<Mutex<TcpStream>>,
}

#[derive(PartialEq)]
enum Phase { Lobby, RoundPause(Instant), Game }

fn send_msg(stream: &mut TcpStream, msg: &Message) -> std::io::Result<()> {
    if let Ok(data) = bincode::serialize(msg) {
        let len = data.len() as u32;
        stream.write_all(&len.to_le_bytes())?;
        stream.write_all(&data)?;
        stream.flush()?;
    }
    Ok(())
}

fn broadcast(players: &[Player], msg: &Message) {
    if let Ok(data) = bincode::serialize(msg) {
        let len = data.len() as u32;
        for p in players {
            if let Ok(mut stream) = p.stream.lock() {
                let _ = stream.write_all(&len.to_le_bytes());
                let _ = stream.write_all(&data);
                let _ = stream.flush();
            }
        }
    }
}

fn broadcast_lobby(players: &[Player]) {
    let state: Vec<(String, bool)> = players.iter().map(|p| (p.name.clone(), p.ready)).collect();
    broadcast(players, &Message::LobbyState { players: state });
}

fn start_question(players: &[Player], idx: usize, questions: &[Question], phase: &mut Phase, q_start: &mut Option<Instant>) {
    let q = questions[idx].clone();
    broadcast(players, &Message::QuestionMsg {
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

fn read_client_loop(client_id: usize, mut stream: TcpStream, tx: mpsc::Sender<ClientMsg>) {
    let mut len_buf = [0u8; 4];
    loop {
        match stream.read_exact(&mut len_buf) {
            Ok(_) => {
                let len = u32::from_le_bytes(len_buf) as usize;
                if len > 65536 { break; } // sanity check
                let mut data = vec![0u8; len];
                if stream.read_exact(&mut data).is_err() { break; }
                if let Ok(msg) = bincode::deserialize::<Message>(&data) {
                    if tx.send(ClientMsg::Message(client_id, msg)).is_err() { break; }
                }
            }
            Err(_) => break,
        }
    }
    let _ = tx.send(ClientMsg::Disconnected(client_id));
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9999")?;
    println!("GT Server on 0.0.0.0:9999");
    println!("Waiting for players...");

    let questions = get_questions();
    let mut players: Vec<Player> = Vec::new();
    let mut players_by_id: HashMap<usize, usize> = HashMap::new(); // id -> player index
    let mut phase = Phase::Lobby;
    let mut q_idx: usize = 0;
    let mut q_start: Option<Instant> = None;
    let mut next_client_id = 0;
    let (tx, rx) = mpsc::channel::<ClientMsg>();

    listener.set_nonblocking(true)?;

    loop {
        // Accept new connections
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New connection from {}", addr);
                let client_id = next_client_id;
                next_client_id += 1;

                let stream_clone = stream.try_clone()?;
                let tx_clone = tx.clone();
                thread::spawn(move || {
                    read_client_loop(client_id, stream_clone, tx_clone);
                });

                let stream_arc = Arc::new(Mutex::new(stream));
                let player_idx = players.len();
                players_by_id.insert(client_id, player_idx);
                players.push(Player {
                    id: client_id,
                    name: format!("Player{}", client_id),
                    score: 0,
                    ready: false,
                    answered: false,
                    stream: stream_arc,
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(_) => break,
        }

        // Timer expired: force unanswered players and advance
        if let Phase::Game = &phase {
            if let Some(start) = q_start {
                if start.elapsed().as_secs() >= QUESTION_TIME as u64 {
                    let correct_answer = questions[q_idx].correct;
                    for p in players.iter_mut().filter(|p| !p.answered) {
                        let score = p.score;
                        if let Ok(mut stream) = p.stream.lock() {
                            let _ = send_msg(&mut stream, &Message::AnswerResult {
                                correct: false, points: 0, score, correct_answer,
                            });
                        }
                        p.answered = true;
                    }
                    let scores = show_scores(&players);
                    q_idx += 1;
                    for p in players.iter_mut() { p.answered = false; }
                    if q_idx >= questions.len() {
                        broadcast(&players, &Message::GameOver { scores });
                        println!("Game over. Back to lobby.");
                        q_idx = 0;
                        for p in players.iter_mut() { p.score = 0; p.ready = false; }
                        phase = Phase::Lobby;
                        broadcast_lobby(&players);
                    } else {
                        broadcast(&players, &Message::RoundEnd { scores });
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
                start_question(&players, q_idx, &questions, &mut phase, &mut q_start);
                continue;
            }
        }

        // Process messages from clients
        match rx.try_recv() {
            Ok(ClientMsg::Message(cid, msg)) => {
                if let Some(&pidx) = players_by_id.get(&cid) {
                    match msg {
                        Message::Join { name } => {
                            if !matches!(phase, Phase::Lobby) { continue; }
                            println!("+ {} joined", name);
                            players[pidx].name = name;
                            broadcast_lobby(&players);
                        }
                        Message::Ready => {
                            if matches!(phase, Phase::Lobby) {
                                players[pidx].ready = !players[pidx].ready;
                                let status = if players[pidx].ready { "ready" } else { "not ready" };
                                println!("* {} is {}", players[pidx].name, status);
                                broadcast_lobby(&players);
                                if !players.is_empty() && players.iter().all(|p| p.ready) {
                                    println!("All ready! Starting...");
                                    broadcast(&players, &Message::GameStart);
                                    q_idx = 0;
                                    for p in players.iter_mut() { p.score = 0; p.answered = false; }
                                    phase = Phase::RoundPause(Instant::now() - Duration::from_secs(ROUND_PAUSE_SECS));
                                }
                            }
                        }
                        Message::Answer(choice) => {
                            if let Phase::Game = &phase {
                                if !players[pidx].answered && q_idx < questions.len() {
                                    let q = &questions[q_idx];
                                    let correct = choice == q.correct;
                                    let time_secs = q_start.map(|s| s.elapsed().as_secs() as u32).unwrap_or(QUESTION_TIME);
                                    let pts = points(correct, time_secs);
                                    players[pidx].score += pts;
                                    players[pidx].answered = true;
                                    let score = players[pidx].score;
                                    let correct_answer = q.correct;

                                    if let Ok(mut stream) = players[pidx].stream.lock() {
                                        let _ = send_msg(&mut stream, &Message::AnswerResult {
                                            correct, points: pts, score, correct_answer,
                                        });
                                    }

                                    if !players.iter().all(|p| p.answered) {
                                        if let Ok(mut stream) = players[pidx].stream.lock() {
                                            let _ = send_msg(&mut stream, &Message::WaitingForOthers);
                                        }
                                    } else {
                                        let scores = show_scores(&players);
                                        q_idx += 1;
                                        for p in players.iter_mut() { p.answered = false; }
                                        if q_idx >= questions.len() {
                                            broadcast(&players, &Message::GameOver { scores });
                                            println!("Game over. Back to lobby.");
                                            q_idx = 0;
                                            for p in players.iter_mut() { p.score = 0; p.ready = false; }
                                            phase = Phase::Lobby;
                                            broadcast_lobby(&players);
                                        } else {
                                            broadcast(&players, &Message::RoundEnd { scores });
                                            phase = Phase::RoundPause(Instant::now());
                                            q_start = None;
                                        }
                                    }
                                }
                            }
                        }
                        Message::Chat { player, text } => {
                            println!("[{}] {}", player, text);
                            broadcast(&players, &Message::Chat { player, text });
                        }
                        _ => {}
                    }
                }
            }
            Ok(ClientMsg::Disconnected(cid)) => {
                if let Some(&pidx) = players_by_id.get(&cid) {
                    println!("Client {} ({}) disconnected", cid, players[pidx].name);
                    players.remove(pidx);
                    players_by_id.retain(|_, &mut idx| idx != pidx);
                    // Update indices in map
                    for (_, idx) in players_by_id.iter_mut() {
                        if *idx > pidx { *idx -= 1; }
                    }
                    broadcast_lobby(&players);
                }
            }
            Err(_) => {}
        }

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
