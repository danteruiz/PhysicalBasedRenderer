// vec4.rs
//
// Created on 2021/10/12 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

use crate::math::vec3::Vec3;
// Vec4
#[derive(Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn inverse(self) -> Vec4 {
        self * -1.0
    }

    pub fn to_vec3(v: Vec4) -> Vec3 {
        Vec3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    pub fn from_vec3(v: Vec3) -> Vec4 {
        Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
            w: scalar * self.w,
        }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, vec: Vec4) -> Vec4 {
        vec * self
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}
