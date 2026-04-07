use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;

pub struct InputHandler;

impl InputHandler {
    pub fn start() -> mpsc::Receiver<char> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut stdin = io::stdin();
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
