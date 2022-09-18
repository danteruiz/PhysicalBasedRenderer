// buffer.rs
//
// Created on 2021/12/02 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::ops::Drop;

use super::*;

use gl::types::GLenum;

pub struct Buffer {
    pub data: Vec<u8>,
    pub(crate) gpu_resource: resource::GPUResource,
    pub(crate) dirty: bool,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Buffer {
        Buffer {
            data,
            gpu_resource: resource::GPUResource {
                handle: 0,
                resource_type: resource::Type::Invalid,
            },
            dirty: true,
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            gpu_resource: resource::GPUResource {
                handle: 0,
                resource_type: resource::Type::Invalid,
            },
            dirty: true,
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.gpu_resource.resource_type != resource::Type::Invalid {
            unsafe {
                println!("deleting buffer: {:?}", self.gpu_resource.resource_type);

                let resource_type = GLenum::from(self.gpu_resource.resource_type.clone());
                gl::DeleteBuffers(resource_type as i32, &self.gpu_resource.handle);
            }
        }
    }
}

// pub struct BufferView {
//     buffer: *const Buffer,
//     offset: usize,
//     size: usize,
// }
