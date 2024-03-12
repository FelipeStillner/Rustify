#![allow(unused)]

mod app;
mod app_state;
mod database;
mod music;
mod playlist;
mod user;
mod play;
mod input;
mod status;

use app::*;
use app_state::*;
use database::*;
use music::*;
use playlist::*;
use user::*;
use play::*;
use input::*;
use status::*;

fn main() {
    App::new().run();
}
