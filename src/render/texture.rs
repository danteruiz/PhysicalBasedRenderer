// texture.rs
//
// Created on 2021/11/29 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/
use gl33::{gl_core_types::*, gl_enumerations::*, global_loader::*};
use image;
use std::fs;

use crate::to_byte_slice;
pub enum Type {
    Tex1D,
    Tex2D,
}

pub struct Texture {
    pub id: u32,
    pub tex_type: Type,
}

impl Texture {
    pub fn new(path: &'static str) -> Texture {
        let bytes = fs::read(path).unwrap();
        let decoder = image::hdr::HDRDecoder::new(bytes.as_slice()).unwrap();
        let info = decoder.metadata();
        let rgb_data = decoder.read_image_hdr().unwrap();

        let mut rgba: Vec<u8> = Vec::with_capacity(rgb_data.len() * (4 * 4));

        for rgb in rgb_data {
            let alpha = 1.0f32;

            rgba.extend_from_slice(&rgb.data[0].to_ne_bytes());
            rgba.extend_from_slice(&rgb.data[1].to_ne_bytes());
            rgba.extend_from_slice(&rgb.data[2].to_ne_bytes());
            rgba.extend_from_slice(&alpha.to_ne_bytes());
        }

        let mut texture_id: u32 = 0;
        unsafe {
            glGenTextures(1, &mut texture_id);
            glBindTexture(GL_TEXTURE_2D, texture_id);

            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);

            let format: GLenum = GL_RGBA;
            let texture_size: GLenum = GL_UNSIGNED_INT;

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA.0 as i32,
                info.width as i32,
                info.height as i32,
                0,
                format,
                texture_size,
                rgba.as_ptr().cast(),
            );
        }

        Texture {
            id: texture_id,
            tex_type: Type::Tex2D,
        }
    }

    // pub fn create_from_gltf(
    //     width: i32,
    //     height: i32,
    //     component: i32,
    //     bits: i32,
    //     data: Vec<u8>,
    // ) -> Texture {
    //     let mut texture_id: u32 = 0;
    //     unsafe {
    //         glGenTextures(1, &mut texture_id);
    //         glBindTexture(GL_TEXTURE_2D, texture_id);
    //
    //         glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    //         glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT.0 as i32);
    //         glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT.0 as i32);
    //         glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR.0 as i32);
    //         glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);
    //
    //         let format: GLenum;
    //
    //         match component {
    //             1 => format = GL_RED,
    //             2 => format = GL_RG,
    //             3 => format = GL_RGB,
    //             _ => format = GL_RGBA,
    //         }
    //
    //         let texture_size: GLenum;
    //
    //         match bits {
    //             8 => texture_size = GL_UNSIGNED_BYTE,
    //             16 => texture_size = GL_UNSIGNED_SHORT,
    //             _ => texture_size = GL_UNSIGNED_INT,
    //         }
    //
    //         glTexImage2D(
    //             GL_TEXTURE_2D,
    //             0,
    //             GL_RGBA.0 as i32,
    //             width,
    //             height,
    //             0,
    //             format,
    //             texture_size,
    //             data.as_ptr().cast(),
    //         );
    //     }
    //
    //     Texture {
    //         id: texture_id,
    //         tex_type: Type::Tex2D,
    //     }
    // }
}
