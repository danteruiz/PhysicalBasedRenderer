// buffer.rs
//
// Created on 2021/12/02 by Enter Your Name Here
// Copyright 2021 Enter Your Name Here
//
// Distributed under the MIT Lisense
// https://mit-license.org/

pub struct Buffer {
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { data: Vec::new() }
    }
}

// pub struct BufferView {
//     buffer: *const Buffer,
//     offset: usize,
//     size: usize,
// }
