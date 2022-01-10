// main.rs
//
// Created on 2021/09/24 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

extern crate egui;
extern crate gl;
extern crate glfw;
extern crate gltf;

mod clock;
mod math;
mod render;

mod app;

fn main() {
    let mut application = app::App::init(1080, 1080);
    application.run();
}
