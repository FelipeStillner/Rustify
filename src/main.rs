#![allow(unused)]

mod app;
mod app_state;
mod database;
mod music;
mod playlist;
mod user;

use app::*;
use app_state::*;
use database::*;
use music::*;
use playlist::*;
use user::*;

fn main() {
    App::new().run();
}
