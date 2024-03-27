use crossterm::ExecutableCommand;
use rodio::Sink;
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::*;

#[derive(Debug, Clone)]
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
    fn play(&self, receiver: &std::sync::mpsc::Receiver<String>, interface: &mut Interface) {
        //let mut stdout = std::io::stdout();
        //self.reset_terminal(&mut stdout);

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink: Sink = rodio::Sink::try_new(&handle).unwrap();
        for music in &self.musics {
            let path = String::from("musics/") + music.filename.as_str();
            let file = std::fs::File::open(path.to_string()).unwrap();
            sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        }

        sink.play();

        interface.music = self.name.to_string();
        interface.music_duration = self.duration;

        let mut status = Status::new();

        let mut instant = Instant::now();
        while !sink.empty() {
            let dt: Duration = instant.elapsed();
            instant = Instant::now();
            status.update(dt, sink.is_paused());

            interface.update();
            interface.music_time = status.time;
            interface.paused = status.paused;

            match receiver.try_recv() {
                Ok(input) => {
                    interface.clear();
                    let input = manipulate_play_input(input);
                    match input {
                        PlayInput::PAUSEPLAY => {
                            if sink.is_paused() {
                                sink.play();
                            } else {
                                sink.pause();
                            }
                        }
                        PlayInput::NEXT => {
                            sink.skip_one();
                        }
                        PlayInput::HELP => {}
                        PlayInput::SETVOLUME(v) => {
                            sink.set_volume(v);
                        }
                        PlayInput::QUIT => {
                            break;
                        }
                        PlayInput::INVALID(err) => {
                            interface.error(err.as_str());
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
