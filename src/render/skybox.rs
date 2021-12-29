// skybox.rs
//
// Created on 2021/12/26 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/
use super::texture;
use crate::math;

fn get_capture_views() -> Vec<math::Mat4> {
    vec![
        math::shared::look_at(
            &math::Point3::new(0.0, 0.0, 0.0),
            &math::Point3::new(1.0, 0.0, 0.0),
            &math::Vec3::new(0.0, -1.0, 0.0),
        ),
        math::shared::look_at(
            &math::Point3::new(0.0, 0.0, 0.0),
            &math::Point3::new(-1.0, 0.0, 0.0),
            &math::Vec3::new(0.0, -1.0, 0.0),
        ),
        math::shared::look_at(
            &math::Point3::new(0.0, 0.0, 0.0),
            &math::Point3::new(0.0, 1.0, 0.0),
            &math::Vec3::new(0.0, 0.0, 1.0),
        ),
        math::shared::look_at(
            &math::Point3::new(0.0, 0.0, 0.0),
            &math::Point3::new(0.0, -1.0, 0.0),
            &math::Vec3::new(0.0, 0.0, -1.0),
        ),
        math::shared::look_at(
            &math::Point3::new(0.0, 0.0, 0.0),
            &math::Point3::new(0.0, 0.0, 1.0),
            &math::Vec3::new(0.0, -1.0, 0.0),
        ),
        math::shared::look_at(
            &math::Point3::new(0.0, 0.0, 0.0),
            &math::Point3::new(0.0, 0.0, -1.0),
            &math::Vec3::new(0.0, -1.0, 0.0),
        ),
    ]
}

pub struct Skybox {
    pub skybox: texture::Texture,
    pub irradiance: texture::Texture,
    pub prefilter: texture::Texture,
    pub brdf: texture::Texture,
}

// impl Skybox {
//     fn new(image_path: &'static str) {
//         let hdr_texture = texture::Texture::new(image_path);
//     }
// }

// fn hdr_cube_map(hdr_texture: &texture::Texture) -> texture::Texture {
//     unsafe {
//         gl::Enable(gl::DEPTH_TEST);
//         gl::DepthFunc(gl::LEQUAL);
//         gl::Enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
//
//         gl::GenRenderbuffers(1, &mut capture_rbo);
//         gl::GenFramebuffers(1, &mut capture_fbo);
//
//         let angle: f32 = 90.0;
//         let capture_projection: math::Mat4 =
//             math::shared::perspective(angle.to_radians(), 1.0, 0.1, 10.0);
//     }
// }
