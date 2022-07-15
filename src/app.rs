// app.rs
//
// Created on 2022/01/06 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use core::f32;
use std::convert::From;

use egui;
use gl;
use glfw;
use glfw::Context;

use crate::{clock, file_watcher, render, ui};

// pub struct AABB {
//     pub center: math::Point3,
//     pub raduis: math::Vec3,
// }
pub struct Entity<'e> {
    pub transform: iml::Transform,
    pub model: &'e render::model::ModelPointer,
    pub material: Material,
    //    pub aabb: AABB,
}

struct RenderArgs<'e> {
    entities: &'e Vec<Entity<'e>>,
    view_matrix: &'e iml::Mat4,
    projection_matrix: &'e iml::Mat4,
}

struct FPSCamera {
    position: iml::Point3,
    orientation: iml::Quat,
    fov: f32,
    yaw: f32,
    pitch: f32,
    last_cursor: iml::Vec2,
    repeat: bool,
}

impl FPSCamera {
    fn new() -> FPSCamera {
        FPSCamera {
            position: iml::Point3::new(-2.0, 5.0, -2.0),
            orientation: iml::Quat::from(iml::Vec3::new(40.0, 50.0, 0.0).to_radians()),
            fov: 90.0,
            yaw: 40.0,
            pitch: 50.0,
            last_cursor: iml::Vec2::new(0.0, 0.0),
            repeat: false,
        }
    }

    fn projection_matrix(&self, width: f32, height: f32, near: f32, far: f32) -> iml::Mat4 {
        let apect_ratio = width / height;
        iml::shared::perspective(self.fov.to_radians(), apect_ratio, near, far)
    }

    fn view_matrix(&self) -> iml::Mat4 {
        let target_position = self.position + (self.orientation * iml::shared::UNIT_Z);
        iml::shared::look_at(&self.position, &target_position, &iml::shared::UNIT_Y)
    }

    fn update(&mut self, window: &mut glfw::Window, sensitivity: f32, delta_time: f32) {
        let button = window.get_mouse_button(glfw::MouseButtonRight);

        if button == glfw::Action::Release {
            self.repeat = false
        }

        if button == glfw::Action::Press && !self.repeat {
            let mouse_position = window.get_cursor_pos();
            self.last_cursor = iml::Vec2::new(mouse_position.0 as f32, mouse_position.1 as f32);
            self.repeat = true;
        }

        if button == glfw::Action::Press {
            let mouse_position = window.get_cursor_pos();
            let y_offset = (mouse_position.1 as f32 - self.last_cursor.y) * sensitivity;
            let x_offset = (mouse_position.0 as f32 - self.last_cursor.x) * sensitivity * -1.0;

            self.last_cursor = iml::Vec2::new(mouse_position.0 as f32, mouse_position.1 as f32);
            self.yaw += x_offset * delta_time;
            self.pitch += y_offset * delta_time;

            // clamp
            if self.pitch > 89.9 {
                self.pitch = 89.9;
            } else if self.pitch < -89.9 {
                self.pitch = -89.9;
            }

            let euler_angle = iml::Vec3::new(self.pitch, self.yaw, 0.0);
            self.orientation = iml::Quat::from(euler_angle.to_radians());
        }

        let mut x_direction = 0.0;
        let mut z_direction = 0.0;

        if glfw_action_to_boolean(window.get_key(glfw::Key::D)) {
            x_direction = -1.0;
        } else if glfw_action_to_boolean(window.get_key(glfw::Key::A)) {
            x_direction = 1.0;
        }

        if glfw_action_to_boolean(window.get_key(glfw::Key::S)) {
            z_direction = -1.0;
        } else if glfw_action_to_boolean(window.get_key(glfw::Key::W)) {
            z_direction = 1.0;
        }

        let z_offset = (self.orientation * iml::shared::UNIT_Z) * z_direction * 20.0;
        let x_offset = (self.orientation * iml::shared::UNIT_X) * x_direction * 20.0;

        self.position = self.position + (z_offset + x_offset) * delta_time;
    }
}

fn glfw_action_to_boolean(action: glfw::Action) -> bool {
    match action {
        glfw::Action::Press | glfw::Action::Repeat => true,
        glfw::Action::Release => false,
    }
}

fn glfw_key_to_egui_key(key: glfw::Key) -> Option<egui::Key> {
    match key {
        glfw::Key::Backspace => Some(egui::Key::Backspace),
        glfw::Key::Enter => Some(egui::Key::Enter),
        glfw::Key::Tab => Some(egui::Key::Tab),
        glfw::Key::Space => Some(egui::Key::Space),
        glfw::Key::Escape => Some(egui::Key::Escape),
        glfw::Key::A => Some(egui::Key::A),
        glfw::Key::B => Some(egui::Key::B),
        glfw::Key::C => Some(egui::Key::C),
        glfw::Key::D => Some(egui::Key::D),
        glfw::Key::E => Some(egui::Key::E),
        glfw::Key::F => Some(egui::Key::F),
        glfw::Key::G => Some(egui::Key::G),
        glfw::Key::H => Some(egui::Key::H),
        glfw::Key::I => Some(egui::Key::I),
        glfw::Key::J => Some(egui::Key::J),
        glfw::Key::K => Some(egui::Key::K),
        glfw::Key::L => Some(egui::Key::L),
        glfw::Key::M => Some(egui::Key::M),
        glfw::Key::N => Some(egui::Key::N),
        glfw::Key::O => Some(egui::Key::O),
        glfw::Key::P => Some(egui::Key::P),
        glfw::Key::Q => Some(egui::Key::Q),
        glfw::Key::R => Some(egui::Key::R),
        glfw::Key::S => Some(egui::Key::S),
        glfw::Key::T => Some(egui::Key::T),
        glfw::Key::U => Some(egui::Key::U),
        glfw::Key::V => Some(egui::Key::V),
        glfw::Key::W => Some(egui::Key::W),
        glfw::Key::X => Some(egui::Key::X),
        glfw::Key::Y => Some(egui::Key::Y),
        glfw::Key::Z => Some(egui::Key::Z),
        glfw::Key::Num0 => Some(egui::Key::Num0),
        glfw::Key::Num1 => Some(egui::Key::Num1),
        glfw::Key::Num2 => Some(egui::Key::Num2),
        glfw::Key::Num3 => Some(egui::Key::Num3),
        glfw::Key::Num4 => Some(egui::Key::Num4),
        glfw::Key::Num5 => Some(egui::Key::Num5),
        glfw::Key::Num6 => Some(egui::Key::Num6),
        glfw::Key::Num7 => Some(egui::Key::Num7),
        glfw::Key::Num8 => Some(egui::Key::Num8),
        glfw::Key::Num9 => Some(egui::Key::Num9),
        _ => None,
    }
}

type WindowEvents = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
pub struct App {
    window: glfw::Window,
    events: WindowEvents,
    glfw: glfw::Glfw,
    model_cache: render::ModelCache,
    texture_cache: render::texture::TextureCache,
    debug_ui: ui::Ui,
}

pub struct Material {
    pub color: iml::Vec3,
    pub roughness: f32,
    pub metallic: f32,
    pub ao: f32,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: iml::Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            roughness: 1.0,
            metallic: 0.0,
            ao: 1.0,
        }
    }
}

#[repr(C)]
pub struct Light {
    pub position: iml::Vec4,
    pub color: iml::Vec4,
}

impl Light {
    pub fn new(position: iml::Vec4, color: iml::Vec4) -> Light {
        Light { position, color }
    }
}

struct LightManager {
    light_buffer: render::Buffer,
    lights: Vec<Light>,
}

impl LightManager {
    fn new() -> LightManager {
        LightManager {
            light_buffer: render::Buffer::default(),
            lights: Vec::new(),
        }
    }

    fn add(&mut self, light: Light) {
        self.lights.push(light);
    }
}

impl App {
    pub fn init(width: u32, height: u32) -> App {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, "PBR Demo", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mut vao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        let model_cache = render::ModelCache::new();
        let texture_cache = render::texture::TextureCache::new();
        App {
            window,
            events,
            glfw,
            model_cache,
            texture_cache,
            debug_ui: ui::Ui::new(),
        }
    }

    pub fn run(self) {
        let App {
            mut window,
            events,
            mut glfw,
            mut model_cache,
            texture_cache,
            mut debug_ui,
        } = self;

        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        unsafe {
            gl::DepthFunc(gl::LEQUAL);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::PROGRAM_POINT_SIZE);
            gl::Enable(gl::LINE_SMOOTH);
        }

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        let skybox = render::skybox::Skybox::new(
            "resources/images/IBL/PaperMill/PaperMill.hdr",
            &mut model_cache,
        );
        let skybox_pipeline = render::shader::Pipeline::new(
            "resources/shaders/skybox.vs",
            "resources/shaders/skybox.fs",
        )
        .unwrap();

        let mut file_wather =
            file_watcher::FileWatcher::new(String::from("resources/shaders/pbr.fs"));
        let fragment_shader_file: &'static str = "resources/shaders/pbr.fs";
        let vertex_shader_file: &'static str = "resources/shaders/pbr.vs";
        let mut pipeline =
            render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file).unwrap();

        let mut clock = clock::Clock::new();
        let mut camera = FPSCamera::new();
        window.make_current();
        window.set_key_polling(true);

        let cube_model = model_cache.shape(&render::model::Shape::Cube);

        let mut floor = Entity {
            transform: iml::Transform::default(),
            model: cube_model,
            material: Material::new(),
        };

        floor.transform.scale = iml::Vec3::new(100.0, 0.5, 100.0);
        floor.transform.translation = iml::Point3::new(0.0, -2.0, 0.0);

        let mut entities = vec![floor];

        let spacing = 3.0;
        let starting_position = iml::Point3::new(0.0, 1.0, 0.0);
        for x in 0..7 {
            let metallic = 1.0 - (x as f32 / 6.0);
            for z in 0..7 {
                let mut position = starting_position.clone();
                position.x = z as f32 * spacing;
                position.z = x as f32 * spacing;

                let model = model_cache.shape(&render::model::Shape::Sphere);
                let mut material = Material::new();
                material.color = iml::Vec3::new(1.0, 0.0, 0.0);
                material.roughness = iml::shared::clamp(z as f32 / 6.0, 0.05, 1.0);
                material.metallic = metallic;
                let transform = iml::Transform::new(position);

                entities.push(Entity {
                    transform,
                    model,
                    material,
                });
            }
        }
        let mut light_manager = LightManager::new();
        light_manager.add(Light::new(
            iml::Vec4::new(0.0, 11.0, 0.0, 300.0),
            iml::Vec4::new(1.0, 1.0, 1.0, 300.0),
        ));
        light_manager.add(Light::new(
            iml::Vec4::new(0.0, 11.0, 15.0, 300.0),
            iml::Vec4::new(1.0, 1.0, 1.0, 300.0),
        ));
        light_manager.add(Light::new(
            iml::Vec4::new(15.0, 11.0, 0.0, 300.0),
            iml::Vec4::new(1.0, 1.0, 1.0, 300.0),
        ));
        light_manager.add(Light::new(
            iml::Vec4::new(15.0, 11.0, 15.0, 300.0),
            iml::Vec4::new(1.0, 1.0, 1.0, 300.0),
        ));

        let mut light_unifrom_buffer = 0;
        if light_manager.lights.len() > 0 {
            light_manager
                .light_buffer
                .data
                .extend_from_slice(to_byte_slice(&light_manager.lights));

            let light_buffer_size =
                light_manager.light_buffer.data.len() * std::mem::size_of::<u8>();
            unsafe {
                gl::GenBuffers(1, &mut light_unifrom_buffer);
                gl::BindBuffer(gl::UNIFORM_BUFFER, light_unifrom_buffer);
                gl::BufferData(
                    gl::UNIFORM_BUFFER,
                    light_buffer_size as isize,
                    light_manager.light_buffer.data.as_ptr().cast(),
                    gl::STATIC_DRAW,
                );

                gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

                gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, light_unifrom_buffer);
            }
        }

        while !window.should_close() {
            let delta_time = clock.delta_time();

            file_wather.update(|| {
                let new_pipeline =
                    render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file);

                match new_pipeline {
                    Ok(pipeline_) => pipeline = pipeline_,
                    Err(error) => println!("failed to compile pipeline: {}", error),
                }
            });
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            }
            let raw_input = App::process_events(&mut glfw, &mut window, &events);
            //debug_ui.update(raw_input, &mut entites, &mut light_manager.lights);

            if light_manager.lights.len() > 0 {
                light_manager.light_buffer.data.clear();
                light_manager
                    .light_buffer
                    .data
                    .extend_from_slice(to_byte_slice(&light_manager.lights));

                let light_buffer_size =
                    light_manager.light_buffer.data.len() * std::mem::size_of::<u8>();
                unsafe {
                    gl::BindBuffer(gl::UNIFORM_BUFFER, light_unifrom_buffer);
                    gl::BufferData(
                        gl::UNIFORM_BUFFER,
                        light_buffer_size as isize,
                        light_manager.light_buffer.data.as_ptr().cast(),
                        gl::STATIC_DRAW,
                    );

                    gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

                    gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, light_unifrom_buffer);
                }
            }
            camera.update(&mut window, 4.0, delta_time);
            let window_size = window.get_size();
            let window_width = window_size.0;
            let window_height = window_size.1;

            let view = camera.view_matrix();
            let projection =
                camera.projection_matrix(window_width as f32, window_height as f32, 0.3, 700.0);

            unsafe {
                gl::Viewport(0, 0, window_width as i32, window_height as i32);
            }
            render_skybox(
                model_cache.shape(&render::model::Shape::Cube),
                projection,
                view,
                &skybox_pipeline,
                skybox.skybox.as_ref(),
            );

            let mut render_args = RenderArgs {
                entities: &entities,
                view_matrix: &view,
                projection_matrix: &projection,
            };
            render_model(
                &mut render_args,
                &pipeline,
                &texture_cache,
                &camera,
                &skybox,
            );

            debug_ui.update(raw_input, &mut light_manager.lights);
            debug_ui.render(window_width as f32, window_height as f32);
            window.swap_buffers();
        }
    }

    pub fn process_events(
        glfw: &mut glfw::Glfw,
        window: &mut glfw::Window,
        events: &WindowEvents,
    ) -> egui::RawInput {
        glfw.poll_events();
        let mut raw_input = egui::RawInput::default();

        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::Key(glfw_key, _, action, _) => {
                    let pressed = match action {
                        glfw::Action::Release => false,
                        glfw::Action::Press => true,
                        glfw::Action::Repeat => true,
                    };

                    match glfw_key {
                        glfw::Key::Enter | glfw::Key::Backspace => {
                            let egui_key = glfw_key_to_egui_key(glfw_key);

                            match egui_key {
                                Some(key) => {
                                    let key_action = egui::Event::Key {
                                        key,
                                        pressed,
                                        modifiers: egui::Modifiers::default(),
                                    };
                                    raw_input.events.push(key_action);
                                }
                                _ => {
                                    println!("Key is not supported: {:?}", glfw_key);
                                }
                            }
                        }
                        _ => {
                            if pressed {
                                let key_text = glfw_key.get_name();

                                match key_text {
                                    Some(text) => raw_input.events.push(egui::Event::Text(text)),
                                    _ => {
                                        if glfw_key == glfw::Key::Space {
                                            raw_input
                                                .events
                                                .push(egui::Event::Text(String::from(" ")));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let position = egui::Pos2 {
                        x: x as f32,
                        y: y as f32,
                    };
                    raw_input.events.push(egui::Event::PointerMoved(position));
                }
                glfw::WindowEvent::MouseButton(glfw_button, action, _) => {
                    let pressed = glfw_action_to_boolean(action);

                    let egui_button = match glfw_button {
                        glfw::MouseButtonLeft => Some(egui::PointerButton::Primary),
                        glfw::MouseButtonMiddle => Some(egui::PointerButton::Middle),
                        glfw::MouseButtonRight => Some(egui::PointerButton::Secondary),
                        _ => None,
                    };

                    match egui_button {
                        Some(button) => {
                            let mouse_position = window.get_cursor_pos();
                            let pos = egui::Pos2 {
                                x: mouse_position.0 as f32,
                                y: mouse_position.1 as f32,
                            };

                            let event = egui::Event::PointerButton {
                                pos,
                                button,
                                pressed,
                                modifiers: egui::Modifiers::default(),
                            };

                            raw_input.events.push(event);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        raw_input
    }
}

fn render_skybox(
    model_pointer: &render::model::ModelPointer,
    projection: iml::Mat4,
    view: iml::Mat4,
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
    let new_view = iml::Mat4::from(iml::Mat3::from(view));
    pipeline.set_uniform_mat4("view\0", &new_view);

    let mut model = model_pointer.borrow_mut();
    let mesh = &mut model.meshes[0];
    let sub_mesh = &mesh.sub_meshes[0];

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);

        render::Backend::set_vertex_buffer(&mut mesh.vertex_buffer);
        render::Backend::set_attributes(&mesh.attributes);
        render::Backend::set_index_buffer(&mut mesh.index_buffer);

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
    render_args: &RenderArgs,
    pipeline: &render::shader::Pipeline,
    texture_cache: &render::texture::TextureCache,
    camera: &FPSCamera,
    skybox: &render::skybox::Skybox,
) {
    for entity in render_args.entities {
        let mut model = entity.model.borrow_mut();
        let model_matrix = &entity.transform.matrix();
        let material = &entity.material;
        for mesh in &mut model.meshes {
            render::Backend::set_vertex_buffer(&mut mesh.vertex_buffer);
            render::Backend::set_attributes(&mesh.attributes);
            render::Backend::set_index_buffer(&mut mesh.index_buffer);

            for sub_mesh in &mesh.sub_meshes {
                unsafe {
                    gl::UseProgram(pipeline.id);

                    let camera_position = &camera.position;
                    pipeline.set_uniform_mat4("model\0", &model_matrix);
                    pipeline.set_uniform_mat4("projection\0", &render_args.projection_matrix);
                    pipeline.set_uniform_mat4("view\0", &render_args.view_matrix);

                    pipeline.set_uniform_vec3("material.color\0", &material.color);
                    pipeline.set_uniform_1f("material.roughness\0", material.roughness);
                    pipeline.set_uniform_1f("material.metallic\0", material.metallic);
                    pipeline.set_uniform_1f("material.ao\0", material.ao);
                    pipeline.set_uniform_1f("material.specular\0", material.roughness);
                    pipeline.set_uniform_point3("camera_position\0", &camera_position);
                    pipeline.set_uniform_1i("u_albedoMap\0", 0);
                    pipeline.set_uniform_1i("u_normalMap\0", 1);
                    pipeline.set_uniform_1i("u_metallicMap\0", 2);
                    pipeline.set_uniform_1i("u_brdfMap\0", 3);
                    pipeline.set_uniform_1i("u_irradianceMap\0", 4);
                    pipeline.set_uniform_1i("u_prefilterMap\0", 5);

                    enable_texture(gl::TEXTURE_2D, 0, texture_cache.white_texture.id);
                    enable_texture(gl::TEXTURE_2D, 1, texture_cache.blue_texture.id);
                    enable_texture(gl::TEXTURE_2D, 2, texture_cache.gray_texture.id);
                    enable_texture(gl::TEXTURE_2D, 3, skybox.brdf.id);
                    enable_texture(gl::TEXTURE_CUBE_MAP, 4, skybox.irradiance.id);
                    enable_texture(gl::TEXTURE_CUBE_MAP, 5, skybox.prefilter.id);

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
}

fn enable_texture(texture_type: gl::types::GLenum, slot: u32, texture_id: u32) {
    let texture_slot = gl::TEXTURE0 + slot;

    unsafe {
        gl::ActiveTexture(texture_slot);
        gl::BindTexture(texture_type, texture_id);
    }
}

fn to_byte_slice<'a, T>(floats: &'a [T]) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            floats.as_ptr() as *const _,
            floats.len() * std::mem::size_of::<T>(),
        )
    }
}
