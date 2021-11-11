// vec3.rs
//
// Created on 2021/10/12 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use crate::math::ops::{Cross, Dot, Normalize};
use crate::math::point3::Point3;

use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
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

    pub fn inverse(&self) -> Vec3 {
        self.clone() * -1.0
    }

    pub fn cos(&self) -> Vec3 {
        Vec3 {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos(),
        }
    }

    pub fn sin(&self) -> Vec3 {
        Vec3 {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
        }
    }

    pub fn to_radians(&self) -> Vec3 {
        Vec3 {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
            z: self.z.to_radians(),
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.x
    }
}

impl Cross for Vec3 {
    type Output = Self;
    fn cross(&self, v2: &Self) -> Vec3 {
        Vec3 {
            x: self.y * v2.z - self.z * v2.y,
            y: self.z * v2.x - self.x * v2.z,
            z: self.x * v2.y - self.y * v2.x,
        }
    }
}

impl Dot for Vec3 {
    type Output = Self;
    fn dot(&self, v: &Self) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl Dot<&Point3> for Vec3 {
    type Output = Self;
    fn dot(&self, p: &&Point3) -> f32 {
        self.x * p.x + self.y * p.y + self.z * p.z
    }
}

impl Normalize for Vec3 {
    fn normalize(&self) -> Vec3 {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        return *self / magnitude;
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

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
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

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.6}, {:.6}, {:.6}", self.x, self.y, self.z,)
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

    #[test]
    fn normalize() {
        let v = Vec3::new(0.0, 0.0, -10.0);
        let result = Vec3::new(0.0, 0.0, -1.0);

        assert_eq!(v.normalize(), result);
    }
}
