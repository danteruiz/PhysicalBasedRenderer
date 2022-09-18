// texture.rs
//
// Created on 2021/11/29 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/
extern crate gl;

use super::stream;
use gl::types::GLenum;
use image;

use std::boxed::Box;
//use std::collections::HashMap;
use std::fs;

const WHITE_COLOR: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
const BLUE_COLOR: [u8; 4] = [0x80, 0x80, 0xFF, 0xFF];
const GRAY_COLOR: [u8; 4] = [0x80, 0x80, 0x80, 0xFF];
const BLACK_COLOR: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

pub enum Type {
    Tex2D,
    TexCUBE,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum WrapMode {
    REPEAT,
    MIRROR,
    CLAMP,
    BORDER,
    MIRROR_REPEAT,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Filter {
    NEAREST,
    LINEAR,
    NEAREST_MIP_NEAREST,
    LINEAR_MIP_NEAREST,
    NEAREST_MIP_LINEAR,
    LINEAR_MIP_LINEAR,
}

pub fn load_hdr_texture(path: &'static str) -> (Vec<u8>, u32, u32) {
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

    (rgba, info.width, info.height)
}

pub struct TextureDesc {
    pub wrap_s: WrapMode,
    pub wrap_t: WrapMode,
    pub min_filter: Filter,
    pub mag_filter: Filter,
}

impl Default for TextureDesc {
    fn default() -> Self {
        Self {
            wrap_s: WrapMode::CLAMP,
            wrap_t: WrapMode::CLAMP,
            min_filter: Filter::LINEAR,
            mag_filter: Filter::LINEAR,
        }
    }
}

pub struct Texture {
    pub id: u32,
    pub format: stream::Format,
    pub _type: Type,
    pub width: u32,
    pub height: u32,
    pub texture_desc: TextureDesc,
}

pub type TexturePointer = Box<Texture>;
impl Texture {
    pub fn new(
        pixels: &Vec<u8>,
        texture_desc: TextureDesc,
        width: u32,
        height: u32,
        format: stream::Format,
        _type: Type,
    ) -> Box<Texture> {
        let mut texture_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            let wrap_s = GLenum::from(texture_desc.wrap_s);
            let wrap_t = GLenum::from(texture_desc.wrap_t);
            let min_filter = GLenum::from(texture_desc.min_filter);
            let mag_filter = GLenum::from(texture_desc.mag_filter);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);

            let data_format = GLenum::from(format.usage);
            let data_type = GLenum::from(format._type);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                data_format as i32,
                width as i32,
                height as i32,
                0,
                data_format,
                data_type,
                pixels.as_ptr().cast(),
            );
        }

        let texture = Texture {
            id: texture_id,
            format,
            _type,
            width,
            height,
            texture_desc,
        };
        Box::new(texture)
    }
}

pub struct TextureCache {
    pub blue_texture: Box<Texture>,
    pub white_texture: Box<Texture>,
    pub gray_texture: Box<Texture>,
    pub black_texture: Box<Texture>,
}

impl TextureCache {
    pub fn new() -> TextureCache {
        let format = stream::Format::new(
            stream::Dimension::VEC3,
            stream::Type::UINT8,
            stream::Usage::RGBA,
        );

        TextureCache {
            blue_texture: Texture::new(
                &Vec::from(BLUE_COLOR),
                TextureDesc::default(),
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
            white_texture: Texture::new(
                &Vec::from(WHITE_COLOR),
                TextureDesc::default(),
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
            gray_texture: Texture::new(
                &Vec::from(GRAY_COLOR),
                TextureDesc::default(),
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
            black_texture: Texture::new(
                &Vec::from(BLACK_COLOR),
                TextureDesc::default(),
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
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
