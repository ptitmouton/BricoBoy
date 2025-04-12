use egui::CentralPanel;
use mygbcartridge::cartridge::Cartridge;
use rfd::FileDialog;

use crate::Device;

use super::emulator_view::EmulatorView;

pub struct AppTemplate {
    device: Option<Device>,
}

impl AppTemplate {
    pub fn new(device: Option<Device>) -> AppTemplate {
        AppTemplate { device }
    }
}

impl eframe::App for AppTemplate {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(0.04);
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });

            if self.device.is_some() {
                ui.add(EmulatorView::new(self.device.as_mut().unwrap()))
            } else {
                ui.vertical_centered(|ui| {
                    ui.label("No ROM loaded");
                    ui.label("Please select a ROM file.");

                    ui.horizontal(|ui| {
                        if ui.button("Open").clicked() {
                            if let Some(file) = FileDialog::new()
                                .set_title("Select a Gameboy file")
                                .add_filter("gb files", &["gb"])
                                .pick_file()
                            {
                                let cartridge = Cartridge::new(file.as_path());
                                let device = Device::new(cartridge);
                                self.device = Some(device);
                            }
                        }
                    });
                })
                .response
            }
        });
    }
}
