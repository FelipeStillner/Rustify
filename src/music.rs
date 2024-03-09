use std::{io::BufReader, thread::sleep, time::Duration};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::io;

#[derive(Debug, Clone)]
pub struct Music {
    pub id: i64,
    pub name: String,
    pub filename: String,
    pub duration: i64,
}

impl Music {
    pub fn new(id: i64, name: &str, filename: &str, duration: i64) -> Self {
        Music {
            id: id,
            name: String::from(name),
            filename: filename.to_string(),
            duration: duration,
        }
    }
    pub fn play(&self) {
        let (tx, rx) = mpsc::channel();
        let th1 = thread::spawn(move || {
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                tx.send(input).unwrap();
            }
        });
        let path = String::from("musics/") + self.filename.as_str();
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let file = std::fs::File::open(path.to_string()).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink.play();
        while !sink.empty() {
            match rx.try_recv() {
                Ok(s) => {
                    let input: Vec<&str>  = s.split_ascii_whitespace().collect();
                    match input[0].trim().to_ascii_lowercase().as_str() {
                        "pause" => {
                            sink.pause();
                        },
                        "play" => {
                            sink.play();
                        },
                        "next" => {
                            break;
                        },
                        "volume" => {
                            sink.set_volume(input[1].parse::<f32>().unwrap());
                        }
                        _ => {}
                    }
                }
                _ =>{}
            } 
        }
        drop(th1);
    }
}
