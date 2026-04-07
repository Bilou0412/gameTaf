use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;

pub struct InputHandler;

impl InputHandler {
    pub fn start() -> mpsc::Receiver<char> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            let mut buffer = [0; 1];
            loop {
                if stdin.read_exact(&mut buffer).is_ok() {
                    tx.send(buffer[0] as char).ok();
                }
            }
        });

        rx
    }
}

pub fn enable_raw_mode() {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    
    // Pour Windows, on peut utiliser `mode con:` pour changer les paramètres
    #[cfg(target_os = "windows")]
    {
        // On va utiliser une approche crossterm à la place
    }
}

pub fn disable_raw_mode() {
    // Restaurer les paramètres initiaux si nécessaire
}
