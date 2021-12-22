// framebuffer.rs
//
// Created on 2021/12/19 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use gl33::{gl_core_types::*, gl_enumerations::*, global_loader::*};

use std::rc;

use super::texture;
pub struct FrameBuffer {
    id: u32,
    width: u32,
    height: u32,
    texture_buffer: Option<rc::Rc<texture::Texture>>,
}

impl FrameBuffer {
    fn create(width: u32, height: u32) -> FrameBuffer {
        let mut id: u32 = 0;

        unsafe {
            glGenFramebuffers(1, &mut id);
        }

        FrameBuffer {
            id,
            width,
            height,
            texture_buffer: None,
        }
    }

    fn set_texture(&self, texture: rc::Rc<texture::Texture>) {
	if p
    }
}
