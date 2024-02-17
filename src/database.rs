use sqlite::Connection;

use crate::*;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Self {
        let connection = sqlite::open("database.db").unwrap();
        Database { connection: connection }
    }
    pub fn find_music(&mut self, music_name: &str) -> Option<Music> {
        let query = "SELECT * FROM musics WHERE name = :name".to_string();
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((":name", music_name)).unwrap();

        let out: Option<Music>;
        
        if let Ok(sqlite::State::Row) = statement.next() {
            let id = statement.read::<i64, _>("id").unwrap();
            let name = statement.read::<String, _>("name").unwrap();
            let filename = statement.read::<String, _>("filename").unwrap();
            let duration = statement.read::<i64, _>("duration").unwrap() as u64;
            out = Some(Music::new(id, name.as_str(), filename.as_str(), duration));
        }
        else {
            out = None;
        }
        out
    }
}
