use crate::*;
use std::io;

pub struct App {
    state: AppState,
    database: Database,
    user: User
}

impl App {
    pub fn new() -> Self {
        App {state: AppState::LOGIN, database: Database::new(), user: User::new(0, "GUEST")}
    }
    pub fn run(&mut self) {
        loop {
            match &self.state {
                AppState::LOGIN => {
                    println!(">> LogIn - input:");
                    let mut input = String::new(); 
                    io::stdin().read_line(&mut input).unwrap();
                    let input_list: Vec<&str> = input.trim().split(" ").collect();
                    if input_list[0].to_uppercase() == "QUIT" {
                        self.state = AppState::QUIT;
                    }
                    else if input_list[0].to_uppercase() == "GUEST" {
                        self.state = AppState::HOME
                    }
                    else {
                        match self.database.find_user(input_list[0].to_uppercase().as_str()) {
                            Some(user) => {
                                self.user = user;
                                self.state = AppState::HOME;
                            }
                            None => {
                                println!("# Error: Invalid User");
                            }
                        }
                    }
                }
                AppState::PLAYMUSIC(music) => {
                    music.play();
                    self.state = AppState::HOME;
                },
                AppState::PLAYPLAYLIST(playlist) => {
                    playlist.play();
                    self.state = AppState::HOME;
                },
                AppState::HOME => {
                    println!(">> Home ({}) - input:", self.user.name);
                    let mut input = String::new(); 
                    io::stdin().read_line(&mut input).unwrap();
                    let input_list: Vec<&str> = input.trim().split(" ").collect();
                    if input_list[0] == "quit" {
                        self.state = AppState::QUIT;
                    }
                    else if input_list[0] == "pm" { //pm <music_name> : play music_name 
                        if input_list.len() == 1{
                            println!("# Error: Include the music name");
                        }
                        else {
                            match self.database.find_music(input_list[1]) {
                                None => {
                                    println!("# Error: Invalid music name!");
                                },
                                Some(music) => {
                                    self.state = AppState::PLAYMUSIC(music);
                                },
                            };
                        }
                    }
                    else if input_list[0] == "pp" { //play playlist
                        if input_list.len() == 1{
                            println!("# Error: Include the playlist name");
                        }
                        else {
                            match self.database.find_playlist(input_list[1], self.user.name.as_str()) {
                                None => {
                                    println!("# Error: Invalid playlist name!");
                                },
                                Some(playlist) => {
                                    self.state = AppState::PLAYPLAYLIST(playlist);
                                },
                            };
                        }
                    }
                    else {
                        println!("# Error: Invalid Input");
                    }
                },
                AppState::QUIT => {
                    println!(">> QUIT");
                    break;
                },
            }
        }
    }
}