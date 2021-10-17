// shared.rs
//
// Created on 2021/10/16 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::convert::From;
use std::ops::Mul;

use crate::math::vec3::Vec3;
//pub type Array3D = [f32; 3];

pub type Array3D = [f32; 3];
impl From<Vec3> for Array3D {
    fn from(v: Vec3) -> Array3D {
        [v.x, v.y, v.z]
    }
}
