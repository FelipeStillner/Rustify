use crate::*;

pub struct User {
    pub id: i64,
    pub name: String,
    playlists: Vec<Playlist>,
    musics: Vec<Music>,
}

impl User {
    pub fn new(id: i64, name: &str) -> Self {
        User {
            id: id,
            name: name.to_string(),
            playlists: vec![],
            musics: vec![],
        }
    }
    pub fn add_playlist(&mut self, playlist: Playlist) {
        self.playlists.push(playlist);
    }
    pub fn get_playlist(&mut self, playlist_name: &str) -> Option<&Playlist> {
        self.playlists
            .iter()
            .find(|&x| x.name.as_str() == playlist_name)
    }
}
