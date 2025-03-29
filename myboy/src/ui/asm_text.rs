use egui::TextStyle;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use mygbcartridge::cartridge::Cartridge;

pub struct AsmTextTable {
    pub cartridge: Cartridge,
    pub selected_address: Option<u16>,
}

impl egui::Widget for AsmTextTable {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading(&self.cartridge.get_title());
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            self.asm_text_table(ui);
                        });
                    });
                });

            ui.response()
        })
        .response
    }
}

impl AsmTextTable {
    pub fn asm_text_table(&self, ui: &mut egui::Ui) {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .column(Column::auto().at_most(50.0))
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto())
            .sense(egui::Sense::click())
            .scroll_to_row(self.selected_address.unwrap_or_default().into(), None)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Address");
                });
                header.col(|ui| {
                    ui.strong("Text");
                });
                header.col(|ui| {
                    ui.strong("mnemonic");
                });
            })
            .body(|body| {
                body.rows(25.0, self.cartridge.data.len(), |mut row| {
                    let index = row.index();
                    row.col(|ui| {
                        ui.label(format!("0x{:04X}", index));
                    });
                    row.col(|ui| {
                        ui.label(format!("0x{:02X}", self.cartridge.data[index]));
                    });
                    row.col(|ui| {
                        ui.label("?");
                    });
                })
            });
    }
}
