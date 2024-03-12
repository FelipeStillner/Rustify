use std::time::Duration;

use crate::*;

pub struct Status {
    pub time: Duration,
    pub paused: bool
}

impl Status {
    pub fn new() -> Self {
        Self { time: Duration::from_secs(0), paused: false }
    }
    pub fn update(&mut self, dt: Duration, paused: bool) {
        self.paused = paused;
        if !self.paused {
            self.time += dt;
        }
    }
}