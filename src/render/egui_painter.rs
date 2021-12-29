// egui_painter.rs
//
// Created on 2021/12/02 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use egui::ClippedMesh;
use gl;

use super::shader;
use crate::math;

pub struct EguiPainter {
    pipeline: shader::Pipeline,
}

impl EguiPainter {
    pub fn new() -> EguiPainter {
        EguiPainter {
            pipeline: shader::Pipeline::new(
                "resources/shaders/egui.vs",
                "resources/shaders/egui.fs",
            )
            .unwrap(),
        }
    }

    pub fn paint(
        &self,
        clipped_meshes: &Vec<ClippedMesh>,
        egui_texture: &egui::epaint::Texture,
        window_size: &math::Vec2,
    ) {
        let egui_texture_id = generate_gl_texture_from_egui_texture(&egui_texture);
        for clipped_mesh in clipped_meshes {
            let mesh: &egui::epaint::Mesh = &clipped_mesh.1;

            let indices: &Vec<u32> = &mesh.indices;
            let vertices: &Vec<egui::epaint::Vertex> = &mesh.vertices;

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
                gl::BindTexture(gl::TEXTURE_2D, egui_texture_id);
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    0 as *const _,
                );

                gl::DeleteBuffers(1, &mut vertex_buffer);
                gl::DeleteBuffers(1, &mut index_buffer);
            }
        }
    }
}

fn generate_gl_texture_from_egui_texture(egui_texture: &egui::epaint::Texture) -> u32 {
    let mut texture_id: u32 = 0;

    let mut pixels: Vec<u8> = Vec::with_capacity(egui_texture.pixels.len() * 4);
    for &alpha in &egui_texture.pixels {
        let srgba = egui::epaint::Color32::from_white_alpha(alpha);
        pixels.push(srgba[0]);
        pixels.push(srgba[1]);
        pixels.push(srgba[2]);
        pixels.push(srgba[3]);
    }

    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let format = gl::RGBA;
        let data_size = gl::UNSIGNED_BYTE;

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            egui_texture.width as i32,
            egui_texture.height as i32,
            0,
            format,
            data_size,
            pixels.as_ptr().cast(),
        );
    }

    texture_id
}
