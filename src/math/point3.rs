// point.rs
//
// Created on 2021/10/17 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

//use crate::math::ops::{Cross, Dot};
use crate::math::vec3::Vec3;
//use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use std::ops::{Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x: x, y: y, z: z }
    }

    pub fn inverse(point: Point3) -> Point3 {
        point.clone() * -1.0
    }

    pub fn as_ptr(self) -> *const f32 {
        &self.x
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

// implement Sub
impl Sub for Point3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for &Point3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
