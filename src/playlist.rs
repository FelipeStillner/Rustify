use crate::*;

#[derive(Debug)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub duration: i64,
    musics: Vec<Music>,
    user_id: i64,
}

impl Playlist {
    pub fn new(id: i64, name: &str, user_id: i64) -> Self {
        Playlist {id: id, name: name.to_string(), duration: 0,  musics: (vec![]), user_id: user_id}
    }
    pub fn add_music(&mut self, music: Music) {
        self.duration += music.duration;
        self.musics.push(music);
    }
    pub fn remove_music(&mut self, id: i64){   
        for music in self.musics.iter() {
            if music.id == id {
                self.duration -= music.duration;
            }
        }
        self.musics.retain(|x| x.id != id);
    }
    pub fn play(&self) {
        for music in &self.musics {
            music.play();
        }
    }
}