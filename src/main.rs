#![allow(unused)]

use crossterm::terminal::Clear;
use crossterm::{ExecutableCommand, QueueableCommand};
use rodio::Sink;
use sqlite::Connection;
use std::io::{self, BufRead, BufReader, Read, Stdout, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, current, sleep, Thread};
use std::time::{Duration, Instant, SystemTime};

mod app;
mod app_state;
mod database;
mod input;
mod interface;
mod music;
mod play;
mod playlist;
mod status;
mod terminal;
mod user;
mod artist;

use app::*;
use app_state::*;
use database::*;
use input::*;
use interface::*;
use music::*;
use play::*;
use playlist::*;
use status::*;
use terminal::*;
use user::*;
use artist::*;

fn main() {
    App::new().run();
}
