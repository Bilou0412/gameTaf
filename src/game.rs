use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: u32,
    pub text: String,
    pub options: [String; 4],
    pub correct: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub current_question_id: u32,
    pub player1_score: u32,
    pub player2_score: u32,
    pub game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            current_question_id: 0,
            player1_score: 0,
            player2_score: 0,
            game_over: false,
        }
    }

    pub fn update(&mut self, _width: i32, _height: i32) {
        // Pas de mise à jour nécessaire pour le quiz
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    // Connection
    Join { name: String },
    // Lobby
    LobbyState { players: Vec<(String, bool)> },
    Ready,
    GameStart,
    // In-game (server pushes to all simultaneously)
    QuestionMsg { question: Question, num: usize, total: usize, timer: u32 },
    Answer(usize),
    AnswerResult { correct: bool, points: u32, score: u32, correct_answer: usize },
    WaitingForOthers,
    RoundEnd { scores: Vec<(String, u32)> },
    GameOver { scores: Vec<(String, u32)> },
    // Chat
    Chat { player: String, text: String },
}
