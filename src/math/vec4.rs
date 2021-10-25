// vec4.rs
//
// Created on 2021/10/12 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::math::point3::Point3;
use crate::math::vec3::Vec3;
// Vec4
#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn zero() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    pub fn inverse(self) -> Vec4 {
        self.clone() * -1.0
    }
}

impl From<Vec3> for Vec4 {
    fn from(v: Vec3) -> Vec4 {
        Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }
}

impl From<Point3> for Vec4 {
    fn from(p: Point3) -> Vec4 {
        Vec4 {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
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

impl Mul for Vec4 {
    type Output = Self;
    fn mul(self, v: Self) -> Self {
        Self {
            x: v.x * self.x,
            y: v.y * self.y,
            z: v.z * self.z,
            w: v.w * self.w,
        }
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

impl Index<usize> for Vec4 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Vec4 index out of bound: {}", index),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Vec4 index out of bound: {}", index),
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn dot() {
    //     let v1 = Vec3::new(3.0, -2.0, 7.0);
    //     let v2 = Vec3::new(0.0, 4.0, -1.0);
    //
    //     let result = -15.0;
    //     assert_eq!(Vec3::dot(&v1, &v2), result);
    // }
    //
    // #[test]
    // fn cross() {
    //     let v1 = Vec3::new(1.0, 3.0, 4.0);
    //     let v2 = Vec3::new(2.0, -5.0, 8.0);
    //
    //     let result = Vec3::new(44.0, 0.0, -11.0);
    //     assert_eq!(Vec3::cross(&v1, &v2), result);
    // }

    #[test]
    fn add() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 2.0);
        let v2 = Vec4::new(4.0, 5.0, 6.0, 3.0);

        let result = Vec4::new(5.0, 7.0, 9.0, 5.0);

        assert_eq!(v1 + v2, result);
    }

    #[test]
    fn sub() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(4.0, 5.0, 6.0, 7.0);

        let result = Vec4::new(-3.0, -3.0, -3.0, -3.0);
        assert_eq!(v1 - v2, result);
    }

    #[test]
    fn add_sub() {
        let v1 = Vec4::new(4.0, 5.0, 6.0, 10.0);
        let v2 = Vec4::new(7.0, -3.0, 0.0, -5.0);
        let v3 = Vec4::new(1.0, 2.0, 3.0, 2.0);

        let result = Vec4::new(10.0, 0.0, 3.0, 3.0);

        assert_eq!(v1 + v2 - v3, result);
    }
}
