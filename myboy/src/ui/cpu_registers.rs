use crate::cpu::{
    cpu::CPU,
    register_set::{ByteRegister, Flag, WordRegister},
};

pub struct CPURegisterView<'a> {
    pub cpu: &'a CPU,
}

impl egui::Widget for CPURegisterView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let regset = self.cpu.register_set;
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("AF:");
                ui.horizontal(|ui| {
                    ui.label(format!("${:02X}", regset.get_b(ByteRegister::A),));
                    ui.label(format!("${:02X}", regset.get_b(ByteRegister::F),))
                        .on_hover_ui(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Flags:");
                                ui.columns(2, |columns| {
                                    columns[0].label("Z (Zero) : Set if result is zero");
                                    columns[1].label(format!("{}", regset.get_flag(Flag::Zero)));
                                });
                                ui.columns(2, |columns| {
                                    columns[0].label("N (Subtract): Set if subtraction");
                                    columns[1]
                                        .label(format!("{}", regset.get_flag(Flag::Subtract)));
                                });
                                ui.columns(2, |columns| {
                                    columns[0]
                                        .label("H (Half Carry): Set if carry from bit 3 to 4");
                                    columns[1]
                                        .label(format!("{}", regset.get_flag(Flag::HalfCarry)));
                                });
                                ui.columns(2, |columns| {
                                    columns[0].label("C (Carry): Set if carry from bit 7");
                                    columns[1].label(format!("{}", regset.get_flag(Flag::Carry)));
                                });
                            });
                        });
                });

                ui.label("BC:");
                ui.horizontal(|ui| {
                    ui.label(format!("${:02X}", regset.get_b(ByteRegister::B),));
                    ui.label(format!("${:02X}", regset.get_b(ByteRegister::C),));
                });
            });

            ui.horizontal(|ui| {
                ui.label("DE:");
                ui.label(format!(
                    "0x{:02X}{:02X}",
                    regset.get_b(ByteRegister::D),
                    regset.get_b(ByteRegister::E)
                ));

                ui.label("HL:");
                ui.label(format!(
                    "0x{:02X}{:02X}",
                    regset.get_b(ByteRegister::H),
                    regset.get_b(ByteRegister::L)
                ));
            });

            ui.horizontal(|ui| {
                ui.label("SP:");
                ui.label(format!("0x{:04X}", regset.get_w(WordRegister::SP)));

                ui.label("PC:");
                ui.label(format!("0x{:04X}", regset.get_w(WordRegister::PC)));
            });
        })
        .response
    }
}
