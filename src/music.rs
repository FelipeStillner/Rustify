use crossterm::terminal::Clear;
use crossterm::{ExecutableCommand, QueueableCommand};
use rodio::Sink;
use std::io::{BufReader, Stdout, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, current, sleep, Thread};
use std::time::{Duration, Instant, SystemTime};

use crate::*;

#[derive(Debug, Clone)]
pub struct Music {
    pub id: i64,
    pub name: String,
    pub filename: String,
    pub duration: i64,
}

impl Music {
    pub fn new(i: i64, n: &str, f: &str, d: i64) -> Self {
        Music {
            id: i,
            name: String::from(n),
            filename: f.to_string(),
            duration: d,
        }
    }
}

impl Play for Music {
    fn play(&self) {
        let mut stdout = std::io::stdout();
        self.reset_terminal(&mut stdout);

        let (sender, receiver): (Sender<Input>, Receiver<Input>) = mpsc::channel();
        let thread = thread::spawn(|| <Music as Play>::get_input(sender));

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let path = String::from("musics/") + self.filename.as_str();
        let file = std::fs::File::open(path.to_string()).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink.play();

        let mut status = Status::new();

        let mut instant = Instant::now();
        while !sink.empty() {
            let dt: Duration = instant.elapsed();
            instant = Instant::now();
            status.update(dt, sink.is_paused());
            match receiver.try_recv() {
                Ok(i) => {
                    match i {
                        Input::PAUSEPLAY => {
                            if sink.is_paused() {
                                sink.play();
                            } else {
                                sink.pause();
                            }
                        }
                        Input::NEXT => {
                            break;
                        }
                        Input::HELP => {}
                        Input::SETVOLUME(v) => {
                            sink.set_volume(v);
                        }
                        Input::QUIT => {
                            break;
                        }
                        Input::STATUS => {
                            self.print_status_terminal(&mut stdout, &status);
                        }
                        Input::INVALID => {
                            stdout.execute(crossterm::cursor::MoveTo(0, 1));
                            stdout.execute(crossterm::terminal::Clear(
                                crossterm::terminal::ClearType::CurrentLine,
                            ));
                            print!("Invalid Input");
                            stdout.execute(crossterm::cursor::MoveTo(0, 2));
                            stdout.execute(crossterm::terminal::Clear(
                                crossterm::terminal::ClearType::CurrentLine,
                            ));
                        }
                    }
                    stdout.execute(crossterm::cursor::MoveTo(0, 2));
                    stdout.execute(crossterm::terminal::Clear(
                        crossterm::terminal::ClearType::CurrentLine,
                    ));
                }
                _ => {}
            }
        }
        drop(thread);
        sink.clear();
    }
    fn reset_terminal(&self, stdout: &mut std::io::Stdout) {
        stdout.execute(crossterm::terminal::SetSize(50, 5));
        stdout.execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ));

        let title = format!("Playing Music: {}", self.name);
        stdout.execute(crossterm::terminal::SetTitle(title));

        stdout.execute(crossterm::cursor::MoveTo(0, 0));
        let static_line = format!("Music: {}, Duration: {}", self.name, self.duration);
        print!("{}", static_line);

        self.print_status_terminal(stdout, &Status::new());
    }
    fn print_status_terminal(&self, stdout: &mut std::io::Stdout, status: &Status) {
        stdout.execute(crossterm::cursor::MoveTo(0, 1));
        stdout.execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::CurrentLine,
        ));
        let status_line = format!(
            "time: {}:{}, pause: {}",
            status.time.as_secs() / 60,
            status.time.as_secs() % 60,
            status.paused
        );
        print!("{}", status_line);

        stdout.execute(crossterm::cursor::MoveTo(0, 2));
        stdout.execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::CurrentLine,
        ));
    }
}
