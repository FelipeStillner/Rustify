use crate::*;

pub struct User {
    pub id: i64,
    pub name: String,
    playlists: Vec<Playlist>,
    musics: Vec<Music>,
}

impl User {
    pub fn new(i: i64, n: &str) -> Self {
        User {
            id: i,
            name: n.to_string(),
            playlists: vec![],
            musics: vec![],
        }
    }
    pub fn add_playlist(&mut self, p: Playlist) {
        self.playlists.push(p);
    }
    pub fn get_playlist(&mut self, n: &str) -> Option<&Playlist> {
        self.playlists
            .iter()
            .find(|&x| x.name.as_str() == n)
    }
}
