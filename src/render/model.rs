// model.rs
//
// Created on 2022/01/07 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::collections::HashMap;

use super::buffer;
use super::stream::{Attribute, Dimension, Format, Slot, Type, Usage};

#[derive(Hash, PartialEq, Eq)]
pub enum Shape {
    Cube,
    Sphere,
    Quad,
}

pub struct SubMesh {
    pub start_index: usize,
    pub num_indices: usize,
}

pub struct Mesh {
    pub buffer_id: u32,
    pub index_id: u32,
    pub sub_meshes: Vec<SubMesh>,
    pub attributes: Vec<Attribute>,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

pub type ModelPointer = Box<Model>;
type ShapeMap = HashMap<Shape, ModelPointer>;

pub struct ModelCache {
    shape_map: HashMap<Shape, ModelPointer>,
}

impl ModelCache {
    pub fn new() -> ModelCache {
        let mut shape_map = ShapeMap::new();
        shape_map.insert(Shape::Quad, generate_quad_model());
        shape_map.insert(Shape::Cube, generate_cube_model());
        shape_map.insert(Shape::Sphere, generate_sphere_model());

        ModelCache { shape_map }
    }

    pub fn shape(&self, shape: &Shape) -> &Model {
        self.shape_map[shape].as_ref()
    }
}

static PI: f32 = 3.14159265359;
static X_SEGMENTS: f32 = 64.0;
static Y_SEGMENTS: f32 = 64.0;

fn generate_quad_model() -> ModelPointer {
    let positions: [f32; 12] = [
        -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0,
    ];

    let normals: [f32; 12] = [0.0; 12];
    let tex_coords: [f32; 8] = [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0];
    let indices: [i32; 6] = [0, 1, 2, 1, 2, 3];

    let sub_mesh: SubMesh = SubMesh {
        start_index: 0,
        num_indices: 6,
    };

    let position_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Position,
        offset: 0,
    };

    let normal_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Normal,
        offset: positions.len(),
    };

    let tex_coord_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC2, Type::FLOAT, Usage::DATA),
        slot: Slot::TexCoord,
        offset: positions.len() + normals.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions));
    buffer_data.extend_from_slice(to_byte_slice(&normals));
    buffer_data.extend_from_slice(to_byte_slice(&tex_coords));

    let vertex_buffer: buffer::Buffer = buffer::Buffer { data: buffer_data };

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices));
    let index_buffer: buffer::Buffer = buffer::Buffer {
        data: index_buffer_data,
    };

    let mut vertex_id: u32 = 0;
    let mut index_id: u32 = 0;

    unsafe {
        gl::GenBuffers(1, &mut vertex_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            vertex_buffer.data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut index_id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (index_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            index_buffer.data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
    }

    let mesh: Mesh = Mesh {
        buffer_id: vertex_id,
        index_id,
        sub_meshes: vec![sub_mesh],
        attributes: vec![position_attribute, normal_attribute, tex_coord_attribute],
    };

    ModelPointer::new(Model { meshes: vec![mesh] })
}

fn generate_cube_model() -> ModelPointer {
    let positions: Vec<f32> = vec![
        // right side 0 - 3
        1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, // top side 4 - 7
        1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0,
        // bottom side 8 - 11
        1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0,
        // left side 12 -15
        -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
        // front 16 - 19
        1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0,
        // back 20 - 23
        1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
    ];

    let indices: Vec<i32> = vec![
        0, 1, 2, 2, 1, 3, // right side
        4, 5, 6, 4, 6, 7, // top side
        8, 9, 10, 8, 10, 11, // bottom
        12, 13, 14, 13, 14, 15, // left
        16, 17, 18, 16, 18, 19, // front
        20, 21, 22, 20, 22, 23, // back
    ];

    let position_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Position,
        offset: 0,
    };

    let sub_mesh: SubMesh = SubMesh {
        start_index: 0,
        num_indices: indices.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions));

    let vertex_buffer: buffer::Buffer = buffer::Buffer { data: buffer_data };

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices));
    let index_buffer: buffer::Buffer = buffer::Buffer {
        data: index_buffer_data,
    };

    let mut vertex_id: u32 = 0;
    let mut index_id: u32 = 0;

    unsafe {
        gl::GenBuffers(1, &mut vertex_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            vertex_buffer.data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut index_id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (index_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            index_buffer.data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
    }

    let mesh: Mesh = Mesh {
        buffer_id: vertex_id,
        index_id,
        sub_meshes: vec![sub_mesh],
        attributes: vec![position_attribute],
    };

    ModelPointer::new(Model { meshes: vec![mesh] })
}

fn generate_sphere_model() -> ModelPointer {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();

    for y in 0..=Y_SEGMENTS as i32 {
        for x in 0..=X_SEGMENTS as i32 {
            let x_segment: f32 = x as f32 / X_SEGMENTS;
            let y_segment: f32 = y as f32 / Y_SEGMENTS;

            let x_pos: f32 = (x_segment * 2.0 * PI).cos() * (y_segment * PI).sin();
            let y_pos: f32 = (y_segment * PI).cos();
            let z_pos: f32 = (x_segment * 2.0 * PI).sin() * (y_segment * PI).sin();

            positions.push(x_pos);
            positions.push(y_pos);
            positions.push(z_pos);

            normals.push(x_pos);
            normals.push(y_pos);
            normals.push(z_pos);
        }
    }

    let mut indices: Vec<u32> = Vec::new();
    for i in 0..Y_SEGMENTS as u32 {
        let mut k1 = i * (X_SEGMENTS as u32 + 1);
        let mut k2 = k1 + X_SEGMENTS as u32 + 1;

        for _ in 0..X_SEGMENTS as i32 {
            if i as f32 != 0.0 {
                indices.push(k1);
                indices.push(k2);
                indices.push(k1 + 1);
            }

            if i != (Y_SEGMENTS as u32 - 1) {
                indices.push(k1 + 1);
                indices.push(k2);
                indices.push(k2 + 1);
            }
            k1 += 1;
            k2 += 1;
        }
    }

    let sub_mesh: SubMesh = SubMesh {
        start_index: 0,
        num_indices: indices.len(),
    };

    let position_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Position,
        offset: 0,
    };

    let normal_attribute: Attribute = Attribute {
        format: Format {
            usage: Usage::DATA,
            dimension: Dimension::VEC3,
            _type: Type::FLOAT,
        },
        slot: Slot::Normal,
        offset: positions.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions[..]));
    buffer_data.extend_from_slice(to_byte_slice(&normals[..]));
    let vertex_buffer: buffer::Buffer = buffer::Buffer { data: buffer_data };

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices[..]));
    let index_buffer: buffer::Buffer = buffer::Buffer {
        data: index_buffer_data,
    };

    let mut vertex_id: u32 = 0;
    let mut index_id: u32 = 0;

    unsafe {
        gl::GenBuffers(1, &mut vertex_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            vertex_buffer.data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut index_id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_id);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (index_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            index_buffer.data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
    }

    let mesh: Mesh = Mesh {
        buffer_id: vertex_id,
        index_id,
        sub_meshes: vec![sub_mesh],
        attributes: vec![position_attribute, normal_attribute],
    };

    ModelPointer::new(Model { meshes: vec![mesh] })
}

fn to_byte_slice<'a, T>(floats: &'a [T]) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            floats.as_ptr() as *const _,
            floats.len() * std::mem::size_of::<T>(),
        )
    }
}
