use egui::TopBottomPanel;
use rfd::FileDialog;

use crate::emulator::EmulatorInstance;

use super::emulator_view::EmulatorView;

pub struct AppTemplate {
    emulator: Option<EmulatorInstance>,
}

impl Default for AppTemplate {
    fn default() -> Self {
        Self { emulator: None }
    }
}

impl eframe::App for AppTemplate {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        TopBottomPanel::top("top_panel").show_animated(ctx, self.emulator.is_some(), |ui| {
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

            ui.add(EmulatorView::new(self.emulator.as_mut().unwrap()))
        });

        if self.emulator.is_none() {
            TopBottomPanel::top("root_top_panel").show_animated(
                ctx,
                self.emulator.is_none(),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Select a Gameboy ROM file");
                        if ui.button("Open").clicked() {
                            if let Some(file) = FileDialog::new()
                                .set_title("Select a Gameboy file")
                                .add_filter("gb files", &["gb"])
                                .pick_file()
                            {
                                let emulator = EmulatorInstance::from_path(file.as_path());
                                self.emulator = Some(emulator);
                            }
                        }
                    });
                },
            );
        }
    }
}
