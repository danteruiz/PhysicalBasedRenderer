// mat3.rs
//
// Created on 2021/10/17 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

//use std::convert::From;
use crate::math::vec3::Vec3;
use std::cmp::PartialEq;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Mat3 {
    pub n: [Vec3; 3],
}

impl Mat3 {
    // pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Mat3 {
    //     Mat3 { n: [v1, v2, v3] }
    // }
    pub fn identity() -> Mat3 {
        Mat3 {
            n: [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ],
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Vec3 {
        &self.n[index]
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Vec3 {
        &mut self.n[index]
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Mat3 {
            n: [self[0] * scalar, self[1] * scalar, self[2] * scalar],
        }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self[0][0] + v.y * self[1][0] + v.z * self[2][0],
            y: v.x * self[0][1] + v.y * self[1][1] + v.z * self[2][1],
            z: v.x * self[0][2] + v.y * self[1][2] + v.z * self[2][2],
        }
    }
}

impl Mul for Mat3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Mat3::identity();

        result[0][0] =
            self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2];
        result[0][1] =
            self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2];
        result[0][2] =
            self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2];

        result[1][0] =
            self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2];
        result[1][1] =
            self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2];
        result[1][2] =
            self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2];

        result[2][0] =
            self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2];
        result[2][1] =
            self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2];
        result[2][2] =
            self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2];

        result
    }
}

impl PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        self[0] == other[0] && self[1] == other[1] && self[2] == other[2]
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_vec3() {
        let a = Mat3::new(
            Vec3::new(1.0, 0.0, 7.0),
            Vec3::new(-5.0, -2.0, 2.0),
            Vec3::new(3.0, 6.0, -4.0),
        );

        assert_eq!(a[1], Vec3::new(-5.0, -2.0, 2.0));
    }

    #[test]
    fn index_float() {
        let a = Mat3::new(
            Vec3::new(1.0, 0.0, 7.0),
            Vec3::new(-5.0, -2.0, 2.0),
            Vec3::new(3.0, 6.0, -4.0),
        );

        assert_eq!(a[1][1], -2.0);
    }

    #[test]
    fn mul() {
        let a = Mat3::new(
            Vec3::new(1.0, 0.0, 7.0),
            Vec3::new(-5.0, -2.0, 2.0),
            Vec3::new(3.0, 6.0, -4.0),
        );

        let b = Mat3::new(
            Vec3::new(-8.0, 7.0, 2.0),
            Vec3::new(6.0, 0.0, 4.0),
            Vec3::new(1.0, -3.0, 5.0),
        );

        let result = Mat3::new(
            Vec3::new(-37.0, -2.0, -50.0),
            Vec3::new(18.0, 24.0, 26.0),
            Vec3::new(31.0, 36.0, -19.0),
        );

        assert_eq!((a * b), result);
    }
}
