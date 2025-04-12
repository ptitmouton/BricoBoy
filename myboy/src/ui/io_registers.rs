use egui::RichText;

use crate::io::{if_register::InterruptType, io_registers::IORegisters};

pub struct IORegisterView<'a> {
    pub registers: &'a IORegisters,
}

impl egui::Widget for IORegisterView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            let lcdc_reg = self.registers.get_lcdl_register();
            ui.label(RichText::new("LCD-Control Register (#FF40)").underline());
            ui.horizontal(|ui| {
                ui.label("LCD Enabled:");
                ui.label(format!("{}", lcdc_reg.lcd_enabled()));
            });
            ui.horizontal(|ui| {
                ui.label("Window Tile Map Display Select:");
                ui.label(format!("{}", lcdc_reg.window_tile_map_bank()));
            });
            ui.horizontal(|ui| {
                ui.label("Window Display Enabled:");
                ui.label(format!("{}", lcdc_reg.window_enabled()));
            });
            ui.horizontal(|ui| {
                ui.label("BG & Window Tile Data Select:");
                ui.label(format!("{}", lcdc_reg.bg_tile_data_bank()));
            });
            ui.horizontal(|ui| {
                ui.label("BG Tile Map Display Select:");
                ui.label(format!("{}", lcdc_reg.bg_tile_map_bank()));
            });
            ui.horizontal(|ui| {
                ui.label("Sprite Size:");
                ui.label(format!("{}", lcdc_reg.obj_size()));
            });
            ui.horizontal(|ui| {
                ui.label("Sprite Display Enabled:");
                ui.label(format!("{}", lcdc_reg.obj_enabled()));
            });
            ui.horizontal(|ui| {
                ui.label("BG Display:");
                ui.label(format!("{}", lcdc_reg.bg_enabled()));
            });

            ui.separator();

            ui.label(RichText::new("LCD").underline());
            ui.horizontal(|ui| {
                ui.label("LCDSTAT (Status #FF41):");
                ui.label(format!("0x{:02X}", self.registers.get_lcdstat()));
            });
            ui.horizontal(|ui| {
                ui.label("LY (Line Register #FF44):");
                ui.label(format!("{}", self.registers.get_lcd_ly()));
            });

            ui.label(RichText::new("General IO Registers (#FF00)").underline());
            ui.horizontal(|ui| {
                ui.label("#FF00:");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff00)));
            });
            ui.horizontal(|ui| {
                ui.label("SB (Serial transfer #FF01):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff01)));
            });
            ui.horizontal(|ui| {
                ui.label("SC (Serial control #FF02):");
                ui.label(format!("0x{:02X}", self.registers.read_byte(0xff02)));
            });
        })
        .response
    }
}
