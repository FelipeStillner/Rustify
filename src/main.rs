mod music;
mod playlist;
mod app;
mod app_state;
mod database;

use music::*;
use playlist::*;
use app::*;
use app_state::*;
use database::*;

fn main() {
    App::new().run();
}

