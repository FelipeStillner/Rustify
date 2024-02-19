mod music;
mod playlist;
mod app;
mod app_state;
mod database;
mod user;

use music::*;
use playlist::*;
use app::*;
use app_state::*;
use database::*;
use user::*;

fn main() {
    App::new().run();
}

