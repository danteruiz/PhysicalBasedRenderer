// stream.rs
//
// Created on 2021/12/19 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

const TYPE_SIZE: [usize; Type::NUM as usize] = [4, 2, 4, 1, 4, 2, 1];
const DIMENSION_SIZE: [usize; Dimension::NUM as usize] = [1, 2, 3, 4];

#[derive(Copy, Debug, Clone)]
pub enum Type {
    FLOAT = 0,
    INT8,
    INT16,
    INT32,
    UINT8,
    UINT16,
    UINT32,
    NUM,
}

#[derive(Copy, Debug, Clone)]
pub enum Usage {
    DATA = 0,
    RED,
    RG,
    RGB,
    RGBA,
}

#[derive(Copy, Debug, Clone)]
pub enum Dimension {
    SCALAR = 0,
    VEC2,
    VEC3,
    VED4,
    NUM,
}

#[derive(Copy, Debug, Clone)]
pub struct Format {
    pub dimension: Dimension,
    pub _type: Type,
    pub usage: Usage,
}

impl Format {
    pub fn new(dimension: Dimension, _type: Type, usage: Usage) -> Format {
        Format {
            dimension,
            _type,
            usage,
        }
    }

    pub fn type_size(&self) -> usize {
        TYPE_SIZE[self._type as usize]
    }
    pub fn dimension_size(&self) -> usize {
        DIMENSION_SIZE[self.dimension as usize]
    }

    pub fn stride(self) -> usize {
        self.type_size() * self.dimension_size()
    }
}

#[repr(u8)]
#[derive(Copy, Debug, Clone)]
pub enum Slot {
    Position = 0,
    Normal,
    TexCoord,
}

#[derive(Copy, Debug, Clone)]
pub struct Attribute {
    pub format: Format,
    pub slot: Slot,
    pub offset: usize,
}

impl Attribute {
    pub fn get_total_offset(self) -> usize {
        return self.offset * self.format.type_size();
    }
}
