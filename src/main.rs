// main.rs
//
// Created on 2021/09/24 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

extern crate gl33;
extern crate glfw;
extern crate gltf;
extern crate glutin;
extern crate winit;

mod math;
mod render;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use gl33::{gl_enumerations::*, global_loader::*};

// use winit::event::{ElementState, Event, KeyboradInput, VirtualKeyCode, WindowEvent};
// use winit::event_loop::{ControlFlow, EventLoop};
//
// use ash::version::EntryV1_0;
// use ash::version::Instancev1_0;
// use std::fmt;
// use std::mem;

// #[repr(u8)]
// enum Topology {
//     Points,
//     Lines,
//     LineStrip,
//     Triangles,
// }

#[allow(dead_code)]
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

#[allow(dead_code)]
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
    m_type: Type,
}

#[repr(u8)]
enum Slot {
    Position = 0,
    Normal,
    // TexCoord,
    // Color,
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

// const TYPE_SIZE: [usize; Type::Num as usize] = [4, 2, 4, 1, 4, 2, 1];
// const DIMENSION_SIZE: [usize; Dimension::Num as usize] = [1, 2, 3, 4];
// struct BufferView {
//     offset: usize,
//     size: usize,
//     format: Format,
// }

struct Buffer {
    data: Vec<u8>,
}

struct SubMesh {
    start_index: usize,
    num_indices: usize,
    material_index: usize,
}

struct Mesh {
    sub_meshes: Vec<SubMesh>,
    attributes: Vec<Attribute>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

struct Model {
    meshes: Vec<Mesh>,
}

// fn get_buffer_data(
//     semantic: gltf::mesh::Semantic,
//     primitive: &gltf::Primitive,
//     buffers: &Vec<gltf::buffer::Data>,
// ) -> Vec<u8> {
//     let accessor = primitive
//         .get(&semantic)
//         .expect(format!("no eccessor for type {:?}", semantic).as_str());
//
//     let buffer_view = accessor
//         .view()
//         .expect(format!("Failed to get buffer for accessor {:?}", semantic).as_str());
//
//     let buffer = buffer_view.buffer();
//
//     let buffer_data = buffers
//         .get(buffer.index())
//         .expect(format!("Failed to get buffer_data for accessor {:?}", semantic).as_str());
//
//     let start = buffer_view.offset();
//     let end = start + buffer_view.length();
//     let buffer_data_slice = &buffer_data[start..end];
//
//     return Vec::from(buffer_data_slice);
// }

// let (gltf, buffers, _) =
//     gltf::import("resources/glTF-models/DamagedHelmet.glb").expect("Failed to load file");
//
// let model: Model = Model { meshes: Vec::new() };
// for mesh in gltf.meshes() {
//     let mut positions: Vec<u8> = Vec::new();
//     let mut normals: Vec<u8> = Vec::new();
//     let mut tex_coords: Vec<u8> = Vec::new();
//
//     let indices: Vec<u32> = Vec::new();
//
//     for primitive in mesh.primitives() {
//         positions.append(&mut get_buffer_data(
//             gltf::mesh::Semantic::Positions,
//             &primitive,
//             &buffers,
//         ));
//         normals.append(&mut get_buffer_data(
//             gltf::mesh::Semantic::Normals,
//             &primitive,
//             &buffers,
//         ));
//         tex_coords.append(&mut get_buffer_data(
//             gltf::mesh::Semantic::TexCoords(0),
//             &primitive,
//             &buffers,
//         ));
//
//         // get indcies
//
//         let indices_accessor = primitive.indices().expect("expecting a buffer");
//
//         println!(
//             "component size: {}, dimensions: {:?}, count: {}, DataType: {:?}",
//             indices_accessor.size(),
//             indices_accessor.dimensions(),
//             indices_accessor.count(),
//             indices_accessor.data_type()
//         );
//
//         //	    let index_buffer_view = indices_accessor =
//     }
// }

// println!("after loading the data");
// glfw::init_hint(glfw::InitHint::JoystickHatButtons(false));
// let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

fn to_byte_slice<'a>(floats: &'a [f32]) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(floats.as_ptr() as *const _, floats.len() * 4) }
}
const WINDOW_TITLE: &'static str = "Physical Based Renderer";

static PI: f32 = 3.14159265359;
static X_SEGMENTS: f32 = 64.0;
static Y_SEGMENTS: f32 = 64.0;

fn generate_sphere_model() -> Model {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();

    for y in 0..Y_SEGMENTS as i32 {
        for x in 0..X_SEGMENTS as i32 {
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

    let mut indices: Vec<f32> = Vec::new();
    for i in 0..Y_SEGMENTS as i32 {
        let k1 = i as f32 * (X_SEGMENTS + 1.0);
        let k2 = k1 + X_SEGMENTS + 1.0;

        for j in 0..X_SEGMENTS as i32 {
            if i as f32 != 0.0 {
                indices.push(k1);
                indices.push(k2);
                indices.push(k1 + 1.0);
            }

            if i as f32 != (Y_SEGMENTS - 1.0) {
                indices.push(k1 + 1.0);
                indices.push(k2);
                indices.push(k2 + 1.0);
            }
        }
    }

    let sub_mesh: SubMesh = SubMesh {
        start_index: 0,
        num_indices: indices.len(),
        material_index: 0,
    };

    let buffer: Vec<u8> = Vec::new();

    let position_attribute: Attribute = Attribute {
        format: Format {
            dimension: Dimension::Vec3,
            m_type: Type::Float,
        },
        slot: Slot::Position,
        offset: 0,
    };

    let normal_attribute: Attribute = Attribute {
        format: Format {
            dimension: Dimension::Vec3,
            m_type: Type::Float,
        },
        slot: Slot::Normal,
        offset: positions.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions[..]));
    buffer_data.extend_from_slice(to_byte_slice(&normals[..]));
    let vertex_buffer: Buffer = Buffer { data: buffer_data };

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices[..]));
    let index_buffer: Buffer = Buffer {
        data: index_buffer_data,
    };

    let mesh: Mesh = Mesh {
        sub_meshes: vec![sub_mesh],
        attributes: vec![position_attribute, normal_attribute],
        vertex_buffer: vertex_buffer,
        index_buffer: index_buffer,
    };

    Model { meshes: vec![mesh] }
}
fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title(WINDOW_TITLE);

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };
    unsafe {
        gl33::global_loader::load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            context.get_proc_address(r_str) as _
        });
    }

    let sphere_model: Model = generate_sphere_model();

    let fragment_shader_file: &'static str = "resources/shaders/pbr.fs";
    let vertex_shader_file: &'static str = "resources/shaders/pbr.vs";
    //render::shader::print_file_contents();

    let pipeline = render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                // Application update code.

                unsafe {
                    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                    glClearColor(0.0, 0.0, 1.0, 1.0);
                }

                let _ = context.swap_buffers();
                //context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
            }
            _ => (),
        }
    });
}
