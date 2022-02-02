// ui.rs
//
// Created on 2021/11/04 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use egui;

use crate::app::*;
use crate::iml;
use crate::render::egui_painter::EguiPainter;

pub struct Ui {
    egui_context: egui::CtxRef,
    egui_painter: EguiPainter,
    my_string: String,
}

impl Ui {
    pub fn new() -> Ui {
        let egui_context = egui::CtxRef::default();
        egui_context.set_visuals(egui::Visuals::light());
        Ui {
            egui_context,
            egui_painter: EguiPainter::new(),
            my_string: String::new(),
        }
    }

    pub fn update(&mut self, raw_input: egui::RawInput, lights: &mut Vec<Light>) {
        self.egui_context.begin_frame(raw_input);
        egui::Window::new("test").show(&self.egui_context, |ui| {
            ui.label("Lights");
            ui.separator();

            let _ = ui.add(egui::TextEdit::singleline(&mut self.my_string));
            let mut count = 1;
            for mut light in lights {
                let light_label = count.to_string();
                ui.label(light_label + ": ");
                let mut light_position = light.position;
                let light_color = light.color;

                ui.add(egui::Slider::new(&mut light_position.x, -70.0..=70.0).text("x"));
                ui.add(egui::Slider::new(&mut light_position.y, -70.0..=70.0).text("y"));
                ui.add(egui::Slider::new(&mut light_position.z, -70.0..=70.0).text("z"));
                ui.add(egui::Slider::new(&mut light_position.w, 0.0..=2000.0).text("intensity"));

                let mut color: [f32; 3] = [light_color.x, light_color.y, light_color.z];
                ui.color_edit_button_rgb(&mut color);

                light.position = light_position;
                light.color = iml::Vec4::from(iml::Vec3::from(color));
                ui.separator();
                count += 1;
            }
            ui.label("Material");
            ui.separator();

            // for entity in entities {
            //     let mut material = &mut entity.material;
            //     ui.add(egui::Slider::new(&mut material.roughness, 0.12..=1.0).text("roughness"));
            //     ui.add(egui::Slider::new(&mut material.metallic, 0.001..=1.0).text("metallic"));
            //     ui.add(egui::Slider::new(&mut material.ao, 0.0..=1.0).text("alpha"));
            //     ui.label("color");
            //     let mut color: [f32; 3] = [material.color.x, material.color.y, material.color.z];
            //     ui.color_edit_button_rgb(&mut color);
            //
            //     material.color = iml::Vec3::from(color);
            // }
        });
    }

    pub fn render(&mut self, width: f32, height: f32) {
        let (_, shapes) = self.egui_context.end_frame();
        let clipped_meshes = self.egui_context.tessellate(shapes);
        self.egui_painter.paint(
            &clipped_meshes,
            &self.egui_context.texture(),
            &iml::Vec2::new(width, height),
        )
    }
}
