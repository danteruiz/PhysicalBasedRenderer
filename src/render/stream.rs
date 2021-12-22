// stream.rs
//
// Created on 2021/12/19 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

const TYPE_SIZE: [usize; Type::Num as usize] = [4, 2, 4, 1, 4, 2, 1];
const DIMENSION_SIZE: [usize; Dimension::Num as usize] = [1, 2, 3, 4];

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Debug, Clone)]
pub enum Type {
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
pub enum Dimension {
    Scalar = 0,
    Vec2,
    Vec3,
    Vec4,
    Num,
}

#[derive(Copy, Debug, Clone)]
pub struct Format {
    pub dimension: Dimension,
    pub m_type: Type,
}

impl Format {
    pub fn get_type_size(&self) -> usize {
        TYPE_SIZE[self.m_type as usize]
    }
    pub fn get_dimension_size(&self) -> usize {
        DIMENSION_SIZE[self.dimension as usize]
    }

    pub fn get_stride(self) -> usize {
        self.get_type_size() * self.get_dimension_size()
    }
}

#[repr(u8)]
#[derive(Copy, Debug, Clone)]
pub enum Slot {
    Position = 0,
    Normal,
    TexCoord,
    Color,
}

#[derive(Copy, Debug, Clone)]
pub struct Attribute {
    pub format: Format,
    pub slot: Slot,
    pub offset: usize,
}

impl Attribute {
    pub fn get_total_offset(self) -> usize {
        return self.offset * self.format.get_type_size();
    }
}
