use crate::*;
use std::io;

pub struct App {
    state: AppState,
    database: Database
}

impl App {
    pub fn new() -> Self {
        App {state: AppState::HOME, database: Database::new()}
    }
    pub fn run(&mut self) {
        loop {
            match &self.state {
                AppState::PLAYMUSIC(music) => {
                    music.play();
                    self.state = AppState::HOME;
                },
                AppState::PLAYPLAYLIST(playlist) => {
                    playlist.play();
                    self.state = AppState::HOME;
                },
                AppState::HOME => {
                    println!("INIT");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    input = String:: from(input.trim());
                    println!("{}", input);
                    if input == "quit" {
                        self.state = AppState::QUIT;
                    }
                    else if input == "pm" { // play music
                        self.state = AppState::PLAYMUSIC(self.database.find_music("Beep 1").unwrap());
                    }
                    else if input == "pp" { // play playlist
                        let beep1: Music = Music::new(0, "Beep1", "beep.wav", 1000);
                        let beep2: Music = Music::new(1, "Beep2", "beep2.wav", 500);
                        let mut beeps: Playlist = Playlist::new();
                        beeps.add_music(beep1);
                        beeps.add_music(beep2);
                        self.state = AppState::PLAYPLAYLIST(beeps);
                    }
                    else {
                        println!("Invalid Input");
                    }
                },
                AppState::QUIT => {
                    println!("QUIT");
                    break;
                },
            }
        }
    }
}