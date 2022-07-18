// material.rs
//
// Created on 2022/07/16 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use iml;

use super::texture::TexturePointer;
pub struct Material {
    pub albedo_map: Option<TexturePointer>,
    pub normal_map: Option<TexturePointer>,
    pub specular_map: Option<TexturePointer>,
    pub emissive_map: Option<TexturePointer>,
    pub color: iml::Vec3,
    pub roughness: f32,
    pub metallic: f32,
    pub ao: f32,
}

impl Material {
    pub fn new(color: iml::Vec3, roughness: f32, metallic: f32, ao: f32) -> Material {
        Material {
            albedo_map: None,
            normal_map: None,
            specular_map: None,
            emissive_map: None,
            color,
            roughness,
            metallic,
            ao,
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material {
            albedo_map: None,
            normal_map: None,
            specular_map: None,
            emissive_map: None,
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
