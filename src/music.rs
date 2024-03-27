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
    fn play(&self, receiver: &std::sync::mpsc::Receiver<String>, interface: &mut Interface) {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let path = String::from("musics/") + self.filename.as_str();
        let file = std::fs::File::open(path.to_string()).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
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
                            break;
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
        sink.clear();
    }
}
