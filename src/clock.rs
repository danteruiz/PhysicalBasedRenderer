// clock.rs
//
// Created on 2021/10/12 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::time::{Duration, Instant};
pub struct Clock {
    last_time: Instant,
}

impl Clock {
    pub fn new() -> Clock {
        return Clock {
            last_time: Instant::now(),
        };
    }

    pub fn get_delta_time(&mut self) -> f32 {
        let current_time = Instant::now();
        let duration: Duration = current_time.duration_since(self.last_time);
        let delta_time: f32 = duration.as_millis() as f32;
        self.last_time = current_time;
        return delta_time / 1000.0;
    }
}
