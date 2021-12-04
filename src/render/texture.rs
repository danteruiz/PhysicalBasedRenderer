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

        let mut rgba: Vec<u8> = Vec::with_capacity(rgb_data.len() * (3 * 4));

        for rgb in rgb_data {
            rgba.extend_from_slice(&rgb.data[0].to_ne_bytes());
            rgba.extend_from_slice(&rgb.data[1].to_ne_bytes());
            rgba.extend_from_slice(&rgb.data[2].to_ne_bytes());
        }

        vertical_flip(&mut rgba, info.width as usize, info.height as usize, 3 * 4);

        let mut texture_id: u32 = 0;
        unsafe {
            glGenTextures(1, &mut texture_id);
            glBindTexture(GL_TEXTURE_2D, texture_id);

            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR.0 as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);

            let format: GLenum = GL_RGB;
            let texture_size: GLenum = GL_FLOAT;

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGB.0 as i32,
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
}

fn vertical_flip(pixels: &mut Vec<u8>, width: usize, height: usize, bytes_per_pixel: usize) {
    let end = height >> 1;
    let bytes_per_row = width * bytes_per_pixel;
    for row in 0..end {
        unsafe {
            let row0 = pixels.as_mut_ptr().add(row * bytes_per_row);
            let row1 = pixels.as_mut_ptr().add((height - row - 1) * bytes_per_row);
            std::ptr::swap_nonoverlapping(row0, row1, bytes_per_row);
        }
    }
}
