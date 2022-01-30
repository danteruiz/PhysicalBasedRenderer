// buffer.rs
//
// Created on 2021/12/02 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::ops::Drop;

use super::*;

#[derive(Default)]
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
                resource_type: 0,
            },
            dirty: true,
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(
                self.gpu_resource.resource_type as i32,
                &self.gpu_resource.handle,
            );
        }
    }
}

// pub struct BufferView {
//     buffer: *const Buffer,
//     offset: usize,
//     size: usize,
// }
