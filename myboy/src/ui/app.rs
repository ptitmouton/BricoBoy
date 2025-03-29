use egui::Widget;
use std::{path::Path, thread};

use crate::device::device;

use super::{asm_text::AsmTextTable, cpu_registers::CPURegisterView};

pub struct AppTemplate {
    device: device::Device,
    path: Box<Path>,
}

impl Default for AppTemplate {
    fn default() -> Self {
        Self {
            device: device::Device::new(),
            path: Path::new("/Users/arinaldoni/Downloads/tetris.gb").into(),
        }
    }
}

impl eframe::App for AppTemplate {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update<'a>(&'a mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Debug");

            if self.device.mem_map.cartridge.is_some() {
                ui.horizontal(|ui| {
                    if self.device.running {
                        if ui.button("Pause").clicked() {
                            self.device.running = false;
                        }
                    } else {
                        if ui.button("Run").clicked() {
                            unsafe {
                                let raw_device_pointer =
                                    &mut self.device as *mut device::Device as usize;
                                thread::spawn(move || {
                                    println!("wil l start shortly");
                                    let raw_device = raw_device_pointer as *mut device::Device;
                                    let _ =
                                        <*mut device::Device>::as_mut(raw_device).unwrap().run();
                                    println!("did start");
                                });
                            }
                        }
                        if ui.button("> Step").clicked() {
                            self.device.running = false;
                            self.device.cycle();
                        }
                    }
                });
            } else {
                if ui
                    .button("Open")
                    .on_hover_text("Load a Gameboy ROM file")
                    .clicked()
                {
                    self.device.load_path(&self.path);
                }
            }

            ui.separator();

            ui.vertical(|ui| {
                ui.heading("CPU");

                CPURegisterView {
                    cpu: &self.device.cpu,
                }
                .ui(ui)
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Debug");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            // ui.separator();

            match &self.device.mem_map.cartridge {
                None => {
                    ui.label("No cartridge loaded");
                }
                Some(cartridge) => {
                    AsmTextTable {
                        cartridge: cartridge.clone(),
                        selected_address: None,
                    }
                    .ui(ui);
                }
            }
        });
    }
}
