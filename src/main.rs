// main.rs
//
// Created on 2021/09/24 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

extern crate egui;
extern crate gl33;
extern crate gltf;
extern crate glutin;

mod math;
mod render;

//use gl46::{gl_command_types::*, gl_core_types::*, gl_enumerations::*, GlFns::*};
use gl33::{gl_enumerations::*, global_loader::*, GLenum};
use glutin::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

//use crate::render::texture;
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
pub struct Attribute {
    format: Format,
    slot: Slot,
    pub offset: usize,
}

impl Attribute {
    pub fn get_total_offset(self) -> usize {
        return self.offset * self.format.get_type_size();
    }
}

struct SubMesh {
    start_index: usize,
    num_indices: usize,
}

struct Mesh {
    gl_buffer_id: u32,
    gl_index_id: u32,
    sub_meshes: Vec<SubMesh>,
    attributes: Vec<Attribute>,
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
static X_SEGMENTS: f32 = 512.0;
static Y_SEGMENTS: f32 = 512.0;

fn generate_quad_model() -> Model {
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

    let tex_coord_attribute: Attribute = Attribute {
        format: Format {
            dimension: Dimension::Vec2,
            m_type: Type::Float,
        },
        slot: Slot::Normal,
        offset: positions.len() + normals.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions));
    buffer_data.extend_from_slice(to_byte_slice(&normals));
    buffer_data.extend_from_slice(to_byte_slice(&tex_coords));

    let vertex_buffer: render::Buffer = render::Buffer { data: buffer_data };

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices));
    let index_buffer: render::Buffer = render::Buffer {
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
        attributes: vec![position_attribute, normal_attribute, tex_coord_attribute],
    };

    Model { meshes: vec![mesh] }
}

fn generate_cube_model() -> Model {
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
        format: Format {
            dimension: Dimension::Vec3,
            m_type: Type::Float,
        },
        slot: Slot::Position,
        offset: 0,
    };

    let sub_mesh: SubMesh = SubMesh {
        start_index: 0,
        num_indices: indices.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions));

    let vertex_buffer: render::Buffer = render::Buffer { data: buffer_data };

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices));
    let index_buffer: render::Buffer = render::Buffer {
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
        attributes: vec![position_attribute],
    };

    Model { meshes: vec![mesh] }
}

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
    let vertex_buffer: render::Buffer = render::Buffer { data: buffer_data };

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices[..]));
    let index_buffer: render::Buffer = render::Buffer {
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
    };

    Model { meshes: vec![mesh] }
}

const EYE_POSITION: math::Point3 = math::Point3 {
    x: 0.0,
    y: 0.0,
    z: 2.0,
};

const LIGHT: Light = Light {
    intensity: 0.4,
    ambient: 1.0,
    position: math::Vec3 {
        x: 5.0,
        y: 7.0,
        z: 5.0,
    },
    color: math::Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
};

const MATERIAL: Material = Material {
    color: math::Vec3 {
        x: 1.0,
        y: 0.2,
        z: 0.4,
    },
    roughness: 1.0,
    metallic: 1.0,
    ao: 1.0,
};

fn render_skybox(
    model: &Model,
    projection: math::Mat4,
    view: math::Mat4,
    pipeline: &render::shader::Pipeline,
    skybox_texture: &render::texture::Texture,
) {
    unsafe {
        glDepthMask(GL_FALSE.0 as u8);
    }

    glUseProgram(pipeline.id);
    pipeline.set_uniform_mat4("projection\0", &projection);
    let new_view = math::Mat4::from(math::Mat3::from(view));
    pipeline.set_uniform_mat4("view\0", &new_view); //&math::Mat4::from(math::Mat3::from(view)));

    let mesh = &model.meshes[0];
    let sub_mesh = &mesh.sub_meshes[0];

    unsafe {
        pipeline.set_uniform_1i("skybox\0", 0);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_CUBE_MAP, skybox_texture.id);
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

        let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
        glDrawElements(
            GL_TRIANGLES,
            sub_mesh.num_indices as i32,
            GL_UNSIGNED_INT,
            start_index as *const _,
        );

        glDepthMask(GL_TRUE.0 as u8);
    }
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

                let camera_position = EYE_POSITION;
                pipeline.set_uniform_mat4("model\0", &model_matrix);
                pipeline.set_uniform_mat4("projection\0", &projection);
                pipeline.set_uniform_mat4("view\0", &view);

                pipeline.set_uniform_1f("light.intensity\0", LIGHT.intensity);
                pipeline.set_uniform_1f("light.ambient\0", LIGHT.ambient);
                pipeline.set_uniform_vec3("light.color\0", &LIGHT.color);
                pipeline.set_uniform_vec3("light.position\0", &LIGHT.position);
                pipeline.set_uniform_vec3("material.color\0", &MATERIAL.color);
                pipeline.set_uniform_1f("material.roughness\0", MATERIAL.roughness);
                pipeline.set_uniform_1f("material.metallic\0", MATERIAL.metallic);
                pipeline.set_uniform_1f("material.ao\0", MATERIAL.ao);
                pipeline.set_uniform_1f("material.specular\0", MATERIAL.roughness);
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
    let mut egui_context = egui::CtxRef::default();

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
    let cube_model: Model = generate_cube_model();

    let fragment_shader_file: &'static str = "resources/shaders/pbr.fs";
    let vertex_shader_file: &'static str = "resources/shaders/pbr.vs";

    let pipeline = render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file).unwrap();
    let skybox_pipeline =
        render::shader::Pipeline::new("resources/shaders/skybox.vs", "resources/shaders/skybox.fs")
            .unwrap();
    let target_position = math::Point3::new(0.0, 0.0, 0.0);
    let view = math::shared::look_at(&EYE_POSITION, &target_position, &math::shared::UNIT_Y);

    let mut raw_input: egui::RawInput = egui::RawInput::default();
    let mut last_mouse_pos = egui::Pos2::new(0.0 as f32, 0.0 as f32);
    let mut last_modifier = egui::Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    };

    let ibl_textures = generate_ibl_environment("resources/images/IBL/PaperMill/PaperMill.hdr");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    ..
                } => {
                    let pos2: egui::Pos2 = egui::Pos2::new(position.x as f32, position.y as f32);
                    last_mouse_pos = pos2.clone();
                    raw_input.events.push(egui::Event::PointerMoved(pos2));
                }
                WindowEvent::ModifiersChanged(modifier_state) => {
                    last_modifier = egui::Modifiers {
                        alt: modifier_state.alt(),
                        ctrl: modifier_state.ctrl(),
                        shift: modifier_state.shift(),
                        mac_cmd: false,
                        command: false,
                    }
                }
                WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    ..
                } => {
                    let button_pressed: bool = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };

                    let mouse_button: egui::PointerButton = match button {
                        glutin::event::MouseButton::Left => egui::PointerButton::Primary,
                        glutin::event::MouseButton::Right => egui::PointerButton::Secondary,
                        glutin::event::MouseButton::Middle => egui::PointerButton::Middle,
                        _ => egui::PointerButton::Primary,
                    };

                    raw_input.events.push(egui::Event::PointerButton {
                        pos: Clone::clone(&last_mouse_pos),
                        button: mouse_button,
                        pressed: button_pressed,
                        modifiers: last_modifier,
                    });
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let final_input = raw_input.clone();
                egui_context.begin_frame(final_input);
                egui::CentralPanel::default().show(&egui_context, |ui| {
                    ui.label("Hello world!");
                    if ui.button("Click me").clicked() {
                        println!("button clicked");
                    }
                });

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
                render_skybox(
                    &cube_model,
                    projection,
                    view,
                    &skybox_pipeline,
                    &ibl_textures.0,
                );
                render_model(&sphere_model, projection, view, &pipeline);

                // let (output, shapes) = egui_context.end_frame();
                // let clipped_meshes = egui_context.tessellate(shapes);

                let _ = context.swap_buffers();
            }
            _ => (),
        }
    });
}

static SKYBOX_RESOLUTION: i32 = 4096;
fn generate_skybox_texture(texture: render::texture::Texture) -> render::texture::Texture {
    let mut frame_buffer: u32 = 0;
    let mut skybox_texture: u32 = 0;
    unsafe {
        glGenFramebuffers(1, &mut frame_buffer);
        glBindFramebuffer(GL_FRAMEBUFFER, frame_buffer);
        glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_RENDERBUFFER, 0);

        glGenTextures(1, &mut skybox_texture);
        glBindTexture(GL_TEXTURE_CUBE_MAP, skybox_texture);

        for index in 0..6 {
            let texture_target: GLenum = GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + index as u32);
            glTexImage2D(
                texture_target,
                0,
                GL_RGB32F.0 as i32,
                SKYBOX_RESOLUTION,
                SKYBOX_RESOLUTION,
                0,
                GL_RGB,
                GL_FLOAT,
                std::ptr::null(),
            );

            glTexParameteri(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_WRAP_S,
                GL_CLAMP_TO_EDGE.0 as i32,
            );
            glTexParameteri(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_WRAP_T,
                GL_CLAMP_TO_EDGE.0 as i32,
            );
            glTexParameteri(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_WRAP_R,
                GL_CLAMP_TO_EDGE.0 as i32,
            );
            glTexParameteri(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_MIN_FILTER,
                GL_LINEAR.0 as i32,
            );
            glTexParameteri(
                GL_TEXTURE_CUBE_MAP,
                GL_TEXTURE_MAG_FILTER,
                GL_LINEAR.0 as i32,
            );
        }

        let angle: f32 = 90.0;
        let capture_projection: math::Mat4 =
            math::shared::perspective(angle.to_radians(), 1.0, 0.1, 10.0);

        let capture_views: Vec<math::Mat4> = vec![
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(1.0, 0.0, 0.0),
                &math::Vec3::new(0.0, -1.0, 0.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(-1.0, 0.0, 0.0),
                &math::Vec3::new(0.0, -1.0, 0.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, 1.0, 0.0),
                &math::Vec3::new(0.0, 0.0, 1.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, -1.0, 0.0),
                &math::Vec3::new(0.0, 0.0, -1.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, 0.0, 1.0),
                &math::Vec3::new(0.0, -1.0, 0.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, 0.0, -1.0),
                &math::Vec3::new(0.0, -1.0, 0.0),
            ),
        ];

        let pipeline = render::shader::Pipeline::new(
            "resources/shaders/skybox.vs",
            "resources/shaders/convertToCubeMap.fs",
        )
        .unwrap();

        glUseProgram(pipeline.id);

        pipeline.set_uniform_mat4("projection\0", &capture_projection);
        pipeline.set_uniform_1i("hdrTexture\0", 0);
        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture.id);

        glViewport(0, 0, SKYBOX_RESOLUTION, SKYBOX_RESOLUTION);
        glBindFramebuffer(GL_FRAMEBUFFER, frame_buffer);

        for index in 0..6 {
            pipeline.set_uniform_mat4("view\0", &capture_views[index]);

            let texture_target: GLenum = GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + index as u32);
            glFramebufferTexture2D(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                texture_target,
                skybox_texture,
                0,
            );
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            let model: Model = generate_cube_model();

            let mesh: &Mesh = &model.meshes[0];
            let sub_mesh: &SubMesh = &mesh.sub_meshes[0];

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

            let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
            glDrawElements(
                GL_TRIANGLES,
                sub_mesh.num_indices as i32,
                GL_UNSIGNED_INT,
                start_index as *const _,
            );
        }

        glBindFramebuffer(GL_FRAMEBUFFER, 0);
        glDeleteFramebuffers(1, &frame_buffer);
    }

    render::texture::Texture {
        id: skybox_texture,
        tex_type: render::texture::Type::Tex2D,
    }
}

fn generate_ibl_environment(
    image_path: &'static str,
) -> (
    render::texture::Texture,
    render::texture::Texture,
    render::texture::Texture,
) {
    let quad_model: Model = generate_quad_model();

    let hdr_texture = render::texture::Texture::new(image_path);
    let mut irradiance_texture = render::texture::Texture {
        id: 0,
        tex_type: render::texture::Type::Tex2D,
    };

    let mut prefilter_texture = render::texture::Texture {
        id: 0,
        tex_type: render::texture::Type::Tex2D,
    };

    let mut capture_fbo: u32 = 0;
    let mut capture_rbo: u32 = 0;

    let skybox_texture = generate_skybox_texture(hdr_texture);
    unsafe {
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LEQUAL);
        glEnable(GL_TEXTURE_CUBE_MAP_SEAMLESS);

        glGenRenderbuffers(1, &mut capture_rbo);
        glGenFramebuffers(1, &mut capture_fbo);

        let angle: f32 = 90.0;
        let capture_projection: math::Mat4 =
            math::shared::perspective(angle.to_radians(), 1.0, 0.1, 10.0);

        let capture_views: Vec<math::Mat4> = vec![
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(1.0, 0.0, 0.0),
                &math::Vec3::new(0.0, 1.0, 0.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(-1.0, 0.0, 0.0),
                &math::Vec3::new(0.0, -1.0, 0.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, 1.0, 0.0),
                &math::Vec3::new(0.0, 0.0, 1.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, -1.0, 0.0),
                &math::Vec3::new(0.0, 0.0, -1.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, 0.0, 1.0),
                &math::Vec3::new(0.0, -1.0, 0.0),
            ),
            math::shared::look_at(
                &math::Point3::new(0.0, 0.0, 0.0),
                &math::Point3::new(0.0, 0.0, -1.0),
                &math::Vec3::new(0.0, -1.0, 0.0),
            ),
        ];

        glBindTexture(GL_TEXTURE_CUBE_MAP, skybox_texture.id);
        glGenerateMipmap(GL_TEXTURE_CUBE_MAP);

        glGenTextures(1, &mut irradiance_texture.id);
        glBindTexture(GL_TEXTURE_CUBE_MAP, irradiance_texture.id);
        for index in 0..5 {
            let texture_target: GLenum = GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + index as u32);
            glTexImage2D(
                texture_target,
                0,
                GL_RGB16F.0 as i32,
                32,
                32,
                0,
                GL_RGB,
                GL_FLOAT,
                std::ptr::null(),
            );
        }

        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_WRAP_S,
            GL_CLAMP_TO_EDGE.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_WRAP_T,
            GL_CLAMP_TO_EDGE.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_WRAP_R,
            GL_CLAMP_TO_EDGE.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_MIN_FILTER,
            GL_LINEAR.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_MAG_FILTER,
            GL_LINEAR.0 as i32,
        );

        glBindFramebuffer(GL_FRAMEBUFFER, capture_fbo);
        glBindRenderbuffer(GL_RENDERBUFFER, capture_rbo);
        glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH_COMPONENT24, 32, 32);

        let irrandiance_pipeline = render::shader::Pipeline::new(
            "resources/shaders/skybox.vs",
            "resources/shaders/irradianceConvolution.fs",
        )
        .unwrap();

        glUseProgram(irrandiance_pipeline.id);

        irrandiance_pipeline.set_uniform_1i("envMap\0", 0);
        irrandiance_pipeline.set_uniform_mat4("projection\0", &capture_projection);

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_CUBE_MAP, skybox_texture.id);

        glViewport(0, 0, 32, 32);
        glBindFramebuffer(GL_FRAMEBUFFER, capture_fbo);

        let mesh: &Mesh = &quad_model.meshes[0];
        let sub_mesh: &SubMesh = &mesh.sub_meshes[0];

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

        for index in 0..5 {
            irrandiance_pipeline.set_uniform_mat4("view\0", &capture_views[index]);
            let texture_target: GLenum = GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + index as u32);

            glFramebufferTexture2D(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                texture_target,
                irradiance_texture.id,
                0,
            );

            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
            glDrawElements(
                GL_TRIANGLES,
                sub_mesh.num_indices as i32,
                GL_UNSIGNED_INT,
                start_index as *const _,
            );
        }

        glBindFramebuffer(GL_FRAMEBUFFER, 0);

        glGenTextures(1, &mut prefilter_texture.id);
        glBindTexture(GL_TEXTURE_CUBE_MAP, prefilter_texture.id);

        for index in 0..5 {
            let texture_target: GLenum = GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + index as u32);
            glTexImage2D(
                texture_target,
                0,
                GL_RGB16F.0 as i32,
                128,
                128,
                0,
                GL_RGB,
                GL_FLOAT,
                std::ptr::null(),
            );
        }

        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_WRAP_S,
            GL_CLAMP_TO_EDGE.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_WRAP_T,
            GL_CLAMP_TO_EDGE.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_WRAP_R,
            GL_CLAMP_TO_EDGE.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_MIN_FILTER,
            GL_LINEAR.0 as i32,
        );
        glTexParameteri(
            GL_TEXTURE_CUBE_MAP,
            GL_TEXTURE_MAG_FILTER,
            GL_LINEAR.0 as i32,
        );

        glGenerateMipmap(GL_TEXTURE_CUBE_MAP);

        let prefiler_pipeline = render::shader::Pipeline::new(
            "resources/shaders/skybox.vs",
            "resources/shaders/prefilterMap.fs",
        )
        .unwrap();

        glUseProgram(prefiler_pipeline.id);
        prefiler_pipeline.set_uniform_mat4("projection\0", &capture_projection);

        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_CUBE_MAP, skybox_texture.id);
        glBindFramebuffer(GL_FRAMEBUFFER, capture_fbo);

        let max_mip_levels = 5;

        for mip in 0..max_mip_levels {
            let pow = 0.5f64.powf(max_mip_levels as f64);
            let mip_width: u32 = (128.0 * pow) as u32;
            let mip_height: u32 = (128.0 * pow) as u32;

            glBindRenderbuffer(GL_RENDERBUFFER, capture_rbo);
            glRenderbufferStorage(
                GL_RENDERBUFFER,
                GL_DEPTH_COMPONENT24,
                mip_width as i32,
                mip_height as i32,
            );

            glViewport(0, 0, mip_width as i32, mip_height as i32);

            let roughness = mip as f32 / (max_mip_levels - 1) as f32;
            prefiler_pipeline.set_uniform_1f("roughness\0", roughness);

            for index in 0..6 {
                prefiler_pipeline.set_uniform_mat4("view\0", &capture_views[index]);
                let texture_target: GLenum =
                    GLenum(GL_TEXTURE_CUBE_MAP_POSITIVE_X.0 + index as u32);
                glFramebufferTexture2D(
                    GL_FRAMEBUFFER,
                    GL_COLOR_ATTACHMENT0,
                    texture_target,
                    prefilter_texture.id,
                    mip,
                );

                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

                let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
                glDrawElements(
                    GL_TRIANGLES,
                    sub_mesh.num_indices as i32,
                    GL_UNSIGNED_INT,
                    start_index as *const _,
                );
            }
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }
    }

    (skybox_texture, irradiance_texture, prefilter_texture)
}
