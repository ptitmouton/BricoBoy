use crate::cpu::{
    cpu::CPU,
    register_set::{ByteRegister, WordRegister},
};

pub struct CPURegisterView<'a> {
    pub cpu: &'a CPU,
}

impl egui::Widget for CPURegisterView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let regset = self.cpu.register_set;
        ui.vertical(|ui| {
            ui.heading("CPU registers");

            ui.horizontal(|ui| {
                ui.label("A:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::A)));
                ui.label("F:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::F)));
            });

            ui.horizontal(|ui| {
                ui.label("B:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::B)));
                ui.label("C:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::C)));
            });

            ui.horizontal(|ui| {
                ui.label("D:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::D)));
                ui.label("E:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::E)));
            });

            ui.horizontal(|ui| {
                ui.label("H:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::H)));
                ui.label("L:");
                ui.label(format!("{:02X}", regset.get_b(ByteRegister::L)));
            });

            ui.horizontal(|ui| {
                ui.label("SP:");
                ui.label(format!("{:04X}", regset.get_w(WordRegister::SP)));
            });
            ui.horizontal(|ui| {
                ui.label("PC:");
                ui.label(format!("{:04X}", regset.get_w(WordRegister::PC)));
            });
        })
        .response
    }
}
