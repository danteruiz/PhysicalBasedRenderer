use std::cmp::PartialEq;
use std::convert::From;
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quat {
    fn new(w: f32, x: f32, y: f32, z: f32) -> Quat {
        Quat {
            w: w,
            x: x,
            y: y,
            z: z,
        }
    }
}
