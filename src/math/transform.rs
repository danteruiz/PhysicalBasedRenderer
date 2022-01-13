// transform.rs
//
// Created on 2022/01/10 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use super::{Mat3, Mat4, Point3, Quat, Vec3, Vec4};

use std::default::Default;

pub struct Transform {
    orientation: Quat,
    scale: Vec3,
    translation: Point3,
}

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        let mut result = Mat4::identity();

        let mut rotation = Mat3::from(self.orientation);

        rotation[0] *= self.scale.x;
        rotation[1] *= self.scale.y;
        rotation[2] *= self.scale.z;

        result[0] = Vec4::from(rotation[0]);
        result[1] = Vec4::from(rotation[1]);
        result[2] = Vec4::from(rotation[2]);

        result[3] = Vec4::from(self.translation);

        result
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            orientation: Quat::identity(),
            scale: Vec3::new(0.0, 0.0, 0.0),
            translation: Point3::new(0.0, 0.0, 0.0),
        }
    }
}
