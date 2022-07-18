// model.rs
//
// Created on 2022/01/07 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::cell::RefCell;
use std::collections::HashMap;

use super::{
    buffer,
    material::Material,
    stream::{Attribute, Dimension, Format, Slot, Type, Usage},
    texture,
};

use gltf;
use iml;

#[derive(Hash, PartialEq, Eq)]
pub enum Shape {
    Cube,
    Sphere,
    Quad,
}

pub struct SubMesh {
    pub start_index: usize,
    pub num_indices: usize,
    pub material_index: usize,
}

impl SubMesh {
    fn new(start_index: usize, num_indices: usize, material_index: usize) -> SubMesh {
        SubMesh {
            start_index,
            num_indices,
            material_index,
        }
    }
}

pub struct Mesh {
    pub matrix: iml::Mat4,
    pub sub_meshes: Vec<SubMesh>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            matrix: iml::Mat4::identity(),
            sub_meshes: Vec::new(),
        }
    }
}

pub struct Model {
    pub index_buffer: buffer::Buffer,
    pub vertex_buffer: buffer::Buffer,
    pub attributes: Vec<Attribute>,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            index_buffer: buffer::Buffer::default(),
            vertex_buffer: buffer::Buffer::default(),
            meshes: Vec::new(),
            attributes: Vec::new(),
            materials: Vec::new(),
        }
    }
}

pub type ModelPointer = RefCell<Model>;
type ShapeMap = HashMap<Shape, ModelPointer>;

pub struct ModelCache {
    shape_map: HashMap<Shape, ModelPointer>,
}

impl ModelCache {
    pub fn new() -> ModelCache {
        let mut shape_map = ShapeMap::new();
        shape_map.insert(Shape::Quad, generate_quad_model());
        shape_map.insert(Shape::Cube, generate_cube_model());
        shape_map.insert(Shape::Sphere, generate_sphere_model());

        ModelCache { shape_map }
    }

    pub fn shape(&self, shape: &Shape) -> &ModelPointer {
        self.shape_map.get(shape).expect("shape not found")
    }

    pub fn get_shape(shape: Shape) -> ModelPointer {
        match shape {
            Shape::Quad => generate_quad_model(),
            Shape::Cube => generate_cube_model(),
            Shape::Sphere => generate_sphere_model(),
        }
    }
}

static PI: f32 = 3.14159265359;
static X_SEGMENTS: f32 = 64.0;
static Y_SEGMENTS: f32 = 64.0;

fn generate_quad_model() -> ModelPointer {
    let positions: [f32; 12] = [
        -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0,
    ];

    let normals: [f32; 12] = [0.0; 12];
    let tex_coords: [f32; 8] = [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0];
    let indices: [i32; 6] = [0, 1, 2, 1, 2, 3];

    let sub_mesh: SubMesh = SubMesh {
        start_index: 0,
        num_indices: 6,
        material_index: 0,
    };

    let position_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Position,
        offset: 0,
    };

    let normal_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Normal,
        offset: positions.len(),
    };

    let tex_coord_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC2, Type::FLOAT, Usage::DATA),
        slot: Slot::TexCoord,
        offset: positions.len() + normals.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions));
    buffer_data.extend_from_slice(to_byte_slice(&normals));
    buffer_data.extend_from_slice(to_byte_slice(&tex_coords));

    let vertex_buffer = buffer::Buffer::new(buffer_data);

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices));
    let index_buffer = buffer::Buffer::new(index_buffer_data);

    let mesh: Mesh = Mesh {
        matrix: iml::Mat4::identity(),
        sub_meshes: vec![sub_mesh],
    };

    ModelPointer::new(Model {
        index_buffer,
        vertex_buffer,
        meshes: vec![mesh],
        attributes: vec![position_attribute, normal_attribute, tex_coord_attribute],
        materials: vec![Material::default()],
    })
}

fn generate_cube_model() -> ModelPointer {
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

    let normals: Vec<f32> = vec![
        // right side 0 - 3
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // top side 4 - 7
        0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, // bottom side 8 - 11
        0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, // left 12 - 15
        -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // front 16 - 19
        0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, // back 20 - 23
        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
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
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Position,
        offset: 0,
    };

    let normal_attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Normal,
        offset: positions.len(),
    };

    let sub_mesh: SubMesh = SubMesh {
        start_index: 0,
        num_indices: indices.len(),
        material_index: 0,
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions));
    buffer_data.extend_from_slice(to_byte_slice(&normals));

    let vertex_buffer = buffer::Buffer::new(buffer_data);

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices));
    let index_buffer = buffer::Buffer::new(index_buffer_data);

    let mesh: Mesh = Mesh {
        // index_buffer,
        // vertex_buffer,
        matrix: iml::Mat4::identity(),
        sub_meshes: vec![sub_mesh],
        //        attributes: vec![position_attribute, normal_attribute],
    };

    ModelPointer::new(Model {
        index_buffer,
        vertex_buffer,
        meshes: vec![mesh],
        attributes: vec![position_attribute, normal_attribute],
        materials: vec![Material::default()],
    })
}

fn generate_sphere_model() -> ModelPointer {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();

    for y in 0..=Y_SEGMENTS as i32 {
        for x in 0..=X_SEGMENTS as i32 {
            let x_segment: f32 = x as f32 / X_SEGMENTS;
            let y_segment: f32 = y as f32 / Y_SEGMENTS;

            let x_pos: f32 = (x_segment * 2.0 * PI).cos() * (y_segment * PI).sin();
            let y_pos: f32 = (y_segment * PI).cos();
            let z_pos: f32 = (x_segment * 2.0 * PI).sin() * (y_segment * PI).sin();

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
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Position,
        offset: 0,
    };

    let normal_attribute: Attribute = Attribute {
        format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
        slot: Slot::Normal,
        offset: positions.len(),
    };

    let mut buffer_data: Vec<u8> = Vec::new();
    buffer_data.extend_from_slice(to_byte_slice(&positions[..]));
    buffer_data.extend_from_slice(to_byte_slice(&normals[..]));
    let vertex_buffer = buffer::Buffer::new(buffer_data);

    let mut index_buffer_data: Vec<u8> = Vec::new();
    index_buffer_data.extend_from_slice(to_byte_slice(&indices[..]));
    let index_buffer = buffer::Buffer::new(index_buffer_data);

    let mesh: Mesh = Mesh {
        matrix: iml::Mat4::identity(),
        sub_meshes: vec![sub_mesh],
    };

    ModelPointer::new(Model {
        index_buffer,
        vertex_buffer,
        meshes: vec![mesh],
        attributes: vec![position_attribute, normal_attribute],
        materials: vec![Material::default()],
    })
}

fn process_gltf_node_tree<F: FnMut(&gltf::scene::Node, iml::Mat4)>(
    node: &gltf::scene::Node,
    matrix: iml::Mat4,
    callback: &mut F,
) {
    callback(&node, matrix);
    for child in node.children() {
        println!("process child node");
        process_gltf_node_tree(&child, matrix, callback);
    }
}

fn load_gltf_texture(image_data: &gltf::image::Data) -> texture::TexturePointer {
    let format = Format::new(Dimension::VEC3, Type::UINT8, Usage::RGB);
    let texture_desc = texture::TextureDesc {
        wrap_s: texture::WrapMode::REPEAT,
        wrap_t: texture::WrapMode::REPEAT,
        min_filter: texture::Filter::LINEAR,
        mag_filter: texture::Filter::LINEAR,
    };
    texture::Texture::new(
        &image_data.pixels,
        texture_desc,
        image_data.width,
        image_data.height,
        format,
        texture::Type::Tex2D,
    )
}

fn load_gltf_material(
    gltf_material: &gltf::material::Material,
    image_data: &Vec<gltf::image::Data>,
) -> Material {
    let albedo_map: Option<texture::TexturePointer> = gltf_material
        .pbr_metallic_roughness()
        .base_color_texture()
        .map_or(None, |image_info| {
            Some(load_gltf_texture(
                &image_data[image_info.texture().source().index()],
            ))
        });

    let normal_map: Option<texture::TexturePointer> =
        gltf_material.normal_texture().map_or(None, |image_info| {
            Some(load_gltf_texture(
                &image_data[image_info.texture().source().index()],
            ))
        });

    let specular_map: Option<texture::TexturePointer> = gltf_material
        .pbr_metallic_roughness()
        .metallic_roughness_texture()
        .map_or(None, |image_info| {
            Some(load_gltf_texture(
                &image_data[image_info.texture().source().index()],
            ))
        });

    let mut emissive_map: Option<texture::TexturePointer> = None;
    if let Some(image_info) = gltf_material.emissive_texture() {
        emissive_map = Some(load_gltf_texture(
            &image_data[image_info.texture().source().index()],
        ));
    }

    let pbr_info = gltf_material.pbr_metallic_roughness();
    let base_color = pbr_info.base_color_factor();

    let mut material = Material::new(
        iml::Vec3::new(base_color[0], base_color[1], base_color[2]),
        pbr_info.roughness_factor(),
        pbr_info.metallic_factor(),
        1.0,
    );

    material.albedo_map = albedo_map;
    material.normal_map = normal_map;
    material.specular_map = specular_map;
    material.emissive_map = emissive_map;

    material
}

pub fn load_gltf_model(path: String) -> Result<ModelPointer, String> {
    let (gltf, buffers, images) = gltf::import(path).expect("failed to load scene");

    if let Some(scene) = gltf.default_scene().or_else(|| gltf.scenes().next()) {
        let mut model: Model = Model::default();

        let mut positions: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut tex_coords: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut process_node = |node: &gltf::scene::Node, transform: iml::Mat4| {
            println!("process node is being called");
            if let Some(gltf_mesh) = node.mesh() {
                let mut mesh = Mesh::default();
                for prim in gltf_mesh.primitives() {
                    let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));

                    println!("--- load gltf material");
                    let material = load_gltf_material(&prim.material(), &images);

                    let gltf_positions = if let Some(iterator) = reader.read_positions() {
                        iterator.flatten().collect::<Vec<_>>()
                    } else {
                        return;
                    };

                    let gltf_normals = if let Some(iterator) = reader.read_normals() {
                        iterator.flatten().collect::<Vec<_>>()
                    } else {
                        return;
                    };

                    let gltf_uv = if let Some(iterator) = reader.read_tex_coords(0) {
                        iterator.into_f32().flatten().collect::<Vec<_>>()
                    } else {
                        panic!("Failed to get tex coords from model");
                        //vec![0.0; gltf_positions.len()]
                    };

                    let gltf_indices = if let Some(iterator) = reader.read_indices() {
                        iterator.into_u32().collect::<Vec<_>>()
                    } else {
                        panic!("Failed to get indices from model");
                    };

                    positions.extend_from_slice(&gltf_positions[..]);
                    normals.extend_from_slice(&gltf_normals[..]);
                    tex_coords.extend_from_slice(&gltf_uv[..]);

                    let sub_mesh =
                        SubMesh::new(indices.len(), gltf_indices.len(), model.materials.len());
                    indices.extend_from_slice(&gltf_indices[..]);

                    mesh.sub_meshes.push(sub_mesh);
                    model.materials.push(material);
                }
                model.meshes.push(mesh);
            } else {
                println!("node does have mesh");
            }
        };

        let matrix = iml::Mat4::identity();
        for node in scene.nodes() {
            println!("calling root process");
            process_gltf_node_tree(&node, matrix, &mut process_node);
        }
        println!("model meshes length: {}", model.meshes.len());

        let position_attribute: Attribute = Attribute {
            format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
            slot: Slot::Position,
            offset: 0,
        };

        let normal_attribute: Attribute = Attribute {
            format: Format::new(Dimension::VEC3, Type::FLOAT, Usage::DATA),
            slot: Slot::Normal,
            offset: positions.len(),
        };

        let tex_coord_attribute = Attribute {
            format: Format::new(Dimension::VEC2, Type::FLOAT, Usage::DATA),
            slot: Slot::TexCoord,
            offset: positions.len() + normals.len(),
        };

        let mut vertex_buffer = buffer::Buffer::default();
        let vertex_buffer_data = &mut vertex_buffer.data;

        vertex_buffer_data.extend_from_slice(to_byte_slice(&positions[..]));
        vertex_buffer_data.extend_from_slice(to_byte_slice(&normals[..]));
        vertex_buffer_data.extend_from_slice(to_byte_slice(&tex_coords[..]));

        let mut index_buffer = buffer::Buffer::default();
        index_buffer
            .data
            .extend_from_slice(to_byte_slice(&indices[..]));

        model.attributes = vec![position_attribute, normal_attribute, tex_coord_attribute];
        model.vertex_buffer = vertex_buffer;
        model.index_buffer = index_buffer;
        Ok(ModelPointer::new(model))
    } else {
        Err(String::from("no default scene in gltf"))
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
