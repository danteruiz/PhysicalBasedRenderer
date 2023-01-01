// app.rs
//
// Created on 2022/01/06 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use core::f32;
use std::convert::From;
use std::{error::Error, result::Result};

use egui;
use glfw;

use ash::{vk, Entry, Instance};

fn glfw_action_to_boolean(action: glfw::Action) -> bool {
    match action {
        glfw::Action::Press | glfw::Action::Repeat => true,
        glfw::Action::Release => false,
    }
}

fn glfw_key_to_egui_key(key: glfw::Key) -> Option<egui::Key> {
    match key {
        glfw::Key::Backspace => Some(egui::Key::Backspace),
        glfw::Key::Enter => Some(egui::Key::Enter),
        glfw::Key::Tab => Some(egui::Key::Tab),
        glfw::Key::Space => Some(egui::Key::Space),
        glfw::Key::Escape => Some(egui::Key::Escape),
        glfw::Key::A => Some(egui::Key::A),
        glfw::Key::B => Some(egui::Key::B),
        glfw::Key::C => Some(egui::Key::C),
        glfw::Key::D => Some(egui::Key::D),
        glfw::Key::E => Some(egui::Key::E),
        glfw::Key::F => Some(egui::Key::F),
        glfw::Key::G => Some(egui::Key::G),
        glfw::Key::H => Some(egui::Key::H),
        glfw::Key::I => Some(egui::Key::I),
        glfw::Key::J => Some(egui::Key::J),
        glfw::Key::K => Some(egui::Key::K),
        glfw::Key::L => Some(egui::Key::L),
        glfw::Key::M => Some(egui::Key::M),
        glfw::Key::N => Some(egui::Key::N),
        glfw::Key::O => Some(egui::Key::O),
        glfw::Key::P => Some(egui::Key::P),
        glfw::Key::Q => Some(egui::Key::Q),
        glfw::Key::R => Some(egui::Key::R),
        glfw::Key::S => Some(egui::Key::S),
        glfw::Key::T => Some(egui::Key::T),
        glfw::Key::U => Some(egui::Key::U),
        glfw::Key::V => Some(egui::Key::V),
        glfw::Key::W => Some(egui::Key::W),
        glfw::Key::X => Some(egui::Key::X),
        glfw::Key::Y => Some(egui::Key::Y),
        glfw::Key::Z => Some(egui::Key::Z),
        glfw::Key::Num0 => Some(egui::Key::Num0),
        glfw::Key::Num1 => Some(egui::Key::Num1),
        glfw::Key::Num2 => Some(egui::Key::Num2),
        glfw::Key::Num3 => Some(egui::Key::Num3),
        glfw::Key::Num4 => Some(egui::Key::Num4),
        glfw::Key::Num5 => Some(egui::Key::Num5),
        glfw::Key::Num6 => Some(egui::Key::Num6),
        glfw::Key::Num7 => Some(egui::Key::Num7),
        glfw::Key::Num8 => Some(egui::Key::Num8),
        glfw::Key::Num9 => Some(egui::Key::Num9),
        _ => None,
    }
}

struct VkInstance {
    entry: Entry,
    instance: Instance,
}

impl VkInstance {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        println!("Crete VkInstance");
        let entry = unsafe { Entry::load()? };
        let instance = Self::create_instance(&entry)?;
        Ok(Self { entry, instance })
    }

    fn create_instance(entry: &ash::Entry) -> Result<Instance, Box<dyn Error>> {
        let app_info = ash::vk::ApplicationInfo {
            api_version: ash::vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };

        unsafe { Ok(entry.create_instance(&create_info, None)?) }
    }
}

type WindowEvents = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;
pub struct App {
    window: glfw::Window,
    events: WindowEvents,
    glfw: glfw::Glfw,
    vk_instance: VkInstance,
    // model_cache: render::ModelCache,
    // texture_cache: render::texture::TextureCache,
    // debug_ui: ui::Ui,
}

impl App {
    pub fn init(width: u32, height: u32) -> Result<App, Box<dyn Error>> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = glfw
            .create_window(width, height, "PBR Demo", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        let vk_instance = VkInstance::new()?;

        Ok(App {
            window,
            events,
            glfw,
            vk_instance,
        })
    }

    pub fn run(self) {
        let App {
            mut window,
            events,
            mut glfw,
            vk_instance,
        } = self;


	vk_instance.instance.enumerate_physical_devices() {
	}
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);

        while !window.should_close() {
            let _ = App::process_events(&mut glfw, &mut window, &events);
        }
    }

    pub fn process_events(
        glfw: &mut glfw::Glfw,
        window: &mut glfw::Window,
        events: &WindowEvents,
    ) -> egui::RawInput {
        glfw.poll_events();
        let mut raw_input = egui::RawInput::default();

        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::Key(glfw_key, _, action, _) => {
                    let pressed = match action {
                        glfw::Action::Release => false,
                        glfw::Action::Press => true,
                        glfw::Action::Repeat => true,
                    };

                    match glfw_key {
                        glfw::Key::Enter | glfw::Key::Backspace => {
                            let egui_key = glfw_key_to_egui_key(glfw_key);

                            match egui_key {
                                Some(key) => {
                                    let key_action = egui::Event::Key {
                                        key,
                                        pressed,
                                        modifiers: egui::Modifiers::default(),
                                    };
                                    raw_input.events.push(key_action);
                                }
                                _ => {
                                    println!("Key is not supported: {:?}", glfw_key);
                                }
                            }
                        }
                        _ => {
                            if pressed {
                                let key_text = glfw_key.get_name();

                                match key_text {
                                    Some(text) => raw_input.events.push(egui::Event::Text(text)),
                                    _ => {
                                        if glfw_key == glfw::Key::Space {
                                            raw_input
                                                .events
                                                .push(egui::Event::Text(String::from(" ")));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let position = egui::Pos2 {
                        x: x as f32,
                        y: y as f32,
                    };
                    raw_input.events.push(egui::Event::PointerMoved(position));
                }
                glfw::WindowEvent::MouseButton(glfw_button, action, _) => {
                    let pressed = glfw_action_to_boolean(action);

                    let egui_button = match glfw_button {
                        glfw::MouseButtonLeft => Some(egui::PointerButton::Primary),
                        glfw::MouseButtonMiddle => Some(egui::PointerButton::Middle),
                        glfw::MouseButtonRight => Some(egui::PointerButton::Secondary),
                        _ => None,
                    };

                    match egui_button {
                        Some(button) => {
                            let mouse_position = window.get_cursor_pos();
                            let pos = egui::Pos2 {
                                x: mouse_position.0 as f32,
                                y: mouse_position.1 as f32,
                            };

                            let event = egui::Event::PointerButton {
                                pos,
                                button,
                                pressed,
                                modifiers: egui::Modifiers::default(),
                            };

                            raw_input.events.push(event);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        raw_input
    }
}
