// mat4.rs
//
// Created on 2021/10/17 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use crate::math::ops::{Determinant, Inverse, Transpose};
use crate::math::vec4::Vec4;
use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Div, Index, IndexMut, Mul};

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

    pub fn zero() -> Mat4 {
        Mat4 {
            n: [Vec4::zero(), Vec4::zero(), Vec4::zero(), Vec4::zero()],
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self[0].x
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

impl Div<f32> for Mat4 {
    type Output = Self;
    fn div(self, scalar: f32) -> Mat4 {
        Mat4 {
            n: [
                self[0] / scalar,
                self[1] / scalar,
                self[2] / scalar,
                self[3] / scalar,
            ],
        }
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

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self[0];
        let y = self[1];
        let z = self[2];
        let w = self[3];

        write!(f, "x: {}, y: {}, z: {}, w: {}", x, y, z, w)
    }
}

impl Transpose for Mat4 {
    fn transpose(&self) -> Mat4 {
        Mat4 {
            n: [
                Vec4::new(self[0][0], self[1][0], self[2][0], self[3][0]),
                Vec4::new(self[0][1], self[1][1], self[2][1], self[3][1]),
                Vec4::new(self[0][2], self[1][2], self[2][2], self[3][2]),
                Vec4::new(self[0][3], self[1][3], self[2][3], self[3][3]),
            ],
        }
    }
}

impl Determinant for Mat4 {
    fn determinant(&self) -> f32 {
        let cofactor00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let cofactor01 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let cofactor02 = self[1][2] * self[2][3] - self[2][2] * self[1][3];
        let cofactor03 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let cofactor04 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let cofactor05 = self[1][1] * self[2][3] - self[2][1] * self[1][3];
        let cofactor06 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let cofactor07 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let cofactor08 = self[1][1] * self[2][2] - self[2][1] * self[1][2];

        let m00 = self[1][1] * cofactor00 - self[2][1] * cofactor01 + self[3][1] * cofactor02;
        let m01 = self[1][0] * cofactor00 - self[2][0] * cofactor01 + self[3][0] * cofactor02;
        let m02 = self[1][0] * cofactor03 - self[2][0] * cofactor04 + self[3][0] * cofactor05;
        let m03 = self[1][0] * cofactor06 - self[2][0] * cofactor07 + self[3][0] * cofactor08;

        let determinant =
            self[0][0] * m00 + -self[0][1] * m01 + self[0][2] * m02 + -self[0][3] * m03;

        determinant
    }
}

impl Inverse for Mat4 {
    fn inverse(&self) -> Mat4 {
        let mut m = Mat4::zero();

        let cofactor00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let cofactor01 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let cofactor02 = self[1][2] * self[2][3] - self[2][2] * self[1][3];
        let cofactor03 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let cofactor04 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let cofactor05 = self[1][1] * self[2][3] - self[2][1] * self[1][3];
        let cofactor06 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let cofactor07 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let cofactor08 = self[1][1] * self[2][2] - self[2][1] * self[1][2];
        let cofactor09 = self[0][2] * self[3][3] - self[3][2] * self[0][3];
        let cofactor10 = self[0][2] * self[2][3] - self[2][2] * self[0][3];
        let cofactor11 = self[0][1] * self[3][3] - self[3][1] * self[0][3];
        let cofactor12 = self[0][1] * self[2][3] - self[2][1] * self[0][3];
        let cofactor13 = self[0][1] * self[3][2] - self[3][1] * self[0][2];
        let cofactor14 = self[0][1] * self[2][2] - self[2][1] * self[0][2];
        let cofactor15 = self[0][2] * self[1][3] - self[1][2] * self[0][3];
        let cofactor16 = self[0][1] * self[1][3] - self[1][1] * self[0][3];
        let cofactor17 = self[0][1] * self[1][2] - self[1][1] * self[0][2];

        m[0][0] = self[1][1] * cofactor00 - self[2][1] * cofactor01 + self[3][1] * cofactor02;
        m[0][1] = -(self[1][0] * cofactor00 - self[2][0] * cofactor01 + self[3][0] * cofactor02);
        m[0][2] = self[1][0] * cofactor03 - self[2][0] * cofactor04 + self[3][0] * cofactor05;
        m[0][3] = -(self[1][0] * cofactor06 - self[2][0] * cofactor07 + self[3][0] * cofactor08);

        m[1][0] = -(self[0][1] * cofactor00 - self[2][1] * cofactor09 + self[3][1] * cofactor10);
        m[1][1] = self[0][0] * cofactor00 - self[2][0] * cofactor09 + self[3][0] * cofactor10;
        m[1][2] = -(self[0][0] * cofactor03 - self[2][0] * cofactor11 + self[3][0] * cofactor12);
        m[1][3] = self[0][0] * cofactor06 - self[2][0] * cofactor13 + self[3][0] * cofactor14;

        m[2][0] = self[0][1] * cofactor01 - self[1][1] * cofactor09 + self[3][1] * cofactor15;
        m[2][1] = -(self[0][0] * cofactor01 - self[1][0] * cofactor09 + self[3][0] * cofactor15);
        m[2][2] = self[0][0] * cofactor04 - self[1][0] * cofactor11 + self[3][0] * cofactor16;
        m[2][3] = -(self[0][0] * cofactor07 - self[1][0] * cofactor13 + self[3][0] * cofactor17);

        m[3][0] = -(self[0][1] * cofactor02 - self[1][1] * cofactor10 + self[2][1] * cofactor15);
        m[3][1] = self[0][0] * cofactor02 - self[1][0] * cofactor10 + self[2][0] * cofactor15;
        m[3][2] = -(self[0][0] * cofactor05 - self[1][0] * cofactor12 + self[2][0] * cofactor16);
        m[3][3] = self[0][0] * cofactor08 - self[1][0] * cofactor14 + self[2][0] * cofactor17;

        m = m.transpose();

        let determinant = m.determinant();
        let one_over_determinate = 1.0 / determinant;
        m * one_over_determinate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn div() {
        let b = Mat4::new(
            Vec4::new(10.0, 10.0, 10.0, 10.0),
            Vec4::new(10.0, 10.0, 10.0, 10.0),
            Vec4::new(10.0, 10.0, 10.0, 10.0),
            Vec4::new(10.0, 10.0, 10.0, 10.0),
        );

        let result = Mat4::new(
            Vec4::new(5.0, 5.0, 5.0, 5.0),
            Vec4::new(5.0, 5.0, 5.0, 5.0),
            Vec4::new(5.0, 5.0, 5.0, 5.0),
            Vec4::new(5.0, 5.0, 5.0, 5.0),
        );

        assert_eq!((b / 2.0), result);
    }
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

    #[test]
    fn determinant() {
        let b = Mat4::new(
            Vec4::new(7.0, 1.0, 9.0, 5.0),
            Vec4::new(5.0, 8.0, 4.0, 3.0),
            Vec4::new(8.0, 2.0, 3.0, 7.0),
            Vec4::new(0.0, 6.0, 8.0, 9.0),
        );

        assert_eq!(b.determinant(), -4071.0);
    }

    #[test]
    fn inverse() {
        let b = Mat4::new(
            Vec4::new(7.0, 1.0, 9.0, 5.0),
            Vec4::new(5.0, 8.0, 4.0, 3.0),
            Vec4::new(8.0, 2.0, 3.0, 7.0),
            Vec4::new(0.0, 6.0, 8.0, 9.0),
        );

        assert_eq!(b.inverse() * b, Mat4::identity())
    }
}
