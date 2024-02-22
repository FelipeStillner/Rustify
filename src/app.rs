use crate::*;
use std::io;

pub struct App {
    state: AppState,
    database: Database,
    user: User,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState::LOGIN,
            database: Database::new(),
            user: User::new(0, "GUEST"),
        }
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
                    } else if input_list[0].to_uppercase() == "GUEST" {
                        self.state = AppState::HOME
                    } else if input_list[0].to_uppercase() == "NEW"{
                        let user: User = User::new(0, input_list[1].to_uppercase().as_str());
                        match self.database.add_user(user) {
                            Ok(_) => {},
                            Err(_) => {
                                println!("# Error: Invalid New User ");
                            }
                        }
                    } else {
                        match self
                            .database
                            .find_user(input_list[0].to_uppercase().as_str())
                        {
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
                }
                AppState::PLAYPLAYLIST(playlist) => {
                    playlist.play();
                    self.state = AppState::HOME;
                }
                AppState::HOME => {
                    println!(">> Home ({}) - input:", self.user.name);
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input_list: Vec<&str> = input.trim().split(" ").collect();
                    if input_list[0] == "quit" {
                        self.state = AppState::QUIT;
                    } else if input_list[0] == "pm" {
                        // pm <music_name> : play music_name
                        if input_list.len() == 1 {
                            println!("# Error: Include the music name");
                        } else {
                            match self.database.find_music(input_list[1]) {
                                None => {
                                    println!("# Error: Invalid music name!");
                                }
                                Some(music) => {
                                    self.state = AppState::PLAYMUSIC(music);
                                }
                            };
                        }
                    } else if input_list[0] == "pp" {
                        // pp <playlist_name> : play playlist_name
                        if input_list.len() == 1 {
                            println!("# Error: Include the playlist name");
                        } else {
                            match self
                                .database
                                .find_playlist(input_list[1], self.user.id)
                            {
                                None => {
                                    println!("# Error: Invalid playlist name!");
                                }
                                Some(playlist) => {
                                    self.state = AppState::PLAYPLAYLIST(playlist);
                                }
                            };
                        }
                    } else if input_list[0] == "am" {
                        // am <music_name> <music_path> : add music to database
                        if input_list.len() <= 2 {
                            println!("# Error: Include the music name and path");
                        } else {
                            let music = Music::new(0, input_list[1].trim(), input_list[2].trim().clone(), 0);
                            match std::fs::File::open(format!("musics/{}", input_list[2].to_string())) {
                                Ok(_) => {
                                    println!("{}", music.filename);
                                    match self.database.add_music(music) {
                                        Ok(_) => {},
                                        Err(_) => {
                                            println!("# Error: name or path invalid");
                                        }
                                    }
                                },
                                Err(_) => {
                                    println!("# Error: Invalid path");
                                }
                            };
                        } 
                    } else if input_list[0] == "ap" {
                        // ap <playlist_name> : add playlist to database
                        if input_list.len() == 1 {
                            println!("# Error: Include the playlist name");
                        } else {
                            let playlist = Playlist::new(0, input_list[1].trim(), self.user.id);
                            match self.database.add_playlist(playlist) {
                                Ok(_) => {},
                                Err(_) => {
                                    println!("# Error: name invalid");
                                }
                            }
                        } 
                    } else if input_list[0] == "amp" {
                        // amp <music_name> <playlist_name> : add music to the playlist
                        if input_list.len() <= 2 {
                            println!("# Error: Include the playlist name");
                        } else {
                            match self.database.find_playlist(input_list[2].trim(), self.user.id) {
                                None => {
                                    println!("# Error: Invalid playlist name");
                                },
                                Some(playlist) => {
                                    match self.database.find_music(input_list[1].trim()) {
                                        None => {
                                            println!("# Error: Invalid music name");
                                        },
                                        Some(music) => {
                                            self.database.add_music_playlist(music.id, playlist.id);
                                        }
                                    }
                                }
                            }
                        } 
                    } else {
                        println!("# Error: Invalid Input");
                    }
                }
                AppState::QUIT => {
                    println!(">> QUIT");
                    break;
                }
            }
        }
    }
}
