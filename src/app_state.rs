use crate::*;

pub enum AppState {
    QUIT,
    HOME,
    LOGIN,
    PLAYMUSIC(Music),
    PLAYPLAYLIST(Playlist),
}
