// app.rs
//
// Created on 2022/01/06 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use egui;
use gl;
use glfw;
use glfw::Context;

use crate::clock;
use crate::math;
use crate::render;
use crate::ui;

struct Entity<'r> {
    transform: math::Transform,
    model: &'r render::Model,
}

struct RenderArgs<'e> {
    entities: &'e Vec<Entity<'e>>,
    view_matrix: &'e math::Mat4,
    projection_matrix: &'e math::Mat4,
}

struct RotatingCamera {
    yaw_offset: f32,
    position: math::Point3,
    orientation: math::Quat,
    fov: f32,
}

impl RotatingCamera {
    fn new() -> RotatingCamera {
        RotatingCamera {
            yaw_offset: 0.0,
            position: math::Point3::new(0.0, 0.0, 2.0),
            orientation: math::Quat::identity(),
            fov: 90.0,
        }
    }

    fn projection_matrix(&self, width: f32, height: f32, near: f32, far: f32) -> math::Mat4 {
        let apect_ratio = width / height;
        math::shared::perspective(self.fov.to_radians(), apect_ratio, near, far)
    }

    fn view_matrix(&self, target_position: &math::Point3) -> math::Mat4 {
        math::shared::look_at(&self.position, &target_position, &math::shared::UNIT_Y)
    }

    fn rotate_around_position(
        &mut self,
        position: &math::Point3,
        yaw_speed: f32,
        distance: f32,
        delta_time: f32,
    ) {
        self.yaw_offset += yaw_speed * delta_time;
        self.orientation =
            math::Quat::from(math::Vec3::new(0.0, self.yaw_offset, 0.0).to_radians());
        self.position = position + ((self.orientation * math::shared::UNIT_Z) * distance);
    }
}
pub struct App {
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    model_cache: render::ModelCache,
    texture_cache: render::texture::TextureCache,
    debug_ui: ui::Ui,
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
    metallic: 0.0,
    ao: 1.0,
};

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

    pub fn run(&mut self) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::PROGRAM_POINT_SIZE);
            gl::Enable(gl::LINE_SMOOTH);
        }

        let skybox = render::skybox::Skybox::new(
            "resources/images/IBL/PaperMill/PaperMill.hdr",
            &mut self.model_cache,
        );
        let skybox_pipeline = render::shader::Pipeline::new(
            "resources/shaders/skybox.vs",
            "resources/shaders/skybox.fs",
        )
        .unwrap();

        let fragment_shader_file: &'static str = "resources/shaders/pbr.fs";
        let vertex_shader_file: &'static str = "resources/shaders/pbr.vs";
        let pipeline =
            render::shader::Pipeline::new(vertex_shader_file, fragment_shader_file).unwrap();

        let target_position = math::Point3::new(0.0, 0.0, 0.0);

        let mut clock = clock::Clock::new();
        let mut camera = RotatingCamera::new();
        self.window.make_current();
        self.window.set_key_polling(true);

        let sphere_model = self.model_cache.shape(&render::model::Shape::Sphere);

        let entity = Entity {
            transform: math::Transform::default(),
            model: sphere_model,
        };

        let entites = vec![entity];

        let distance = 0.0;
        while !self.window.should_close() {
            let delta_time = clock.delta_time();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            }
            let raw_input = self.process_events();

            self.debug_ui.update(raw_input);
            camera.rotate_around_position(
                &target_position,
                math::shared::TWO_PI,
                -distance,
                delta_time,
            );
            let window_size = self.window.get_size();
            let window_width = window_size.0;
            let window_height = window_size.1;

            let view = camera.view_matrix(&target_position);
            let projection =
                camera.projection_matrix(window_width as f32, window_height as f32, 0.3, 700.0);

            unsafe {
                gl::Viewport(0, 0, window_width as i32, window_height as i32);
            }
            render_skybox(
                self.model_cache.shape(&render::model::Shape::Cube),
                projection,
                view,
                &skybox_pipeline,
                skybox.skybox.as_ref(),
            );

            let render_args = RenderArgs {
                entities: &entites,
                view_matrix: &view,
                projection_matrix: &&projection,
            };
            render_model(
                &render_args,
                &pipeline,
                &self.texture_cache,
                &camera,
                &skybox,
            );

            self.debug_ui
                .render(window_width as f32, window_height as f32);
            self.window.swap_buffers();
        }
    }

    pub fn process_events(&mut self) -> egui::RawInput {
        self.glfw.poll_events();
        let mut raw_input = egui::RawInput::default();

        let mouse_position = self.window.get_cursor_pos();
        let mouse_primary_action = self.window.get_mouse_button(glfw::MouseButtonLeft);

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

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }

        raw_input.modifiers = egui_modifiers;
        raw_input.events.push(egui_mouse_pointer_action);
        raw_input
    }
}

fn render_skybox(
    model: &render::model::Model,
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
    pipeline.set_uniform_mat4("view\0", &new_view);

    let mesh = &model.meshes[0];
    let sub_mesh = &mesh.sub_meshes[0];

    unsafe {
        pipeline.set_uniform_1i("skybox\0", 0);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.buffer_id);

        for attribute in &mesh.attributes {
            let format = &attribute.format;
            gl::VertexAttribPointer(
                attribute.slot as u32,
                format.dimension_size() as i32,
                gl::FLOAT,
                0,
                format.stride() as i32,
                attribute.get_total_offset() as *const _,
            );
            gl::EnableVertexAttribArray(attribute.slot as u32);
        }
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.index_id);

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
    camera: &RotatingCamera,
    skybox: &render::skybox::Skybox,
) {
    for entity in render_args.entities {
        let model = &entity.model;
        let model_matrix = &entity.transform.matrix();
        for mesh in model.meshes {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, mesh.buffer_id);

                for attribute in &mesh.attributes {
                    let format = &attribute.format;
                    gl::VertexAttribPointer(
                        attribute.slot as u32,
                        format.dimension_size() as i32,
                        gl::FLOAT,
                        0,
                        format.stride() as i32,
                        attribute.get_total_offset() as *const _,
                    );
                    gl::EnableVertexAttribArray(attribute.slot as u32);
                }
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.index_id);
            }

            for sub_mesh in &mesh.sub_meshes {
                unsafe {
                    gl::UseProgram(pipeline.id);

                    let camera_position = &camera.position;
                    pipeline.set_uniform_mat4("model\0", &model_matrix);
                    pipeline.set_uniform_mat4("projection\0", &render_args.projection_matrix);
                    pipeline.set_uniform_mat4("view\0", &render_args.view_matrix);

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

                    enable_texture(gl::TEXTURE_2D, 0, texture_cache.white_texture.id);
                    //enable_texture(1, texture_cache.blue_texture.id);
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
