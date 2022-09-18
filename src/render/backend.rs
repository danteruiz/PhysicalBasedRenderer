// backend.rs
//
// Created on 2021/12/03 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use gl::types::GLenum;

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
        Backend::sync_buffer(buffer, resource::Type::IndexBuffer);
    }

    pub fn set_vertex_buffer(buffer: &mut buffer::Buffer) {
        Backend::sync_buffer(buffer, resource::Type::ArrayBuffer);
    }

    pub fn set_uniform_buffer(buffer: &mut buffer::Buffer) {
        Backend::sync_buffer(buffer, resource::Type::UniformBuffer);
    }

    fn sync_buffer(buffer: &mut buffer::Buffer, resource_type: resource::Type) {
        unsafe {
            let gpu_resource = &mut buffer.gpu_resource;

            if gpu_resource.resource_type == resource::Type::Invalid {
                gl::GenBuffers(1, &mut gpu_resource.handle);
                gpu_resource.resource_type = resource_type;
            }

            let resource_type = GLenum::from(gpu_resource.resource_type.clone());
            gl::BindBuffer(resource_type.into(), gpu_resource.handle);
            if buffer.dirty {
                let data_size = (buffer.data.len() * std::mem::size_of::<u8>()) as isize;
                let data = buffer.data.as_ptr().cast();

                println!("data size: {}", data_size);
                gl::BufferData(resource_type.into(), data_size, data, gl::DYNAMIC_DRAW);

                buffer.dirty = false;
            }
        }
    }
}
