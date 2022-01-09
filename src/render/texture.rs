// texture.rs
//
// Created on 2021/11/29 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/
extern crate gl;

use super::gl_utils;
use super::stream;
use gl::types::GLenum;
use image;

use std::boxed::Box;
use std::collections::HashMap;
use std::fs;

const WHITE_COLOR: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
const BLUE_COLOR: [u8; 4] = [0x80, 0x80, 0xFF, 0xFF];
const GRAY_COLOR: [u8; 4] = [0x80, 0x80, 0x80, 0xFF];
const BLACK_COLOR: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

pub enum Type {
    Tex2D,
    TexCUBE,
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

pub struct Texture {
    pub id: u32,
    pub format: stream::Format,
    pub _type: Type,
    pub width: u32,
    pub height: u32,
    pub wrap_mode: i32,
    pub filter_mode: i32,
}

pub type TexturePointer = Box<Texture>;
impl Texture {
    pub fn new(
        pixels: &Vec<u8>,
        wrap_mode: i32,
        filter_mode: i32,
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
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_mode);

            let data_format = GLenum::from(format.usage);
            let data_type = GLenum::from(format._type);

            println!("{:?}", data_format);
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
            wrap_mode,
            filter_mode,
        };
        Box::new(texture)
    }
}

static TEXTURE_COUNT: u32 = 0;
type TextureHandle = usize;
type TextureHandleMap = HashMap<String, TextureHandle>;
pub struct TextureCache {
    pub blue_texture: Box<Texture>,
    pub white_texture: Box<Texture>,
    pub gray_texture: Box<Texture>,
    pub black_texture: Box<Texture>,
    texture_handle_map: TextureHandleMap,
    textures: Vec<Box<Texture>>,
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
                gl::CLAMP_TO_EDGE as i32,
                gl::LINEAR as i32,
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
            white_texture: Texture::new(
                &Vec::from(WHITE_COLOR),
                gl::CLAMP_TO_EDGE as i32,
                gl::LINEAR as i32,
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
            gray_texture: Texture::new(
                &Vec::from(GRAY_COLOR),
                gl::CLAMP_TO_EDGE as i32,
                gl::LINEAR as i32,
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
            black_texture: Texture::new(
                &Vec::from(BLACK_COLOR),
                gl::CLAMP_TO_EDGE as i32,
                gl::LINEAR as i32,
                1,
                1,
                format.clone(),
                Type::Tex2D,
            ),
            texture_handle_map: TextureHandleMap::new(),
            textures: Vec::new(),
        }
    }

    pub fn texture_from_handle(&mut self, texture_handle: TextureHandle) -> *mut Texture {
        self.textures[texture_handle].as_mut()
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
