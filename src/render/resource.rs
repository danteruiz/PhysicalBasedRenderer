// resource.rs
//
// Created on 2022/01/23 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

#[derive(Default)]
pub(crate) struct GPUResource {
    pub(crate) handle: u32,
    pub(crate) resource_type: u32,
}
