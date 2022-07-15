// main.rs
//
// Created on 2021/09/24 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

//extern crate ahash;
extern crate ahash;
extern crate egui;
extern crate gl;
extern crate glfw;
extern crate gltf;
extern crate iml;
extern crate rspirv;

mod app;
mod clock;
mod file_watcher;
mod render;
mod ui;

fn main() {
    let application = app::App::init(1080, 1080);

    application.run();
}
