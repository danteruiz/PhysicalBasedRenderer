// backend.rs
//
// Created on 2021/12/03 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

struct CameraInfo {
    projection: math::Mat4,
    view: math::Mat4,
}

struct Backend {
    vao_buffer: u32,
    camera_info_buffer: u32,
    camera_info: CameraInfo,
}

impl Backend {
    fn updateCameraInfo(&mut self, projection: math::Mat4, view: math::Mat4) {}
}
