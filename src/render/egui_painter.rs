// egui_painter.rs
//
// Created on 2021/12/02 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use super::{stream, texture};

use std::collections::HashMap;

use ahash;
use egui::{ClippedPrimitive, Color32};

use gl;

use super::shader;
use crate::iml;

pub struct EguiPainter {
    pipeline: shader::Pipeline,
    delete_buffers: Vec<u32>,
    texture_map: HashMap<egui::TextureId, texture::TexturePointer>,
}

impl EguiPainter {
    pub fn new() -> EguiPainter {
        EguiPainter {
            pipeline: shader::Pipeline::new(
                "resources/shaders/egui.vs",
                "resources/shaders/egui.fs",
            )
            .unwrap(),
            delete_buffers: Vec::new(),
            texture_map: HashMap::new(),
        }
    }

    pub fn paint(
        &mut self,
        clipped_primitives: Vec<ClippedPrimitive>,
        egui_texture: egui::TexturesDelta,
        window_size: iml::Vec2,
    ) {
        for buffer in &self.delete_buffers {
            unsafe {
                gl::DeleteBuffers(1, buffer);
            }
        }

        self.delete_buffers.clear();
        self.remove_egui_textures(egui_texture.free);
        self.add_egui_textures(egui_texture.set);
        for clipped_primitive in clipped_primitives {
            match clipped_primitive.primitive {
                egui::epaint::Primitive::Mesh(mesh) => self.paint_primitive(mesh, &window_size),
                egui::epaint::Primitive::Callback(_) => panic!("unsupported epaint type: Callback"),
            }
        }
    }

    fn paint_primitive(&mut self, mesh: egui::epaint::Mesh, window_size: &iml::Vec2) {
        let indices: &Vec<u32> = &mesh.indices;
        let vertices: &Vec<egui::epaint::Vertex> = &mesh.vertices;

        let texture_pointer = self.texture_map.get(&mesh.texture_id);
        let texture_id = match texture_pointer {
            Some(texture) => texture.id,
            None => 0,
        };

        let mut vertex_buffer: u32 = 0;
        let mut index_buffer: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<egui::epaint::Vertex>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut index_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

            // pos
            gl::VertexAttribPointer(
                0 as u32,
                2 as i32,
                gl::FLOAT,
                0,
                std::mem::size_of::<egui::epaint::Vertex>() as i32,
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0 as u32);

            // ui
            gl::VertexAttribPointer(
                1 as u32,
                2 as i32,
                gl::FLOAT,
                0,
                std::mem::size_of::<egui::epaint::Vertex>() as i32,
                std::mem::size_of::<egui::epaint::Pos2>() as *const _,
            );
            gl::EnableVertexAttribArray(1 as u32);

            // color
            gl::VertexAttribPointer(
                2 as u32,
                4 as i32,
                gl::UNSIGNED_BYTE,
                0,
                std::mem::size_of::<egui::epaint::Vertex>() as i32,
                (std::mem::size_of::<egui::epaint::Pos2>() * 2) as *const _,
            );
            gl::EnableVertexAttribArray(2 as u32);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);

            gl::UseProgram(self.pipeline.id);
            self.pipeline
                .set_uniform_vec2("window_size\0", &window_size);

            self.pipeline.set_uniform_1i("u_sampler\0", 0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            self.delete_buffers.push(vertex_buffer);
            self.delete_buffers.push(index_buffer);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
    }

    fn add_egui_textures(
        &mut self,
        egui_texure_map: ahash::AHashMap<egui::TextureId, egui::epaint::image::ImageDelta>,
    ) {
        for (texture_id, image_delta) in egui_texure_map.iter() {
            if self.texture_map.get(&texture_id).is_none() {
                match &image_delta.image {
                    egui::epaint::image::ImageData::Color(image) => {
                        let width = image.size[0];
                        let height = image.size[1];
                        let texture_pointer =
                            generate_gl_texture_from_egui_color_image(width, height, &image.pixels);
                        self.texture_map.insert(*texture_id, texture_pointer);
                    }
                    egui::epaint::image::ImageData::Font(image) => {
                        let texture_pointer = generate_gl_texture_from_egui_front_image(image);
                        self.texture_map.insert(*texture_id, texture_pointer);
                    }
                }
            }
        }
    }

    fn remove_egui_textures(&mut self, textures: Vec<egui::TextureId>) {
        for texture in textures {
            let texture_pointer = self.texture_map.remove(&texture);

            match texture_pointer {
                Some(texture) => unsafe {
                    gl::DeleteTextures(1, &texture.id);
                },
                None => println!("Invalid texture to delete"),
            }
        }
    }
}

fn generate_gl_texture_from_egui_color_image(
    width: usize,
    height: usize,
    color_pixels: &Vec<Color32>,
) -> texture::TexturePointer {
    let mut pixels: Vec<u8> = Vec::with_capacity(color_pixels.len() * 4);
    for &color in color_pixels {
        let srgba = color.to_array();
        pixels.push(srgba[0]);
        pixels.push(srgba[1]);
        pixels.push(srgba[2]);
        pixels.push(srgba[3]);
    }

    let format = stream::Format::new(
        stream::Dimension::VEC4,
        stream::Type::UINT8,
        stream::Usage::RGBA,
    );

    texture::Texture::new(
        &pixels,
        texture::TextureDesc::default(),
        width as u32,
        height as u32,
        format,
        texture::Type::Tex2D,
    )
}

fn generate_gl_texture_from_egui_front_image(image: &egui::FontImage) -> texture::TexturePointer {
    let pixel_iterator = image.srgba_pixels(1.0);
    let mut pixels: Vec<u8> = Vec::with_capacity(pixel_iterator.len() * 4);

    for color in pixel_iterator {
        pixels.push(color[0]);
        pixels.push(color[1]);
        pixels.push(color[2]);
        pixels.push(color[3]);
    }

    let format = stream::Format::new(
        stream::Dimension::VEC4,
        stream::Type::UINT8,
        stream::Usage::RGBA,
    );

    texture::Texture::new(
        &pixels,
        texture::TextureDesc::default(),
        image.width() as u32,
        image.height() as u32,
        format,
        texture::Type::Tex2D,
    )
}
