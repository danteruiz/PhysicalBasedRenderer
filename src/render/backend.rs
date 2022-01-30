// backend.rs
//
// Created on 2021/12/03 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use super::*;
//use crate::math;

// struct CameraInfo {
//     projection: math::Mat4,
//     view: math::Mat4,
// }

pub struct Backend {
    vao_buffer: u32,
    camera_info_buffer: u32,
    //camera_info: CameraInfo,
}

impl Backend {
    // pub fn updateCameraInfo(projection: math::Mat4, view: math::Mat4) {}

    pub fn set_attributes(attributes: &stream::Attributes) {
        for attribute in attributes {
            let format = &attribute.format;

            let slot = attribute.slot as u32;
            let format_dimension = format.dimension_size() as i32;
            let stride = format.stride() as i32;
            let offset = attribute.get_total_offset() as *const _;

            unsafe {
                gl::VertexAttribPointer(slot, format_dimension, gl::FLOAT, 0, stride, offset);
                gl::EnableVertexAttribArray(slot);
            }
        }
    }

    pub fn set_index_buffer(buffer: &mut buffer::Buffer) {
        Backend::sync_buffer(buffer, gl::ELEMENT_ARRAY_BUFFER);
    }

    pub fn set_vertex_buffer(buffer: &mut buffer::Buffer) {
        Backend::sync_buffer(buffer, gl::ARRAY_BUFFER);
    }

    fn sync_buffer(buffer: &mut buffer::Buffer, resource_type: u32) {
        unsafe {
            let gpu_resource = &mut buffer.gpu_resource;

            if gpu_resource.handle == 0 {
                gl::GenBuffers(1, &mut gpu_resource.handle);
                gpu_resource.resource_type = resource_type;
            }

            let resource_type = gpu_resource.resource_type;
            gl::BindBuffer(resource_type, gpu_resource.handle);
            if buffer.dirty {
                let data_size = (buffer.data.len() * std::mem::size_of::<u8>()) as isize;
                let data = buffer.data.as_ptr().cast();
                gl::BufferData(resource_type, data_size, data, gl::DYNAMIC_DRAW);
            }
        }
    }
}
