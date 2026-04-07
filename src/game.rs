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
    QuestionRequest,
    Question(Question),
    Answer(usize),
    AnswerResult { correct: bool, score: u32 },
    NextQuestion,
    GameOver { final_score: u32 },
    TimerStart { seconds: u32 },
    TimerTick { remaining: u32 },
    Chat { player: String, text: String },
}
