// main.rs
//
// Created on 2021/09/24 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

extern crate glfw;
extern crate gltf;

use glfw::Context;
use std::fmt;
use std::mem;

#[repr(u8)]
enum Topology {
    Points,
    Lines,
    LineStrip,
    Triangles,
}

#[repr(u8)]
enum Type {
    Float = 0,
    Int8,
    Int16,
    Int32,
    UInt8,
    UInt16,
    UInt32,
    Num,
}

#[repr(u8)]
enum Dimension {
    Scalar = 0,
    Vec2,
    Vec3,
    Vec4,
    Num,
}
struct Format {
    dimension: Dimension,
}

#[repr(u8)]
enum Slot {
    Position = 0,
    Normal,
    TexCoord,
    Color,
}

struct Attribute {
    format: Format,
    slot: Slot,
    offset: usize,
}

impl Attribute {
    fn get_total_offset(self) -> usize {
        return self.offset;
    }
}

const TYPE_SIZE: [usize; Type::Num as usize] = [4, 2, 4, 1, 4, 2, 1];
const DIMENSION_SIZE: [usize; Dimension::Num as usize] = [1, 2, 3, 4];
struct BufferView {
    offset: usize,
    size: usize,
    format: Format,
}

struct Buffer {
    data: Vec<u8>,
    dirty: bool,
}

struct SubMesh {
    start_index: usize,
    num_indices: usize,
    material_index: usize,
}

struct Mesh {
    sub_meshes: Vec<SubMesh>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

struct Model {
    meshes: Vec<Mesh>,
}

fn get_buffer_data(
    semantic: gltf::mesh::Semantic,
    primitive: &gltf::Primitive,
    buffers: &Vec<gltf::buffer::Data>,
) -> Vec<u8> {
    let accessor = primitive
        .get(&semantic)
        .expect(format!("no eccessor for type {:?}", semantic).as_str());

    let buffer_view = accessor
        .view()
        .expect(format!("Failed to get buffer for accessor {:?}", semantic).as_str());

    let buffer = buffer_view.buffer();

    let buffer_data = buffers
        .get(buffer.index())
        .expect(format!("Failed to get buffer_data for accessor {:?}", semantic).as_str());

    let start = buffer_view.offset();
    let end = start + buffer_view.length();
    let buffer_data_slice = &buffer_data[start..end];

    for value in buffer_data_slice {
        println!("value {}", value);
    }

    return Vec::new();
}

fn main() {
    let (gltf, buffers, _) =
        gltf::import("resources/glTF-models/DamagedHelmet.glb").expect("Failed to load file");

    let model: Model = Model { meshes: Vec::new() };
    for mesh in gltf.meshes() {
        let mut positions: Vec<u8> = Vec::new();
        let mut normals: Vec<u8> = Vec::new();
        let mut tex_coords: Vec<u8> = Vec::new();

        let indices: Vec<u32> = Vec::new();

        for primitive in mesh.primitives() {
            positions.append(&mut get_buffer_data(
                gltf::mesh::Semantic::Positions,
                &primitive,
                &buffers,
            ));
            // normals.append(&mut get_buffer_data(
            //     gltf::mesh::Semantic::Normals,
            //     &primitive,
            //     &buffers,
            // ));
            // tex_coords.append(&mut get_buffer_data(
            //     gltf::mesh::Semantic::TexCoords(0),
            //     &primitive,
            //     &buffers,
            // ));
        }
    }

    // println!("after loading the data");
    // glfw::init_hint(glfw::InitHint::JoystickHatButtons(false));
    // let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    //
    // // Create a windowed mode window and its OpenGL context
    // let (mut window, events) = glfw
    //     .create_window(
    //         300,
    //         300,
    //         "Hello this is window",
    //         glfw::WindowMode::Windowed,
    //     )
    //     .expect("Failed to create GLFW window.");
    //
    // // Make the window's context current
    // //window.make_current();
    // window.make_current();
    // window.set_key_polling(true);
    //
    // // Loop until the user closes the window
    // while !window.should_close() {
    //     // Swap front and back buffers
    //     window.swap_buffers();
    //
    //     // Poll for and process events
    //     glfw.poll_events();
    //     for (_, event) in glfw::flush_messages(&events) {
    //         println!("{:?}", event);
    //         match event {
    //             glfw::WindowEvent::Key(
    //                 glfw::Key::Escape,
    //                 _,
    //                 glfw::Action::Press,
    //                 _,
    //             ) => window.set_should_close(true),
    //             _ => {}
    //         }
    //     }
    // }
}
