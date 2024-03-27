use crate::*;

pub struct Interface {
    stdout: std::io::Stdout,
    instant: Instant,
    time: Duration,
    pub music_time: Duration,
    pub state: String,
    pub music: String,
    pub music_duration: i64,
    pub playlist: String,
    pub user_name: String,
    pub paused: bool,
}

impl Interface {
    pub fn new() -> Self {
        let mut stdout = std::io::stdout();
        stdout
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::All,
            ))
            .unwrap()
            .execute(crossterm::cursor::MoveTo(0, 4))
            .unwrap()
            .execute(crossterm::terminal::SetSize(50, 5));
        Self {
            stdout: stdout,
            instant: Instant::now(),
            time: Duration::new(0, 0),
            state: "".to_string(),
            music: "".to_string(),
            playlist: "".to_string(),
            user_name: "".to_string(),
            music_duration: 0,
            paused: false,
            music_time: Duration::new(0, 0),
        }
    }
    pub fn clear(&mut self) {
        self.stdout
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::All,
            ))
            .unwrap()
            .execute(crossterm::cursor::MoveTo(0, 4));
    }
    pub fn write_line(&mut self, s: &str, line: u16) {
        self.stdout
            .execute(crossterm::cursor::SavePosition)
            .unwrap()
            .execute(crossterm::cursor::MoveTo(0, line))
            .unwrap()
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine,
            ))
            .unwrap()
            .execute(crossterm::style::Print(s))
            .unwrap()
            .execute(crossterm::cursor::RestorePosition);
    }
    pub fn update(&mut self) {
        let d = self.instant.elapsed();
        if d > self.time + Duration::from_millis(1) {
            self.time = d;
            match self.state.as_str() {
                "login" => {
                    let s = format!(">> LogIn - input:");
                    self.write_line(&s, 0);
                }
                "home" => {
                    let s = format!(">> Home ({}) - input:", self.user_name);
                    self.write_line(&s, 0);
                }
                "play_music" => {
                    let s = format!(">> Playing Music: {}", self.music);
                    self.write_line(&s, 0);
                    let static_line =
                        format!("Music: {}, Duration: {}", self.music, self.music_duration);
                    self.write_line(&static_line, 1);
                    let status_line = format!(
                        "time: {}:{}, pause: {}",
                        self.music_time.as_secs() / 60,
                        self.music_time.as_secs() % 60,
                        self.paused
                    );
                    self.write_line(&status_line, 2);
                }
                "play_playlist" => {
                    let s = format!(">> Playing Playlist: {}", self.playlist);
                    self.write_line(&s, 0);
                    let static_line =
                        format!("Music: {}, Duration: {}", self.music, self.music_duration);
                    self.write_line(&static_line, 1);
                    let status_line = format!(
                        "time: {}:{}, pause: {}",
                        self.music_time.as_secs() / 60,
                        self.music_time.as_secs() % 60,
                        self.paused
                    );
                    self.write_line(&status_line, 2);
                }
                "quit" => {
                    let s = format!(">> Quit");
                    self.write_line(&s, 0);
                }
                err => {
                    panic!("{err}");
                }
            }
        }
    }
    pub fn error(&mut self, s: &str) {
        self.write_line(s, 3);
    }
}
