use crate::*;

pub enum AppState {
    QUIT,
    HOME,
    LOGIN,
    EDIT,
    PLAYMUSIC(Music),
    PLAYPLAYLIST(Playlist),
}
