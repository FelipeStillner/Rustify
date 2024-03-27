use crate::*;

pub struct App {
    pub state: AppState,
    database: Database,
    pub user: User,
    last_state: AppState,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState::LOGIN,
            database: Database::new(),
            user: User::new(0, "guest"),
            last_state: AppState::QUIT,
        }
    }
    pub fn run(&mut self) {
        let (terminal, receiver) = Terminal::new();
        let thread = thread::spawn(move || terminal.get_input());

        let mut interface = Interface::new();
        interface.state = "login".to_string();

        loop {
            interface.update();
            match &self.state {
                AppState::PLAYMUSIC(music) => {
                    interface.clear();
                    music.play(&receiver, &mut interface);
                    self.state = AppState::HOME;
                    interface.state = "home".to_string();
                    interface.clear();
                }
                AppState::PLAYPLAYLIST(playlist) => {
                    interface.clear();
                    playlist.play(&receiver, &mut interface);
                    self.state = AppState::HOME;
                    interface.state = "home".to_string();
                    interface.clear();
                }
                AppState::QUIT => {
                    break;
                }
                _ => {}
            }
            interface.update();

            if let Ok(input) = receiver.try_recv() {
                interface.clear();
                match &self.state {
                    AppState::LOGIN => {
                        let input = manipulate_login_input(input);
                        match input {
                            LoginInput::GUEST => {
                                self.state = AppState::HOME;
                                interface.state = "home".to_string();
                                interface.user_name = "guest".to_string();
                            }
                            LoginInput::QUIT => {
                                self.state = AppState::QUIT;
                                interface.state = "quit".to_string();
                            }
                            LoginInput::NEW(user) => {
                                let user: User = User::new(0, &user);
                                match self.database.add_user(user) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        interface.error("# Error: Invalid New User ");
                                    }
                                }
                            }
                            LoginInput::OTHER(user_name) => {
                                match self.database.find_user(user_name.as_str()) {
                                    Some(user) => {
                                        self.user = user;
                                        self.state = AppState::HOME;
                                        interface.state = "home".to_string();
                                        interface.user_name = user_name;
                                    }
                                    None => {
                                        interface.error("# Error: Invalid User");
                                    }
                                }
                            }
                        }
                    }
                    AppState::HOME => {
                        println!(">> Home ({}) - input:", self.user.name);
                        let input = manipulate_home_input(input);
                        match input {
                            HomeInput::QUIT => {
                                self.state = AppState::QUIT;
                                interface.state = "quit".to_string();
                            }
                            HomeInput::PLAYPLAYLIST(playlist) => {
                                match self.database.find_playlist(playlist.as_str(), self.user.id) {
                                    Some(p) => {
                                        self.state = AppState::PLAYPLAYLIST(p);
                                        interface.state = "play_playlist".to_string();
                                        interface.playlist = playlist;
                                    }
                                    None => {
                                        interface.error("# Error: Invalid playlist name!");
                                    }
                                };
                            }
                            HomeInput::PLAYMUSIC(music) => {
                                match self.database.find_music(music.as_str()) {
                                    Some(m) => {
                                        self.state = AppState::PLAYMUSIC(m);
                                        interface.state = "play_music".to_string();
                                        interface.playlist = music;
                                    }
                                    None => {
                                        interface.error("# Error: Invalid music name!");
                                    }
                                };
                            }
                            HomeInput::INVALID => {
                                interface.error("# Error: Invalid input!");
                            }
                        }
                    }
                    AppState::EDIT => {
                        todo!()
                    }
                    _ => {}
                }
                interface.write_line("", 4);
            } else {
                //todo
            }
        }
    }
}
