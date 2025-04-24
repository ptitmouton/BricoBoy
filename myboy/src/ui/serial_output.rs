use egui::RichText;

use crate::Device;

pub struct SerialOutputView<'a> {
    pub device: &'a Device<'static>,
}

impl egui::Widget for SerialOutputView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading("Serial");

            ui.code(RichText::new(String::from_utf8_lossy(&self.device.serial_buffer)).monospace());
        })
        .response
    }
}

impl SerialOutputView<'_> {
    pub fn new(device: &'static Device) -> SerialOutputView<'static> {
        SerialOutputView { device }
    }
}
