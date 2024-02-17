use crate::*;

#[derive(Debug)]
pub struct Playlist {
    musics: Vec<Music>
}

impl Playlist {
    pub fn new() -> Self {
        Playlist { musics: (vec![]) }
    }
    pub fn add_music(&mut self, music: Music) {
        self.musics.push(music);
    }
    pub fn remove_music(&mut self, id: i64){   
        self.musics.retain(|x| x.id != id);
    }
    pub fn play(&self) {
        for music in &self.musics {
            music.play();
        }
    }
}