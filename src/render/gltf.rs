// gltf.rs
//
// Created on 2021/11/04 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

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
