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

mod clock;
mod math;

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
//
// #[repr(u8)]
// enum Topology {
//     Points,
//     Lines,
//     LineStrip,
//     Triangles,
// }
//
// #[repr(u8)]
// enum Type {
//     Float = 0,
//     Int8,
//     Int16,
//     Int32,
//     UInt8,
//     UInt16,
//     UInt32,
//     Num,
// }
//
// #[repr(u8)]
// enum Dimension {
//     Scalar = 0,
//     Vec2,
//     Vec3,
//     Vec4,
//     Num,
// }
// struct Format {
//     dimension: Dimension,
// }
//
// #[repr(u8)]
// enum Slot {
//     Position = 0,
//     Normal,
//     TexCoord,
//     Color,
// }
//
// struct Attribute {
//     format: Format,
//     slot: Slot,
//     offset: usize,
// }
//
// impl Attribute {
//     fn get_total_offset(self) -> usize {
//         return self.offset;
//     }
// }
//
// const TYPE_SIZE: [usize; Type::Num as usize] = [4, 2, 4, 1, 4, 2, 1];
// const DIMENSION_SIZE: [usize; Dimension::Num as usize] = [1, 2, 3, 4];
// struct BufferView {
//     offset: usize,
//     size: usize,
//     format: Format,
// }
//
// struct Buffer {
//     data: Vec<u8>,
//     dirty: bool,
// }
//
// struct SubMesh {
//     start_index: usize,
//     num_indices: usize,
//     material_index: usize,
// }
//
// struct Mesh {
//     sub_meshes: Vec<SubMesh>,
//     vertex_buffer: Buffer,
//     index_buffer: Buffer,
// }
//
// struct Model {
//     meshes: Vec<Mesh>,
// }
//
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

const WINDOW_TITLE: &'static str = "Physical Based Renderer";

fn main() {
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

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title(WINDOW_TITLE);

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
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

    let mat = math::Mat3::new();

    println!("{:?}", mat);
    let mut clock: clock::Clock = clock::Clock::new();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.

                unsafe {
                    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                    glClearColor(0.0, 0.0, 1.0, 1.0);
                }

                let _ = context.swap_buffers();
                //context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            }
            _ => (),
        }
    });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn vec3_dot() {
        let v1 = math::Vec3::new(3.0, -2.0, 7.0);
        let v2 = math::Vec3::new(0.0, 4.0, -1.0);

        let result = -15.0;
        assert_eq!(math::Vec3::dot(&v1, &v2), result);
    }

    #[test]
    fn vec3_cross() {
        let v1 = math::Vec3::new(1.0, 3.0, 4.0);
        let v2 = math::Vec3::new(2.0, -5.0, 8.0);

        let result = math::Vec3::new(44.0, 0.0, -11.0);
        assert_eq!(math::Vec3::cross(&v1, &v2), result);
    }

    #[test]
    fn vec3_add() {
        let v1 = math::Vec3::new(1.0, 2.0, 3.0);
        let v2 = math::Vec3::new(4.0, 5.0, 6.0);

        let result = math::Vec3::new(5.0, 7.0, 9.0);

        assert_eq!(v1 + v2, result);
    }

    #[test]
    fn vec3_sub() {
        let v1 = math::Vec3::new(1.0, 2.0, 3.0);
        let v2 = math::Vec3::new(4.0, 5.0, 6.0);

        let result = math::Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(v1 - v2, result);
    }

    #[test]
    fn vec3_add_sub() {
        let v1 = math::Vec3::new(4.0, 5.0, 6.0);
        let v2 = math::Vec3::new(7.0, -3.0, 0.0);
        let v3 = math::Vec3::new(1.0, 2.0, 3.0);

        let result = math::Vec3::new(10.0, 0.0, 3.0);
        assert_eq!(v1 + v2 - v3, result);
    }
}
