// shared.rs
//
// Created on 2021/10/16 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use crate::math::mat4::Mat4;
use crate::math::ops::{Cross, Dot, Inverse, Normalize};
use crate::math::point3::Point3;
use crate::math::vec3::Vec3;
use crate::math::vec4::Vec4;

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
//
// pub const UNIT_Z: Vec3 = Vec3 {
//     x: 0.0,
//     y: 0.0,
//     z: 1.0,
// };
//
// pub const UNIT_X_NEG: Vec3 = Vec3 {
//     x: -1.0,
//     y: 0.0,
//     z: 0.0,
// };
//
// pub const UNIT_Y_NEG: Vec3 = Vec3 {
//     x: 0.0,
//     y: -1.0,
//     z: 0.0,
// };
//
// pub const UNIT_Z_NEG: Vec3 = Vec3 {
//     x: 0.0,
//     y: 0.0,
//     z: -1.0,
// };

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
    let forward = (target - eye).normalize();
    let right = Vec3::cross(&forward, &target_up).normalize();
    let up = Vec3::cross(&right, &forward);

    let mut result = Mat4::identity();

    let r = Mat4::new(
        Vec4::from(right),
        Vec4::from(up),
        Vec4::from(forward * -1.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );

    let mut t = Mat4::identity();
    t[3] = Vec4::from(eye);
    let cam_to_world_mat = r * t;
    cam_to_world_mat.inverse()
}
