use crate::*;

pub struct Terminal {
    sender: std::sync::mpsc::Sender<String>,
}

impl Terminal {
    pub fn new() -> (Self, std::sync::mpsc::Receiver<String>) {
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        (Self { sender: sender }, receiver)
    }
    pub fn get_input(&self) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.split_ascii_whitespace().collect::<Vec<&str>>().len() > 0 {
                self.sender.send(input).unwrap();
            } else {
            }
        }
    }
}
