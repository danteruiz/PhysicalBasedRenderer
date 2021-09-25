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
    Float,
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
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Num,
}
struct Format {}
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
    start_index: u32,
    num_indices: u32,
    material_index: u32,
}

struct Mesh {
    sub_meshes: Vec<SubMesh>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

fn main() {
    let (gltf, buffers, _) = gltf::import(
        "resources/glTF-models/DamagedHelmet/glTF-Binary/DamagedHelmet.glb",
    )
    .expect("Failed to load file");

    // let gltf = gltf::Gltf::open(
    //     "resources/glTF-models/DamagedHelmet/glTF/DamagedHelmet.gltf",
    // )
    // .expect("Failed to load file");

    // for scene in gltf.scenes() {
    //     for node in scene.nodes() {
    //         println!(
    //             "Node #{} has {} children",
    //             node.index(),
    //             node.children().count()
    //         );
    //
    //         let mesh = node.mesh().expect("expected to get a mesh");
    //
    //         for primitive in mesh.primitives() {
    //             println!("- Primitive #{}", primitive.index());
    //
    //             let reader =
    //                 primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    //
    //             let iter = reader.read_normals().expect("expected position");
    //
    //             for vertex_position in iter {
    //                 println!("{:?}", vertex_position);
    //             }
    //             // if Some(iter) = reader.read_positions() {
    //             //     for vertex_position in iter {
    //             //         println!("{:?}", vertex_position);
    //             //     }
    //             // }
    //         }
    //     }
    // }

    println!("after loading the data");
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
