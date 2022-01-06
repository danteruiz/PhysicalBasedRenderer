// main.rs
//
// Created on 2021/09/24 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

extern crate egui;
extern crate gl;
extern crate glfw;
extern crate gltf;

mod math;
mod render;

use glfw::Context;

use std::sync::mpsc::Receiver;

//use std::sync::mpsc::Receiver;

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
    TexCoord,
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
        slot: Slot::TexCoord,
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

static mut LIGHT: Light = Light {
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

static mut MATERIAL: Material = Material {
    color: math::Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
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
        gl::DepthMask(gl::FALSE as u8);
    }

    unsafe {
        gl::UseProgram(pipeline.id);
    }
    pipeline.set_uniform_mat4("projection\0", &projection);
    let new_view = math::Mat4::from(math::Mat3::from(view));
    pipeline.set_uniform_mat4("view\0", &new_view); //&math::Mat4::from(math::Mat3::from(view)));

    let mesh = &model.meshes[0];
    let sub_mesh = &mesh.sub_meshes[0];

    unsafe {
        pipeline.set_uniform_1i("skybox\0", 0);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.gl_buffer_id);

        for attribute in &mesh.attributes {
            let format: &Format = &attribute.format;
            gl::VertexAttribPointer(
                attribute.slot as u32,
                format.get_dimension_size() as i32,
                gl::FLOAT,
                0,
                format.get_stride() as i32,
                attribute.get_total_offset() as *const _,
            );
            gl::EnableVertexAttribArray(attribute.slot as u32);
        }
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.gl_index_id);

        let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
        gl::DrawElements(
            gl::TRIANGLES,
            sub_mesh.num_indices as i32,
            gl::UNSIGNED_INT,
            start_index as *const _,
        );

        gl::DepthMask(gl::TRUE as u8);
    }
}

fn render_model(
    model: &Model,
    projection: math::Mat4,
    view: math::Mat4,
    pipeline: &render::shader::Pipeline,
    texture_cache: &render::texture::TextureCache,
    skybox: &render::skybox::Skybox,
) {
    for mesh in &model.meshes {
        let model_matrix = math::Mat4::identity();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.gl_buffer_id);

            for attribute in &mesh.attributes {
                let format: &Format = &attribute.format;
                gl::VertexAttribPointer(
                    attribute.slot as u32,
                    format.get_dimension_size() as i32,
                    gl::FLOAT,
                    0,
                    format.get_stride() as i32,
                    attribute.get_total_offset() as *const _,
                );
                gl::EnableVertexAttribArray(attribute.slot as u32);
            }
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.gl_index_id);
        }

        for sub_mesh in &mesh.sub_meshes {
            unsafe {
                gl::UseProgram(pipeline.id);

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
                pipeline.set_uniform_1i("u_albedoMap\0", 0);
                pipeline.set_uniform_1i("u_normalMap\0", 1);
                pipeline.set_uniform_1i("u_metallicMap\0", 2);
                pipeline.set_uniform_1i("u_brdfMap\0", 3);
                pipeline.set_uniform_1i("u_irradianceMap\0", 4);
                pipeline.set_uniform_1i("u_prefilterMap\0", 5);

                enable_texture(0, texture_cache.white_texture.id);
                //enable_texture(1, texture_cache.blue_texture.id);
                enable_texture(2, texture_cache.gray_texture.id);
                enable_texture(3, skybox.brdf.id);
                enable_cube_texture(4, skybox.irradiance.id);
                enable_cube_texture(5, skybox.prefilter.id);

                let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
                gl::DrawElements(
                    gl::TRIANGLES,
                    sub_mesh.num_indices as i32,
                    gl::UNSIGNED_INT,
                    start_index as *const _,
                );
            }
        }
    }
}

fn enable_cube_texture(slot: u32, texture_id: u32) {
    let texture_slot = gl::TEXTURE0 + slot;
    unsafe {
        gl::ActiveTexture(texture_slot);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);
    }
}
fn enable_texture(slot: u32, texture_id: u32) {
    let texture_slot = gl::TEXTURE0 + slot;
    unsafe {
        gl::ActiveTexture(texture_slot);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
    }
}

fn process_events(
    glfw: &mut glfw::Glfw,
    window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
) -> egui::RawInput {
    glfw.poll_events();
    let mut raw_input = egui::RawInput::default();

    let mouse_position = window.get_cursor_pos();
    let mouse_primary_action = window.get_mouse_button(glfw::MouseButtonLeft);

    let is_mouse_button_pressed: bool = match mouse_primary_action {
        glfw::Action::Release => false,
        glfw::Action::Press => true,
        glfw::Action::Repeat => true,
    };

    let egui_position = egui::Pos2 {
        x: mouse_position.0 as f32,
        y: mouse_position.1 as f32,
    };

    let egui_modifiers = egui::Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    };

    let egui_mouse_pointer_action = egui::Event::PointerButton {
        pos: egui_position,
        button: egui::PointerButton::Primary,
        pressed: is_mouse_button_pressed,
        modifiers: egui_modifiers,
    };

    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }

    raw_input.modifiers = egui_modifiers;
    raw_input.events.push(egui_mouse_pointer_action);
    raw_input
}

fn main() {
    let mut egui_context = egui::CtxRef::default();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(700, 700, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut vao: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
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

    let texture_cache = render::texture::TextureCache::new();
    let _egui_painter = render::egui_painter::EguiPainter::new();
    egui_context.set_visuals(egui::Visuals::light());

    let ibl_textures = generate_ibl_environment("resources/images/IBL/PaperMill/PaperMill.hdr");
    while !window.should_close() {
        // process input
        let raw_input = process_events(&mut glfw, &mut window, &events);

        egui_context.begin_frame(raw_input);
        egui::Window::new("").show(&egui_context, |ui| {
            ui.label("Lighting");
            ui.separator();
            unsafe {
                ui.add(egui::Slider::new(&mut LIGHT.position.x, -20.0..=20.0).text(": x"));
                ui.add(egui::Slider::new(&mut LIGHT.position.y, -20.0..=20.0).text(": y"));
                ui.add(egui::Slider::new(&mut LIGHT.position.z, -20.0..=20.0).text(": z"));
            }

            ui.label("Material");
            ui.separator();

            unsafe {
                ui.add(egui::Slider::new(&mut MATERIAL.roughness, 0.001..=1.0).text("roughness"));
                ui.add(egui::Slider::new(&mut MATERIAL.metallic, 0.001..=1.0).text("metallic"));
                //ui.add(egui::Slider::new(&mut LIGHT.position.z, 0.001..=1.0));
            }

            ui.label("color");
            ui.separator();

            unsafe {
                let mut color: [f32; 3] = [MATERIAL.color.x, MATERIAL.color.y, MATERIAL.color.z];
                ui.color_edit_button_rgb(&mut color);

                MATERIAL.color = math::Vec3::from(color);
            }
        });
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::PROGRAM_POINT_SIZE);
            gl::Enable(gl::LINE_SMOOTH);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            let window_size = window.get_size();
            let window_width = window_size.0;
            let window_height = window_size.1;
            let angle: f32 = 90.0;
            let projection = math::shared::perspective(
                angle.to_radians(),
                (window_width as f32 / window_height as f32) as f32,
                0.3,
                700.0,
            );

            gl::Viewport(0, 0, window_width as i32, window_height as i32);

            render_skybox(
                &cube_model,
                projection,
                view,
                &skybox_pipeline,
                &ibl_textures.skybox,
            );
            render_model(
                &sphere_model,
                projection,
                view,
                &pipeline,
                &texture_cache,
                &ibl_textures,
            );

            let (_, shapes) = egui_context.end_frame();
            let clipped_meshes = egui_context.tessellate(shapes);

            _egui_painter.paint(
                &clipped_meshes,
                &egui_context.texture(),
                &math::Vec2::new(window_size.0 as f32, window_size.1 as f32),
            );

            window.swap_buffers();
        }
    }
}

static SKYBOX_RESOLUTION: i32 = 1080;
fn generate_skybox_texture(texture: render::texture::Texture) -> render::texture::Texture {
    let mut frame_buffer: u32 = 0;
    let mut skybox_texture: u32 = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut frame_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, 0);

        gl::GenTextures(1, &mut skybox_texture);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture);

        for index in 0..6 {
            let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index;
            gl::TexImage2D(
                texture_target,
                0,
                gl::RGB32F as i32,
                SKYBOX_RESOLUTION,
                SKYBOX_RESOLUTION,
                0,
                gl::RGB,
                gl::FLOAT,
                std::ptr::null(),
            );

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
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

        gl::UseProgram(pipeline.id);

        pipeline.set_uniform_mat4("projection\0", &capture_projection);
        pipeline.set_uniform_1i("hdrTexture\0", 0);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture.id);

        gl::Viewport(0, 0, SKYBOX_RESOLUTION, SKYBOX_RESOLUTION);
        gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);

        for index in 0..6 {
            pipeline.set_uniform_mat4("view\0", &capture_views[index]);

            let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32;
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                texture_target,
                skybox_texture,
                0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let model: Model = generate_cube_model();

            let mesh: &Mesh = &model.meshes[0];
            let sub_mesh: &SubMesh = &mesh.sub_meshes[0];

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.gl_buffer_id);

            for attribute in &mesh.attributes {
                let format: &Format = &attribute.format;
                gl::VertexAttribPointer(
                    attribute.slot as u32,
                    format.get_dimension_size() as i32,
                    gl::FLOAT,
                    0,
                    format.get_stride() as i32,
                    attribute.get_total_offset() as *const _,
                );
                gl::EnableVertexAttribArray(attribute.slot as u32);
            }
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.gl_index_id);

            let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
            gl::DrawElements(
                gl::TRIANGLES,
                sub_mesh.num_indices as i32,
                gl::UNSIGNED_INT,
                start_index as *const _,
            );
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::DeleteFramebuffers(1, &frame_buffer);
    }

    render::texture::Texture {
        id: skybox_texture,
        tex_type: render::texture::Type::Tex2D,
    }
}

fn generate_ibl_environment(image_path: &'static str) -> render::skybox::Skybox {
    let quad_model: Model = generate_quad_model();
    let cube_model = generate_cube_model();

    let hdr_texture = render::texture::Texture::new(image_path);
    let mut irradiance_texture = render::texture::Texture {
        id: 0,
        tex_type: render::texture::Type::Tex2D,
    };

    let mut prefilter_texture = render::texture::Texture {
        id: 0,
        tex_type: render::texture::Type::Tex2D,
    };

    let mut brdf_texture = render::texture::Texture {
        id: 0,
        tex_type: render::texture::Type::Tex2D,
    };

    let mut capture_fbo: u32 = 0;
    let mut capture_rbo: u32 = 0;

    let skybox_texture = generate_skybox_texture(hdr_texture);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        gl::Enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);

        gl::GenRenderbuffers(1, &mut capture_rbo);
        gl::GenFramebuffers(1, &mut capture_fbo);

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

        // irradiance map
        {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);
            gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);

            gl::GenTextures(1, &mut irradiance_texture.id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, irradiance_texture.id);
            for index in 0..6 {
                let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32;
                gl::TexImage2D(
                    texture_target,
                    0,
                    gl::RGB16F as i32,
                    32,
                    32,
                    0,
                    gl::RGB,
                    gl::FLOAT,
                    std::ptr::null(),
                );
            }

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            gl::BindFramebuffer(gl::FRAMEBUFFER, capture_fbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, capture_rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, 32, 32);

            let irrandiance_pipeline = render::shader::Pipeline::new(
                "resources/shaders/skybox.vs",
                "resources/shaders/irradianceConvolution.fs",
            )
            .unwrap();

            gl::UseProgram(irrandiance_pipeline.id);

            irrandiance_pipeline.set_uniform_1i("envMap\0", 0);
            irrandiance_pipeline.set_uniform_mat4("projection\0", &capture_projection);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);

            gl::Viewport(0, 0, 32, 32);
            gl::BindFramebuffer(gl::FRAMEBUFFER, capture_fbo);

            let mesh: &Mesh = &cube_model.meshes[0];
            let sub_mesh: &SubMesh = &mesh.sub_meshes[0];

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.gl_buffer_id);

            for attribute in &mesh.attributes {
                let format: &Format = &attribute.format;
                gl::VertexAttribPointer(
                    attribute.slot as u32,
                    format.get_dimension_size() as i32,
                    gl::FLOAT,
                    0,
                    format.get_stride() as i32,
                    attribute.get_total_offset() as *const _,
                );
                gl::EnableVertexAttribArray(attribute.slot as u32);
            }
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.gl_index_id);

            for index in 0..6 {
                irrandiance_pipeline.set_uniform_mat4("view\0", &capture_views[index]);
                let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32;

                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0,
                    texture_target,
                    irradiance_texture.id,
                    0,
                );

                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
                gl::DrawElements(
                    gl::TRIANGLES,
                    sub_mesh.num_indices as i32,
                    gl::UNSIGNED_INT,
                    start_index as *const _,
                );
            }
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        // prefilter
        {
            gl::GenTextures(1, &mut prefilter_texture.id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, prefilter_texture.id);

            for index in 0..6 {
                let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index;
                gl::TexImage2D(
                    texture_target,
                    0,
                    gl::RGB16F as i32,
                    128,
                    128,
                    0,
                    gl::RGB,
                    gl::FLOAT,
                    std::ptr::null(),
                );
            }

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );

            gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);

            let prefiler_pipeline = render::shader::Pipeline::new(
                "resources/shaders/skybox.vs",
                "resources/shaders/prefilterMap.fs",
            )
            .unwrap();

            gl::UseProgram(prefiler_pipeline.id);
            prefiler_pipeline.set_uniform_mat4("projection\0", &capture_projection);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, capture_fbo);

            let max_mip_levels = 5;

            let mesh: &Mesh = &cube_model.meshes[0];
            let sub_mesh: &SubMesh = &mesh.sub_meshes[0];

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.gl_buffer_id);

            for attribute in &mesh.attributes {
                let format: &Format = &attribute.format;
                gl::VertexAttribPointer(
                    attribute.slot as u32,
                    format.get_dimension_size() as i32,
                    gl::FLOAT,
                    0,
                    format.get_stride() as i32,
                    attribute.get_total_offset() as *const _,
                );
                gl::EnableVertexAttribArray(attribute.slot as u32);
            }
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.gl_index_id);
            for mip in 0..max_mip_levels {
                let pow = 0.5f64.powf(mip as f64);
                let mip_width: u32 = (128.0 * pow) as u32;
                let mip_height: u32 = (128.0 * pow) as u32;

                gl::BindRenderbuffer(gl::RENDERBUFFER, capture_rbo);
                gl::RenderbufferStorage(
                    gl::RENDERBUFFER,
                    gl::DEPTH_COMPONENT24,
                    mip_width as i32,
                    mip_height as i32,
                );

                gl::Viewport(0, 0, mip_width as i32, mip_height as i32);

                let roughness = mip as f32 / (max_mip_levels - 1) as f32;
                prefiler_pipeline.set_uniform_1f("roughness\0", roughness);

                for index in 0..6 {
                    prefiler_pipeline.set_uniform_mat4("view\0", &capture_views[index]);
                    let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32;
                    gl::FramebufferTexture2D(
                        gl::FRAMEBUFFER,
                        gl::COLOR_ATTACHMENT0,
                        texture_target,
                        prefilter_texture.id,
                        mip,
                    );

                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
                    gl::DrawElements(
                        gl::TRIANGLES,
                        sub_mesh.num_indices as i32,
                        gl::UNSIGNED_INT,
                        start_index as *const _,
                    );
                }
            }
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        // BRDF
        {
            gl::GenTextures(1, &mut brdf_texture.id);
            gl::BindTexture(gl::TEXTURE_2D, brdf_texture.id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB16F as i32,
                1080,
                1080,
                0,
                gl::RG,
                gl::FLOAT,
                std::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindFramebuffer(gl::FRAMEBUFFER, capture_fbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, capture_rbo);

            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, 512, 512);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                brdf_texture.id,
                0,
            );
            gl::Viewport(0, 0, 1080, 1080);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let brdf_pipeline = render::shader::Pipeline::new(
                "resources/shaders/brdf.vs",
                "resources/shaders/brdf.fs",
            )
            .unwrap();

            gl::UseProgram(brdf_pipeline.id);
            let mesh: &Mesh = &quad_model.meshes[0];
            let sub_mesh: &SubMesh = &mesh.sub_meshes[0];

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.gl_buffer_id);

            for attribute in &mesh.attributes {
                let format: &Format = &attribute.format;
                gl::VertexAttribPointer(
                    attribute.slot as u32,
                    format.get_dimension_size() as i32,
                    gl::FLOAT,
                    0,
                    format.get_stride() as i32,
                    attribute.get_total_offset() as *const _,
                );
                gl::EnableVertexAttribArray(attribute.slot as u32);
            }

            let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
            gl::DrawElements(
                gl::TRIANGLES,
                sub_mesh.num_indices as i32,
                gl::UNSIGNED_INT,
                start_index as *const _,
            );
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    render::skybox::Skybox {
        skybox: skybox_texture,
        irradiance: irradiance_texture,
        prefilter: prefilter_texture,
        brdf: brdf_texture,
    }
}
