// ui.rs
//
// Created on 2021/11/04 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use egui;

use crate::app::*;
use crate::math;
use crate::render::egui_painter::EguiPainter;

pub struct Ui {
    egui_context: egui::CtxRef,
    egui_painter: EguiPainter,
}

impl Ui {
    pub fn new() -> Ui {
        let egui_context = egui::CtxRef::default();
        egui_context.set_visuals(egui::Visuals::light());
        Ui {
            egui_context,
            egui_painter: EguiPainter::new(),
        }
    }

    pub fn update(&mut self, raw_input: egui::RawInput, entities: &mut Vec<Entity>) {
        self.egui_context.begin_frame(raw_input);

        egui::Window::new("").show(&self.egui_context, |ui| {
            // ui.label("Lights");
            // ui.separator();
            // let mut distance = 0.0;
            // ui.add(egui::Slider::new(&mut distance, 2.0..=10.0).text(": z"));

            ui.label("Material");
            ui.separator();

            for entity in entities {
                let mut material = &mut entity.material;
                ui.add(egui::Slider::new(&mut material.roughness, 0.001..=1.0).text("roughness"));
                ui.add(egui::Slider::new(&mut material.metallic, 0.001..=1.0).text("metallic"));
                ui.add(egui::Slider::new(&mut material.ao, 0.0..=1.0).text("alpha"));
                ui.label("color");
                let mut color: [f32; 3] = [material.color.x, material.color.y, material.color.z];
                ui.color_edit_button_rgb(&mut color);

                material.color = math::Vec3::from(color);
            }
        });
    }

    pub fn render(&mut self, width: f32, height: f32) {
        let (_, shapes) = self.egui_context.end_frame();
        let clipped_meshes = self.egui_context.tessellate(shapes);
        self.egui_painter.paint(
            &clipped_meshes,
            &self.egui_context.texture(),
            &math::Vec2::new(width, height),
        )
    }
}
