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

use gl33::{gl_enumerations::*, global_loader::*};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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
#[derive(Copy, Debug, Clone)]
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
#[derive(Copy, Debug, Clone)]
enum Dimension {
    Scalar = 0,
    Vec2,
    Vec3,
    Vec4,
    Num,
}

#[derive(Copy, Debug, Clone)]
struct Format {
    dimension: Dimension,
    m_type: Type,
}
const TYPE_SIZE: [usize; Type::Num as usize] = [4, 2, 4, 1, 4, 2, 1];
const DIMENSION_SIZE: [usize; Dimension::Num as usize] = [1, 2, 3, 4];
impl Format {
    fn get_type_size(&self) -> usize {
        TYPE_SIZE[self.m_type as usize]
    }
    fn get_dimension_size(&self) -> usize {
        DIMENSION_SIZE[self.dimension as usize]
    }

    fn get_stride(self) -> usize {
        self.get_type_size() * self.get_dimension_size()
    }
}

#[repr(u8)]
#[derive(Copy, Debug, Clone)]
enum Slot {
    Position = 0,
    Normal,
    // TexCoord,
    // Color,
}

#[derive(Copy, Debug, Clone)]
struct Attribute {
    format: Format,
    slot: Slot,
    offset: usize,
}

impl Attribute {
    fn get_total_offset(self) -> usize {
        return self.offset * self.format.get_type_size();
    }
}

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
    gl_buffer_id: u32,
    gl_index_id: u32,
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

fn to_byte_slice<'a, T>(floats: &'a [T]) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            floats.as_ptr() as *const _,
            floats.len() * std::mem::size_of::<T>(),
        )
    }
}
const WINDOW_TITLE: &'static str = "Physical Based Renderer";

static PI: f32 = 3.14159265359;
static X_SEGMENTS: f32 = 32.0;
static Y_SEGMENTS: f32 = 32.0;

fn generate_sphere_model() -> Model {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();

    for y in 0..=Y_SEGMENTS as i32 {
        for x in 0..=X_SEGMENTS as i32 {
            let x_segment: f32 = x as f32 / X_SEGMENTS;
            let y_segment: f32 = y as f32 / Y_SEGMENTS;

            let x_pos: f32 = (x_segment * 1.0 * PI).cos() * (y_segment * PI).sin();
            let y_pos: f32 = (y_segment * PI).cos();
            let z_pos: f32 = (x_segment * 1.0 * PI).sin() * (y_segment * PI).sin();

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
        material_index: 0,
    };

    //let buffer: Vec<u8> = Vec::new();

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

    let mut vertex_id: u32 = 0;
    let mut index_id: u32 = 0;

    unsafe {
        glGenBuffers(1, &mut vertex_id);
        glBindBuffer(GL_ARRAY_BUFFER, vertex_id);
        glBufferData(
            GL_ARRAY_BUFFER,
            (vertex_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            vertex_buffer.data.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        glGenBuffers(1, &mut index_id);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, index_id);
        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            (index_buffer.data.len() * std::mem::size_of::<u8>()) as isize,
            index_buffer.data.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }

    let mesh: Mesh = Mesh {
        gl_buffer_id: vertex_id,
        gl_index_id: index_id,
        sub_meshes: vec![sub_mesh],
        attributes: vec![position_attribute, normal_attribute],
        vertex_buffer: vertex_buffer,
        index_buffer: index_buffer,
    };

    Model { meshes: vec![mesh] }
}

fn render_model(
    model: &Model,
    projection: math::Mat4,
    view: math::Mat4,
    pipeline: &render::shader::Pipeline,
) {
    for mesh in &model.meshes {
        let model_matrix = math::Mat4::identity();
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, mesh.gl_buffer_id);

            for attribute in &mesh.attributes {
                let format: &Format = &attribute.format;
                glVertexAttribPointer(
                    attribute.slot as u32,
                    format.get_dimension_size() as i32,
                    GL_FLOAT,
                    0,
                    format.get_stride() as i32,
                    attribute.get_total_offset() as *const _,
                );
                glEnableVertexAttribArray(attribute.slot as u32);
            }
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, mesh.gl_index_id);
        }
        for sub_mesh in &mesh.sub_meshes {
            unsafe {
                glUseProgram(pipeline.id);

                glUniformMatrix4fv(
                    glGetUniformLocation(pipeline.id, "model\0".as_ptr()),
                    1,
                    0,
                    model_matrix.as_ptr(),
                );
                glUniformMatrix4fv(
                    glGetUniformLocation(pipeline.id, "projection\0".as_ptr()),
                    1,
                    0,
                    projection.as_ptr(),
                );
                glUniformMatrix4fv(
                    glGetUniformLocation(pipeline.id, "view\0".as_ptr()),
                    1,
                    0,
                    view.as_ptr(),
                );
                let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();

                glDrawElements(
                    GL_TRIANGLES,
                    sub_mesh.num_indices as i32,
                    GL_UNSIGNED_INT,
                    start_index as *const _,
                );
            }
        }
    }
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

    let mut vao: u32 = 0;
    unsafe {
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);
    }

    let sphere_model: Model = generate_sphere_model();

    let fragment_shader_file: &'static str = "resources/shaders/phong.fs";
    let vertex_shader_file: &'static str = "resources/shaders/debug.vs";

    let pipeline = render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file).unwrap();

    // create gl buffers for model
    let eye_position = math::Point3::new(0.0, 0.0, -4.0);
    //let camera_orientation = math::Quat::identity();
    let target_position = math::Point3::new(0.0, 0.0, 0.0);
    let view = math::shared::look_at(&eye_position, &target_position, &math::shared::UNIT_Y);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                unsafe {
                    glEnable(GL_BLEND);
                    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
                    glEnable(GL_DEPTH_TEST);
                    glEnable(GL_PROGRAM_POINT_SIZE);
                    glEnable(GL_LINE_SMOOTH);
                    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                    glClearColor(0.0, 0.0, 1.0, 1.0);
                }
                let &window = &context.window();

                let inner_size = window.inner_size();
                let angle: f32 = 90.0;
                let projection = math::shared::perspective(
                    angle.to_radians(),
                    (inner_size.width as f32 / inner_size.height as f32) as f32,
                    0.3,
                    700.0,
                );

                unsafe {
                    glViewport(0, 0, inner_size.width as i32, inner_size.height as i32);
                }
                render_model(&sphere_model, projection, view, &pipeline);
                let _ = context.swap_buffers();
            }
            _ => (),
        }
    });
}
