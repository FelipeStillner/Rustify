use std::{
    io::{BufRead, Read, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::ExecutableCommand;
use rodio::Sink;

use crate::*;

pub trait Play {
    fn play(&self);
    fn get_input(sender: std::sync::mpsc::Sender<Input>) {
        loop {
            let mut stdout = std::io::stdout();
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let s: Vec<&str> = s.split_ascii_whitespace().collect();
            if s.len() > 0 {
                match s[0].trim().to_ascii_lowercase().as_str() {
                    "p" => {
                        sender.send(Input::PAUSEPLAY).unwrap();
                    }
                    "n" => {
                        sender.send(Input::NEXT).unwrap();
                    }
                    "v" => {
                        let value = s[1].parse::<f32>();
                        match value {
                            Ok(v) => {
                                sender.send(Input::SETVOLUME(v)).unwrap();
                            }
                            Err(_) => {
                                println!("Invalid Value for volume")
                            }
                        }
                    }
                    "h" => {
                        sender.send(Input::HELP).unwrap();
                    }
                    "q" => {
                        sender.send(Input::QUIT).unwrap();
                    }
                    "s" => {
                        sender.send(Input::STATUS).unwrap();
                    }
                    _ => {
                        sender.send(Input::INVALID).unwrap();
                    }
                }
            } else {
                stdout.execute(crossterm::cursor::MoveTo(0, 1));
                stdout.execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::CurrentLine,
                ));
                print!("No Input");
                stdout.execute(crossterm::cursor::MoveTo(0, 2));
                stdout.execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::CurrentLine,
                ));
            }
        }
    }
    fn reset_terminal(&self, stdout: &mut std::io::Stdout);
    fn print_status_terminal(&self, stdout: &mut std::io::Stdout, status: &Status);
}
