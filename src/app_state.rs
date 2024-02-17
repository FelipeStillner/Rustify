use crate::{Music, Playlist};

pub enum AppState {
    QUIT,
    HOME,
    PLAYMUSIC(Music),
    PLAYPLAYLIST(Playlist),
}