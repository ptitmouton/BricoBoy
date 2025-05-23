// use crate::ui::emulator_view::EmulatorView;
// use egui::CentralPanel;

use crate::Device;

#[expect(dead_code)]
pub struct AppTemplate {
    device: Device<'static>,
}

#[expect(dead_code)]
impl AppTemplate {
    pub fn new(device: Device<'static>) -> AppTemplate {
        AppTemplate { device }
    }
}

impl eframe::App for AppTemplate {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(0.04);
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        // CentralPanel::default().show(ctx, |ui| {
        //     let emulator_view = EmulatorView::new(Box::new(device));
        //     egui::menu::bar(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }

        //         egui::widgets::global_theme_preference_buttons(ui);
        //     });

        //     ui.add(emulator_view);
        // });
    }
}
