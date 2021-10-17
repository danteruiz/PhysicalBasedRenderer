// mat3.rs
//
// Created on 2021/10/17 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

//use std::convert::From;
use crate::math::vec3::Vec3;
use std::ops::{Index, IndexMut, Mul};
#[derive(Debug)]
pub struct Mat3 {
    pub n: [Vec3; 3],
}

// impl Mul for [f32; 3] {
//     type Output = Self;
//     fn mul(&self, scalar: f32) -> Self {
//         [self[0] * scalar, self[1] * scalar, self[2] * scalar]
//     }
// }
impl Mat3 {
    pub fn new() -> Mat3 {
        Mat3 {
            n: [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ],
        }
    }

    // pub fn new(mat: Mat3) -> Mat3 {
    //     Mat3 {
    //         n: [mat[0].clone(), mat[1].clone(), mat[2].clone()],
    //     }
    // }
    //
    // pub fn new(x: [f32; 3], y: [f32; 3], z: [f32; 3]) -> Mat3 {
    //     Mat3 { n: [x, y, z] }
    // }
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
    type Output = Self;
    fn mul(self, scalar: Vec3) -> Self {
        Mat3 {
            n: [self[0] * scalar, self[1] * scalar, self[2] * scalar],
        }
    }
}

impl Mul for Mat3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Mat3::new();

        result[0][0] =
            self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2];
        result[0][1] =
            self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2];
        result[0][2] =
            self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2];

        result[1][0] =
            self[1][0] * other[0][0] + self[1][0] * other[1][1] + self[0][2] * other[1][2];
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
