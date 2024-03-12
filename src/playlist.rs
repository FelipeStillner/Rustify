use crossterm::ExecutableCommand;
use rodio::Sink;
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::*;

#[derive(Debug)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub duration: i64,
    pub musics: Vec<Music>,
    pub user_id: i64,
}

impl Playlist {
    pub fn new(i: i64, n: &str, u: i64) -> Self {
        Playlist {
            id: i,
            name: n.to_string(),
            duration: 0,
            musics: (vec![]),
            user_id: u,
        }
    }
    pub fn add_music(&mut self, music: Music) {
        self.duration += music.duration;
        self.musics.push(music);
    }
    pub fn remove_music(&mut self, id: i64) {
        for music in self.musics.iter() {
            if music.id == id {
                self.duration -= music.duration;
            }
        }
        self.musics.retain(|x| x.id != id);
    }
}

impl Play for Playlist {
    fn play(&self) {
        let mut stdout = std::io::stdout();
        self.reset_terminal(&mut stdout);

        let (sender, receiver): (Sender<Input>, Receiver<Input>) = mpsc::channel();
        let thread = thread::spawn(|| <Music as Play>::get_input(sender));

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        for music in &self.musics {
            let path = String::from("musics/") + music.filename.as_str();
            let file = std::fs::File::open(path.to_string()).unwrap();
            sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        }

        sink.play();

        let mut status = Status::new();

        while !sink.empty() {
            match receiver.try_recv() {
                Ok(i) => match i {
                    Input::PAUSEPLAY => {
                        if sink.is_paused() {
                            sink.play();
                        } else {
                            sink.pause();
                        }
                    }
                    Input::NEXT => {
                        sink.skip_one();
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
                },
                _ => {}
            }
        }
        drop(thread);
    }
    fn reset_terminal(&self, stdout: &mut std::io::Stdout) {
        todo!()
    }
    fn print_status_terminal(&self, s: &mut std::io::Stdout, status: &Status) {
        todo!()
    }
}
