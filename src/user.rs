use crate::*;

pub struct User {
    pub id: i64,
    pub name: String,
    playlists: Vec<Playlist>,
    musics: Vec<Music>
}

impl User {
    pub fn new(id: i64, name: &str) -> Self {
        User { id: id, name: name.to_string(), playlists: vec![], musics: vec![] }
    }
    pub fn add_music(&mut self, music: Music) {
        self.musics.push(music);
    }
    pub fn add_playlist(&mut self, playlist: Playlist) {
        self.playlists.push(playlist);
    }
    pub fn get_playlist(&mut self, playlist_name: &str) -> Option<&Playlist> {
        self.playlists.iter().find(|&x| x.name.as_str() == playlist_name)
    }
    pub fn get_music(&mut self, music_name: &str) -> Option<&Music> {
        self.musics.iter().find(|&x| x.name.as_str() == music_name)
    }
}