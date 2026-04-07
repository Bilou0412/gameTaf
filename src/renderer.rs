use crate::game::{GameState, Question};
use std::io::{self, Write};

pub struct Renderer;

impl Renderer {
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }

    pub fn draw(_state: &GameState) {
        Self::clear_screen();
        println!("╔════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                       🎯 QUIZ INTERACTIF - Attendez...                        ║");
        println!("╚════════════════════════════════════════════════════════════════════════════════╝");
        io::stdout().flush().unwrap();
    }

    pub fn draw_question(question: &Question, current_num: u32, total: u32) {
        Self::clear_screen();
        println!("╔════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                       🎯 QUIZ INTERACTIF                                      ║");
        println!("╠════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ Question {}/{}", current_num, total);
        println!("╠════════════════════════════════════════════════════════════════════════════════╣");
        
        let text = format!("║ {}", question.text);
        println!("{:80}", text);
        
        println!("║");
        for (i, option) in question.options.iter().enumerate() {
            let line = format!("║   [{}] {}", i + 1, option);
            println!("{:80}", line);
        }
        
        println!("║");
        println!("║ Tapez le numéro (1-4) pour répondre:");
        println!("╚════════════════════════════════════════════════════════════════════════════════╝");
        io::stdout().flush().unwrap();
    }

    pub fn draw_result(correct: bool, score: u32) {
        Self::clear_screen();
        let result_text = if correct { "✅ CORRECT!" } else { "❌ INCORRECT" };
        println!("╔════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                              {}                                  ║", result_text);
        println!("╠════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ Score: {}", score);
        println!("╠════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ Prochaine question en 2 secondes...");
        println!("╚════════════════════════════════════════════════════════════════════════════════╝");
        io::stdout().flush().unwrap();
    }

    pub fn draw_game_over(final_score: u32) {
        Self::clear_screen();
        println!("╔════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                          🏆 FIN DU JEU 🏆                                     ║");
        println!("╠════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ Score final: {}", final_score);
        println!("║");
        println!("║ Merci d'avoir joué!");
        println!("╚════════════════════════════════════════════════════════════════════════════════╝");
        io::stdout().flush().unwrap();
    }
}
