use std::cmp::PartialEq;
use std::cmp::{Add, Mul, Sub};

#[derive(Debug)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
