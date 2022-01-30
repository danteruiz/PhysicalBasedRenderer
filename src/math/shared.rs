// shared.rs
//
// Created on 2021/10/16 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use crate::math::mat4::Mat4;
use crate::math::ops::{Cross, Dot, Normalize};
use crate::math::point3::Point3;
use crate::math::vec3::Vec3;

// pub const UNIT_X: Vec3 = Vec3 {
//     x: 1.0,
//     y: 0.0,
//     z: 0.0,
// };
//
pub const UNIT_Y: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub const TWO_PI: f32 = 6.28319; //2.0 * std::f32::consts::PI;

pub const UNIT_Z: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

pub const UNIT_X: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let mut result = Mat4::zero();

    let tan_half_fov = (fov / 2.0).tan();

    result[0][0] = 1.0 / (aspect * tan_half_fov);
    result[1][1] = 1.0 / tan_half_fov;
    result[2][2] = -(far + near) / (far - near);
    result[2][3] = -1.0;
    result[3][2] = -(2.0 * far * near) / (far - near);
    result
}

pub fn look_at(eye: &Point3, target: &Point3, target_up: &Vec3) -> Mat4 {
    let f = (target - eye).normalize();
    let s = Vec3::cross(&f, &target_up).normalize();
    let u = Vec3::cross(&s, &f).normalize();

    let mut result = Mat4::identity();

    result[0][0] = s.x;
    result[1][0] = s.y;
    result[2][0] = s.z;
    result[0][1] = u.x;
    result[1][1] = u.y;
    result[2][1] = u.z;
    result[0][2] = -f.x;
    result[1][2] = -f.y;
    result[2][2] = -f.z;
    result[3][0] = -Vec3::dot(&s, &eye);
    result[3][1] = -Vec3::dot(&u, &eye);
    result[3][2] = Vec3::dot(&f, &eye);
    result
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
