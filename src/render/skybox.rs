// skybox.rs
//
// Created on 2021/12/26 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/
use super::{backend::*, model, shader, stream, texture};
use crate::iml;

fn get_capture_views() -> Vec<iml::Mat4> {
    vec![
        iml::shared::look_at(
            &iml::Point3::new(0.0, 0.0, 0.0),
            &iml::Point3::new(1.0, 0.0, 0.0),
            &iml::Vec3::new(0.0, -1.0, 0.0),
        ),
        iml::shared::look_at(
            &iml::Point3::new(0.0, 0.0, 0.0),
            &iml::Point3::new(-1.0, 0.0, 0.0),
            &iml::Vec3::new(0.0, -1.0, 0.0),
        ),
        iml::shared::look_at(
            &iml::Point3::new(0.0, 0.0, 0.0),
            &iml::Point3::new(0.0, 1.0, 0.0),
            &iml::Vec3::new(0.0, 0.0, 1.0),
        ),
        iml::shared::look_at(
            &iml::Point3::new(0.0, 0.0, 0.0),
            &iml::Point3::new(0.0, -1.0, 0.0),
            &iml::Vec3::new(0.0, 0.0, -1.0),
        ),
        iml::shared::look_at(
            &iml::Point3::new(0.0, 0.0, 0.0),
            &iml::Point3::new(0.0, 0.0, 1.0),
            &iml::Vec3::new(0.0, -1.0, 0.0),
        ),
        iml::shared::look_at(
            &iml::Point3::new(0.0, 0.0, 0.0),
            &iml::Point3::new(0.0, 0.0, -1.0),
            &iml::Vec3::new(0.0, -1.0, 0.0),
        ),
    ]
}

pub struct Skybox {
    pub skybox: texture::TexturePointer,
    pub irradiance: texture::TexturePointer,
    pub prefilter: texture::TexturePointer,
    pub brdf: texture::TexturePointer,
}

impl Skybox {
    pub fn new(image_path: &'static str, model_cache: &mut model::ModelCache) -> Skybox {
        let data = texture::load_hdr_texture(image_path);

        let texture_format = stream::Format::new(
            stream::Dimension::VEC3,
            stream::Type::FLOAT,
            stream::Usage::RGB,
        );
        let hdr_texture = texture::Texture::new(
            &data.0,
            gl::CLAMP_TO_EDGE as i32,
            gl::LINEAR as i32,
            data.1,
            data.2,
            texture_format,
            texture::Type::Tex2D,
        );
        let skybox_texture = generate_skybox_texture(hdr_texture.as_ref(), model_cache);
        let irradiance_texture = generate_irradiance_map(&skybox_texture.as_ref(), model_cache);
        let prefilter_texture = generate_prefilter_texture(&skybox_texture.as_ref(), model_cache);
        let brdf_texture = generte_brdf_texture(model_cache);

        Skybox {
            skybox: skybox_texture,
            irradiance: irradiance_texture,
            prefilter: prefilter_texture,
            brdf: brdf_texture,
        }
    }
}

static SKYBOX_RESOLUTION: i32 = 1080;
fn generate_skybox_texture(
    hdr_texture: &texture::Texture,
    model_cache: &mut model::ModelCache,
) -> texture::TexturePointer {
    let mut skybox_id: u32 = 0;
    unsafe {
        let mut frame_buffer: u32 = 0;

        gl::GenFramebuffers(1, &mut frame_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, 0);

        gl::GenTextures(1, &mut skybox_id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_id);

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
        let capture_projection: iml::Mat4 =
            iml::shared::perspective(angle.to_radians(), 1.0, 0.1, 10.0);

        let capture_views = get_capture_views();

        let pipeline = shader::Pipeline::new(
            "resources/shaders/skybox.vs",
            "resources/shaders/convertToCubeMap.fs",
        )
        .unwrap();

        gl::UseProgram(pipeline.id);

        pipeline.set_uniform_mat4("projection\0", &capture_projection);
        pipeline.set_uniform_1i("hdrTexture\0", 0);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, hdr_texture.id);

        gl::Viewport(0, 0, SKYBOX_RESOLUTION, SKYBOX_RESOLUTION);
        gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);

        for index in 0..6 {
            pipeline.set_uniform_mat4("view\0", &capture_views[index]);

            let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32;
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                texture_target,
                skybox_id,
                0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let model = &mut model_cache.shape(&model::Shape::Cube).borrow_mut();

            let mesh = &mut model.meshes[0];
            let sub_mesh = &mesh.sub_meshes[0];

            Backend::set_vertex_buffer(&mut mesh.vertex_buffer);
            Backend::set_attributes(&mesh.attributes);
            Backend::set_index_buffer(&mut mesh.index_buffer);

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

    let skybox_texture = texture::Texture {
        id: skybox_id,
        format: stream::Format::new(
            stream::Dimension::VEC3,
            stream::Type::FLOAT,
            stream::Usage::RGB,
        ),
        width: SKYBOX_RESOLUTION as u32,
        height: SKYBOX_RESOLUTION as u32,
        wrap_mode: gl::CLAMP_TO_EDGE as i32,
        filter_mode: gl::LINEAR as i32,
        _type: texture::Type::TexCUBE,
    };

    Box::new(skybox_texture)
}

fn generate_irradiance_map(
    skybox_texture: &texture::Texture,
    model_cache: &mut model::ModelCache,
) -> texture::TexturePointer {
    let capture_views = get_capture_views();
    let mut irradiance_id: u32 = 0;
    unsafe {
        let mut capture_rbo: u32 = 0;
        let mut capture_fbo: u32 = 0;

        gl::GenRenderbuffers(1, &mut capture_rbo);
        gl::GenFramebuffers(1, &mut capture_fbo);
        let angle: f32 = 90.0;
        let capture_projection: iml::Mat4 =
            iml::shared::perspective(angle.to_radians(), 1.0, 0.1, 10.0);

        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);
        gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);

        gl::GenTextures(1, &mut irradiance_id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, irradiance_id);
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

        let irrandiance_pipeline = shader::Pipeline::new(
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

        let cube_model = &mut model_cache.shape(&model::Shape::Cube).borrow_mut();
        let mesh = &mut cube_model.meshes[0];
        let sub_mesh: &model::SubMesh = &mesh.sub_meshes[0];

        Backend::set_vertex_buffer(&mut mesh.vertex_buffer);
        Backend::set_attributes(&mesh.attributes);
        Backend::set_index_buffer(&mut mesh.index_buffer);

        for index in 0..6 {
            irrandiance_pipeline.set_uniform_mat4("view\0", &capture_views[index]);
            let texture_target = gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32;

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                texture_target,
                irradiance_id,
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

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::DeleteFramebuffers(1, &capture_fbo);
        gl::DeleteRenderbuffers(1, &capture_rbo);
    }

    let irradiance_texture = texture::Texture {
        id: irradiance_id,
        format: stream::Format::new(
            stream::Dimension::VEC3,
            stream::Type::FLOAT,
            stream::Usage::RGB,
        ),
        width: SKYBOX_RESOLUTION as u32,
        height: SKYBOX_RESOLUTION as u32,
        wrap_mode: gl::CLAMP_TO_EDGE as i32,
        filter_mode: gl::LINEAR as i32,
        _type: texture::Type::TexCUBE,
    };

    Box::new(irradiance_texture)
}

fn generate_prefilter_texture(
    skybox_texture: &texture::Texture,
    model_cache: &mut model::ModelCache,
) -> texture::TexturePointer {
    let mut prefilter_id = 0;
    let capture_views = get_capture_views();
    unsafe {
        let mut capture_rbo: u32 = 0;
        let mut capture_fbo: u32 = 0;

        gl::GenRenderbuffers(1, &mut capture_rbo);
        gl::GenFramebuffers(1, &mut capture_fbo);

        gl::GenTextures(1, &mut prefilter_id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, prefilter_id);

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

        let prefiler_pipeline = shader::Pipeline::new(
            "resources/shaders/skybox.vs",
            "resources/shaders/prefilterMap.fs",
        )
        .unwrap();

        let angle: f32 = 90.0;
        let capture_projection: iml::Mat4 =
            iml::shared::perspective(angle.to_radians(), 1.0, 0.1, 10.0);
        gl::UseProgram(prefiler_pipeline.id);
        prefiler_pipeline.set_uniform_mat4("projection\0", &capture_projection);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.id);
        gl::BindFramebuffer(gl::FRAMEBUFFER, capture_fbo);

        let max_mip_levels = 5;

        let mut cube_model = model_cache.shape(&model::Shape::Cube).borrow_mut();
        let mesh = &mut cube_model.meshes[0];
        let sub_mesh = &mesh.sub_meshes[0];

        Backend::set_vertex_buffer(&mut mesh.vertex_buffer);
        Backend::set_attributes(&mesh.attributes);
        Backend::set_index_buffer(&mut mesh.index_buffer);
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
                    prefilter_id,
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

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::DeleteFramebuffers(1, &capture_fbo);
        gl::DeleteRenderbuffers(1, &capture_rbo);
    }

    let prefilter_texture = texture::Texture {
        id: prefilter_id,
        format: stream::Format::new(
            stream::Dimension::VEC3,
            stream::Type::FLOAT,
            stream::Usage::RGB,
        ),
        width: SKYBOX_RESOLUTION as u32,
        height: SKYBOX_RESOLUTION as u32,
        wrap_mode: gl::CLAMP_TO_EDGE as i32,
        filter_mode: gl::LINEAR as i32,
        _type: texture::Type::TexCUBE,
    };

    Box::new(prefilter_texture)
}

fn generte_brdf_texture(model_cache: &mut model::ModelCache) -> texture::TexturePointer {
    let mut brdf_id: u32 = 0;
    unsafe {
        let mut capture_rbo: u32 = 0;
        let mut capture_fbo: u32 = 0;

        gl::GenRenderbuffers(1, &mut capture_rbo);
        gl::GenFramebuffers(1, &mut capture_fbo);

        gl::GenTextures(1, &mut brdf_id);
        gl::BindTexture(gl::TEXTURE_2D, brdf_id);

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
            brdf_id,
            0,
        );
        gl::Viewport(0, 0, 1080, 1080);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        let brdf_pipeline =
            shader::Pipeline::new("resources/shaders/brdf.vs", "resources/shaders/brdf.fs")
                .unwrap();

        gl::UseProgram(brdf_pipeline.id);
        let mut quad_model = model_cache.shape(&model::Shape::Quad).borrow_mut();
        let mesh = &mut quad_model.meshes[0];
        let sub_mesh: &model::SubMesh = &mesh.sub_meshes[0];

        Backend::set_vertex_buffer(&mut mesh.vertex_buffer);
        Backend::set_attributes(&mesh.attributes);
        Backend::set_index_buffer(&mut mesh.index_buffer);

        let start_index = sub_mesh.start_index * std::mem::size_of::<u32>();
        gl::DrawElements(
            gl::TRIANGLES,
            sub_mesh.num_indices as i32,
            gl::UNSIGNED_INT,
            start_index as *const _,
        );

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::DeleteFramebuffers(1, &capture_fbo);
        gl::DeleteRenderbuffers(1, &capture_rbo);
    }

    let brdf_texture = texture::Texture {
        id: brdf_id,
        format: stream::Format::new(
            stream::Dimension::VEC3,
            stream::Type::FLOAT,
            stream::Usage::RGB,
        ),
        width: SKYBOX_RESOLUTION as u32,
        height: SKYBOX_RESOLUTION as u32,
        wrap_mode: gl::CLAMP_TO_EDGE as i32,
        filter_mode: gl::LINEAR as i32,
        _type: texture::Type::TexCUBE,
    };

    Box::new(brdf_texture)
}
