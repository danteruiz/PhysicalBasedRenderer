// vec3.rs
//
// Created on 2021/10/12 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::cmp::PartialEq;
//use std::convert::From;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

// Vec3
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn inverse(self) -> Vec3 {
        self * -1.0
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, v: Self) -> Self {
        Self {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of bound: {}", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bound: {}", index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        let v1 = Vec3::new(3.0, -2.0, 7.0);
        let v2 = Vec3::new(0.0, 4.0, -1.0);

        let result = -15.0;
        assert_eq!(Vec3::dot(&v1, &v2), result);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 3.0, 4.0);
        let v2 = Vec3::new(2.0, -5.0, 8.0);

        let result = Vec3::new(44.0, 0.0, -11.0);
        assert_eq!(Vec3::cross(&v1, &v2), result);
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let result = Vec3::new(5.0, 7.0, 9.0);

        assert_eq!(v1 + v2, result);
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let result = Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(v1 - v2, result);
    }

    #[test]
    fn add_sub() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(7.0, -3.0, 0.0);
        let v3 = Vec3::new(1.0, 2.0, 3.0);

        let result = Vec3::new(10.0, 0.0, 3.0);
        assert_eq!(v1 + v2 - v3, result);
    }
}
