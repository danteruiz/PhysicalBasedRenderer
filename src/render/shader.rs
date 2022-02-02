// shader.rs
//
// Created on 2021/10/21 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/
use gl;

use std::ffi::CString;
use std::fs;
use std::ops::Drop;

use crate::iml;
static SHADER_BASE_PATH: &'static str = "resources/shaders/";

pub struct Pipeline {
    pub id: u32,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Pipeline {
    pub fn set_uniform_mat4(&self, name: &str, matrix: &iml::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const _),
                1,
                0,
                matrix.as_ptr(),
            );
        }
    }

    pub fn set_uniform_vec2(&self, name: &str, vec: &iml::Vec2) {
        unsafe {
            gl::Uniform2fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const _),
                1,
                vec.as_ptr(),
            );
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, vec: &iml::Vec3) {
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const _),
                1,
                vec.as_ptr(),
            );
        }
    }

    pub fn set_uniform_point3(&self, name: &str, p: &iml::Point3) {
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const _),
                1,
                p.as_ptr() as *const _,
            );
        }
    }

    pub fn set_uniform_1f(&self, name: &str, f: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const _),
                f,
            );
        }
    }

    pub fn set_uniform_1i(&self, name: &str, i: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const _),
                i,
            );
        }
    }
}
fn parse_shader_file(shader_file: &str) -> String {
    let file = fs::read_to_string(shader_file).unwrap();

    let mut result: String = String::new();
    for line in file.lines() {
        let mut buf = String::from(line);
        if buf.contains("#include") {
            buf.replace_range(0..8, SHADER_BASE_PATH);
            buf.retain(|c| !c.is_whitespace());
            let include_source = parse_shader_file(&buf);
            result.push_str(&include_source);
        } else {
            result.push_str(&buf);
            result.push_str("\n");
        }
    }
    result
}

fn compile_shader(shader_type: gl::types::GLenum, source: &String) -> Result<u32, String> {
    let program_object: u32 = unsafe { gl::CreateShader(shader_type) };

    if program_object == 0 {
        return Err(String::from("Failed to create shader object"));
    }

    let mut compiled: i32 = 1;
    let info_log: CString;

    unsafe {
        gl::ShaderSource(
            program_object,
            1,
            &source.as_ptr().cast(),
            &(source.len() as i32),
        );

        gl::CompileShader(program_object);
        gl::GetShaderiv(program_object, gl::COMPILE_STATUS, &mut compiled);

        let mut info_length: i32 = 0;
        let mut p: i32 = 0;
        gl::GetShaderiv(program_object, gl::INFO_LOG_LENGTH, &mut info_length);

        let mut buffer: Vec<u8> = Vec::with_capacity(info_length as usize + 1);

        buffer.extend([b' '].iter().cycle().take(info_length as usize));
        info_log = CString::from_vec_unchecked(buffer);
        gl::GetShaderInfoLog(
            program_object,
            info_length,
            &mut p,
            info_log.as_ptr() as *mut _,
        );
    }

    if compiled == 0 {
        return Err(info_log.to_string_lossy().into_owned());
    }

    Ok(program_object)
}

fn build_program(shaders: Vec<u32>) -> Result<u32, String> {
    let program = unsafe { gl::CreateProgram() };

    let mut linked = 0;
    unsafe {
        for shader in shaders {
            gl::AttachShader(program, shader);
        }

        gl::LinkProgram(program);

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut linked);
    };

    if linked == 0 {
        return Err(String::from("Failed to link shader program"));
    }
    Ok(program)
}

impl Pipeline {
    pub fn new(
        vertex_shader: &'static str,
        fragment_shader: &'static str,
    ) -> Result<Pipeline, String> {
        let vs_source: String = parse_shader_file(vertex_shader);
        let vs_shader = compile_shader(gl::VERTEX_SHADER, &vs_source)?;

        let fs_source: String = parse_shader_file(fragment_shader);
        let fs_shader = compile_shader(gl::FRAGMENT_SHADER, &fs_source)?;

        let program_result = build_program(vec![vs_shader, fs_shader]);

        match program_result {
            Err(error) => return Err(error),
            Ok(program) => return Ok(Pipeline { id: program }),
        }
    }
}
