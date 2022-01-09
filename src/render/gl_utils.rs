// gl_utils.rs
//
// Created on 2022/01/08 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use gl;

use std::convert::From;

use super::{stream, texture};

impl From<stream::Usage> for gl::types::GLenum {
    fn from(usage: stream::Usage) -> gl::types::GLenum {
        match usage {
            stream::Usage::RED => gl::RED,
            stream::Usage::RG => gl::RG,
            stream::Usage::RGB => gl::RGB,
            _ => gl::RGBA,
        }
    }
}

impl From<stream::Format> for gl::types::GLenum {
    fn from(format: stream::Format) -> gl::types::GLenum {
        match format._type {
            stream::Type::INT16 => match format.usage {
                stream::Usage::RG => gl::RG16I,
                stream::Usage::RGB => gl::RGB16I,
                stream::Usage::RGBA => gl::RGBA16I,
                _ => gl::RED,
            },
            stream::Type::INT32 => match format.usage {
                stream::Usage::RG => gl::RG32I,
                stream::Usage::RGB => gl::RGB32I,
                stream::Usage::RGBA => gl::RGBA32I,
                _ => gl::RED,
            },
            stream::Type::FLOAT => match format.usage {
                stream::Usage::RG => gl::RG32F,
                stream::Usage::RGB => gl::RGB32F,
                stream::Usage::RGBA => gl::RGBA32F,
                _ => gl::RED,
            },
            stream::Type::UINT16 => match format.usage {
                stream::Usage::RG => gl::RG16UI,
                stream::Usage::RGB => gl::RGB16UI,
                stream::Usage::RGBA => gl::RGBA16UI,
                _ => gl::RED,
            },
            stream::Type::UINT32 => match format.usage {
                stream::Usage::RG => gl::RG32UI,
                stream::Usage::RGB => gl::RGB32UI,
                stream::Usage::RGBA => gl::RGBA32UI,
                _ => gl::RED,
            },
            _ => gl::RGBA,
        }
    }
}

impl From<stream::Type> for gl::types::GLenum {
    fn from(_type: stream::Type) -> gl::types::GLenum {
        match _type {
            stream::Type::FLOAT => gl::FLOAT,
            stream::Type::INT32 => gl::INT,
            stream::Type::INT8 => gl::BYTE,
            stream::Type::UINT32 => gl::UNSIGNED_INT,
            stream::Type::UINT8 => gl::UNSIGNED_BYTE,
            _ => gl::FLOAT,
        }
    }
}

impl From<texture::Type> for gl::types::GLenum {
    fn from(_type: texture::Type) -> gl::types::GLenum {
        match _type {
            texture::Type::TexCUBE => gl::TEXTURE_CUBE_MAP,
            _ => gl::TEXTURE_2D,
        }
    }
}
