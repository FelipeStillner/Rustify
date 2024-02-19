use sqlite::Connection;

use crate::*;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Self {
        let connection = sqlite::open("database.db").unwrap();
        Database { connection: connection}
    }
    pub fn find_music(&self, music_name: &str) -> Option<Music> {
        let query = "SELECT * FROM musics WHERE name = :name".to_string();
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((":name", music_name)).unwrap();

        if let Ok(sqlite::State::Row) = statement.next() {
            let id = statement.read::<i64, _>("id").unwrap();
            let name = statement.read::<String, _>("name").unwrap();
            let filename = statement.read::<String, _>("filename").unwrap();
            let duration = statement.read::<i64, _>("duration").unwrap();
            Some(Music::new(id, name.as_str(), filename.as_str(), duration))
        }
        else {
            None
        }
    }
    pub fn find_music_id(&self, music_id: i64) -> Option<Music> {
        let query = "SELECT * FROM musics WHERE id = :id".to_string();
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((":id", music_id)).unwrap();

        if let Ok(sqlite::State::Row) = statement.next() {
            let id = statement.read::<i64, _>("id").unwrap();
            let name = statement.read::<String, _>("name").unwrap();
            let filename = statement.read::<String, _>("filename").unwrap();
            let duration = statement.read::<i64, _>("duration").unwrap();
            Some(Music::new(id, name.as_str(), filename.as_str(), duration))
        }
        else {
            None
        }
    }
    pub fn find_user(&self, user_name: &str) -> Option<User> {
        let query = "SELECT * FROM users WHERE name = :name".to_string();
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((":name", user_name)).unwrap();
        
        if let Ok(sqlite::State::Row) = statement.next() {
            let id = statement.read::<i64, _>("id").unwrap();
            let name = statement.read::<String, _>("name").unwrap();
            Some(User::new(id, name.as_str()))
        }
        else {
            None
        }
    }
    pub fn find_playlist(&mut self, name: &str, user_name: &str) -> Option<Playlist> {
        let mut playlist: Playlist;
        match self.find_user(user_name) {
            Some(user) => {
                let query = "SELECT * FROM playlists WHERE name = :name".to_string();
                let mut statement = self.connection.prepare(query).unwrap();
                statement.bind((":name", name)).unwrap();          
                if let Ok(sqlite::State::Row) = statement.next() {
                    let id = statement.read::<i64, _>("id").unwrap();
                    let name = statement.read::<String, _>("name").unwrap();
                    let playlist_user_id = statement.read::<i64, _>("user_id").unwrap();
                    if playlist_user_id == 0 || playlist_user_id == user.id {
                        playlist = Playlist::new(id, name.as_str(), playlist_user_id);
                        self.load_playlist(&mut playlist);
                        return Some(playlist);
                    }
                    None
                }
                else {
                    None
                }
            },
            None => {
                None
            }
        }
    }
    pub fn load_playlist(&self, playlist: &mut Playlist) {
        let query = "SELECT * FROM music_playlist WHERE playlist_id = :id".to_string();
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((":id", playlist.id)).unwrap();
        
        while let Ok(sqlite::State::Row) = statement.next() {
            let music_id = statement.read::<i64, _>("music_id").unwrap();
            playlist.add_music(self.find_music_id(music_id).unwrap())
        }
    }
}
