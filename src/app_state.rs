use crate::*;

#[derive(Clone)]
pub enum AppState {
    QUIT,
    HOME,
    LOGIN,
    EDIT,
    PLAYMUSIC(Music),
    PLAYPLAYLIST(Playlist),
}
