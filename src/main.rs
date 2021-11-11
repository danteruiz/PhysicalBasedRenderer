// main.rs
//
// Created on 2021/09/24 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

extern crate egui;
extern crate gl33;
extern crate gl46;
extern crate glfw;
extern crate gltf;
extern crate glutin;
extern crate winit;

mod math;
mod render;

//use gl46::{gl_command_types::*, gl_core_types::*, gl_enumerations::*, GlFns::*};
use gl33::{gl_enumerations::*, global_loader::*};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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

struct Material {
    color: math::Vec3,
    roughness: f32,
    specular: f32,
    metallic: f32,
    ao: f32,
}

struct Light {
    intensity: f32,
    ambient: f32,
    position: math::Vec3,
    color: math::Vec3,
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

const EYE_POSITION: math::Point3 = math::Point3 {
    x: 0.0,
    y: 0.0,
    z: -2.0,
};

const light: Light = Light {
    intensity: 0.4,
    ambient: 1.0,
    position: math::Vec3 {
        x: 0.0,
        y: 4.0,
        z: 10.0,
    },
    color: math::Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
};

const material: Material = Material {
    color: math::Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    },
    roughness: 1.0,
    specular: 0.3,
    metallic: 0.5,
    ao: 1.0,
};
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

                let camera_position = EYE_POSITION * -1.0;
                pipeline.set_uniform_mat4("model\0", &model_matrix);
                pipeline.set_uniform_mat4("projection\0", &projection);
                pipeline.set_uniform_mat4("view\0", &view);

                pipeline.set_uniform_1f("light.intensity\0", light.intensity);
                pipeline.set_uniform_1f("light.ambient\0", light.ambient);
                pipeline.set_uniform_vec3("light.color\0", &light.color);
                pipeline.set_uniform_vec3("light.position\0", &light.position);
                pipeline.set_uniform_vec3("material.color\0", &material.color);
                pipeline.set_uniform_1f("material.roughness\0", material.roughness);
                pipeline.set_uniform_1f("material.metallic\0", material.metallic);
                pipeline.set_uniform_1f("material.ao\0", material.ao);
                pipeline.set_uniform_1f("material.specular\0", material.roughness);
                pipeline.set_uniform_point3("camera_position\0", &camera_position);

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
    let egui_context = egui::CtxRef::default();

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

    let fragment_shader_file: &'static str = "resources/shaders/pbr.fs";
    let vertex_shader_file: &'static str = "resources/shaders/pbr.vs";

    let mut pipeline =
        render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file).unwrap();
    // create gl buffers for model
    //let eye_position = math::Point3::new(0.0, 0.0, -4.0);
    //let camera_orientation = math::Quat::identity();
    let target_position = math::Point3::new(0.0, 0.0, 0.0);
    let view = math::shared::look_at(&EYE_POSITION, &target_position, &math::shared::UNIT_Y);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let new_pipeline = render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file);

        match new_pipeline {
            Ok(p) => {
                unsafe {
                    glDeleteShader(pipeline.id);
                }
                pipeline = p;
            }
            Err(error) => {
                println!("error: {}", error);
            }
        };
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                //let raw_input: egui::RawInput = gather_input();
                unsafe {
                    glEnable(GL_BLEND);
                    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
                    glEnable(GL_DEPTH_TEST);
                    glEnable(GL_PROGRAM_POINT_SIZE);
                    glEnable(GL_LINE_SMOOTH);
                    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                    glClearColor(0.0, 0.0, 0.0, 1.0);
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
