// mat4.rs
//
// Created on 2021/10/17 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use crate::math::vec4::Vec4;
use std::cmp::PartialEq;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
    pub n: [Vec4; 4],
}
impl Mat4 {
    pub fn new(v1: Vec4, v2: Vec4, v3: Vec4, v4: Vec4) -> Mat4 {
        Mat4 {
            n: [v1, v2, v3, v4],
        }
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            n: [
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 1.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;
    fn index(&self, index: usize) -> &Vec4 {
        &self.n[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Vec4 {
        &mut self.n[index]
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Mat4 {
            n: [
                self[0] * scalar,
                self[1] * scalar,
                self[2] * scalar,
                self[3] * scalar,
            ],
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, v: Vec4) -> Vec4 {
        Vec4 {
            x: v.x * self[0][0] + v.y * self[1][0] + v.z * self[2][0] + v.w * self[3][0],
            y: v.x * self[0][1] + v.y * self[1][1] + v.z * self[2][1] + v.w * self[3][1],
            z: v.x * self[0][2] + v.y * self[1][2] + v.z * self[2][2] + v.w * self[3][2],
            w: v.x * self[0][3] + v.y * self[1][3] + v.z * self[2][3] + v.w * self[3][3],
        }
    }
}

impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Mat4::identity();

        result[0][0] = self[0][0] * other[0][0]
            + self[1][0] * other[0][1]
            + self[2][0] * other[0][2]
            + self[3][0] * other[0][3];
        result[0][1] = self[0][1] * other[0][0]
            + self[1][1] * other[0][1]
            + self[2][1] * other[0][2]
            + self[3][1] * other[0][3];
        result[0][2] = self[0][2] * other[0][0]
            + self[1][2] * other[0][1]
            + self[2][2] * other[0][2]
            + self[3][2] * other[0][3];
        result[0][3] = self[0][3] * other[0][0]
            + self[1][3] * other[0][1]
            + self[2][3] * other[0][2]
            + self[3][3] * other[0][3];

        result[1][0] = self[0][0] * other[1][0]
            + self[1][0] * other[1][1]
            + self[2][0] * other[1][2]
            + self[3][0] * other[1][3];
        result[1][1] = self[0][1] * other[1][0]
            + self[1][1] * other[1][1]
            + self[2][1] * other[1][2]
            + self[3][1] * other[1][3];
        result[1][2] = self[0][2] * other[1][0]
            + self[1][2] * other[1][1]
            + self[2][2] * other[1][2]
            + self[3][2] * other[1][3];
        result[1][3] = self[0][3] * other[1][0]
            + self[1][3] * other[1][1]
            + self[2][3] * other[1][2]
            + self[3][3] * other[1][3];

        result[2][0] = self[0][0] * other[2][0]
            + self[1][0] * other[2][1]
            + self[2][0] * other[2][2]
            + self[3][0] * other[2][3];
        result[2][1] = self[0][1] * other[2][0]
            + self[1][1] * other[2][1]
            + self[2][1] * other[2][2]
            + self[3][1] * other[2][3];
        result[2][2] = self[0][2] * other[2][0]
            + self[1][2] * other[2][1]
            + self[2][2] * other[2][2]
            + self[3][2] * other[2][3];
        result[2][3] = self[0][3] * other[2][0]
            + self[1][3] * other[2][1]
            + self[2][3] * other[2][2]
            + self[3][3] * other[2][3];

        result[3][0] = self[0][0] * other[3][0]
            + self[1][0] * other[3][1]
            + self[2][0] * other[3][2]
            + self[3][0] * other[3][3];
        result[3][1] = self[0][1] * other[3][0]
            + self[1][1] * other[3][1]
            + self[2][1] * other[3][2]
            + self[3][1] * other[3][3];
        result[3][2] = self[0][2] * other[3][0]
            + self[1][2] * other[3][1]
            + self[2][2] * other[3][2]
            + self[3][2] * other[3][3];
        result[3][3] = self[0][3] * other[3][0]
            + self[1][3] * other[3][1]
            + self[2][3] * other[3][2]
            + self[3][3] * other[3][3];

        result
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        self[0] == other[0] && self[1] == other[1] && self[2] == other[2] && self[3] == other[3]
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mul() {
        let a = Mat4::new(
            Vec4::new(5.0, 0.0, 3.0, 1.0),
            Vec4::new(2.0, 6.0, 8.0, 8.0),
            Vec4::new(6.0, 2.0, 1.0, 5.0),
            Vec4::new(1.0, 0.0, 4.0, 6.0),
        );

        let b = Mat4::new(
            Vec4::new(7.0, 1.0, 9.0, 5.0),
            Vec4::new(5.0, 8.0, 4.0, 3.0),
            Vec4::new(8.0, 2.0, 3.0, 7.0),
            Vec4::new(0.0, 6.0, 8.0, 9.0),
        );

        let result = Mat4::new(
            Vec4::new(96.0, 24.0, 58.0, 90.0),
            Vec4::new(68.0, 56.0, 95.0, 107.0),
            Vec4::new(69.0, 18.0, 71.0, 81.0),
            Vec4::new(69.0, 52.0, 92.0, 142.0),
        );

        assert_eq!((a * b), result);
    }
}
