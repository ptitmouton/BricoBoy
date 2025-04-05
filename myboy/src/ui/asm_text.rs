use egui::{Color32, Id, TextStyle, Widget};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::emulator::EmulatorInstance;

pub struct AsmTextTable<'a> {
    pub emulator: &'a mut EmulatorInstance,

    last_scrolled_rowid: Option<u16>,
}

impl<'a> Widget for AsmTextTable<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading(&self.emulator.cartridge_data.cartridge.get_title());
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        self.asm_text_table(ui);
                    });
                });

            ui.response()
        })
        .response
    }
}

impl AsmTextTable<'_> {
    pub fn new(emulator: &mut EmulatorInstance) -> AsmTextTable<'_> {
        let last_scrolled_rowid = None;
        AsmTextTable {
            emulator,
            last_scrolled_rowid,
        }
    }

    fn asm_text_table(&mut self, ui: &mut egui::Ui) {
        let last_scrolled_rowid_id = Id::from("last_scrolled_rowid");
        let mut last_scrolled_rowid = ui.data_mut(|d| {
            *d.get_temp_mut_or_insert_with(last_scrolled_rowid_id, || self.last_scrolled_rowid)
        });

        let body_text_size = TextStyle::Body.resolve(ui.style()).size;

        let current_address_rowid = *self.emulator.device.cpu.register_set.pc();

        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .column(Column::auto().at_least(40.0).at_most(60.0).clip(true))
            .column(
                Column::remainder()
                    .at_least(200.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto().at_least(100.0).resizable(true));

        if last_scrolled_rowid.is_none_or(|rowid| rowid != current_address_rowid as u16) {
            last_scrolled_rowid.replace(current_address_rowid as u16);
            self.last_scrolled_rowid
                .replace(current_address_rowid as u16);
            table = table.scroll_to_row(current_address_rowid as usize, None);
        }

        table
            .sense(egui::Sense::click())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Address");
                });
                header.col(|ui| {
                    ui.strong("Text");
                });
                header.col(|ui| {
                    ui.strong("value");
                });
            })
            .body(|body| {
                body.rows(
                    body_text_size + 5.0,
                    self.emulator.cartridge_data.instructions.len(),
                    |mut row| {
                        let rowid = row.index();
                        if *self.emulator.device.cpu.register_set.pc() == rowid as u16 {
                            row.set_selected(true);
                        }
                        if let Some(addr) = self.emulator.device.breakpoint {
                            if addr == rowid as u16 {
                                row.set_selected(true);
                            }
                        }
                        let is_breakpoint = self.emulator.device.breakpoint == Some(rowid as u16);

                        let instruction = &self.emulator.cartridge_data.instructions[rowid];
                        match instruction {
                            Ok(instruction) => {
                                row.col(|ui| {
                                    let label = match is_breakpoint {
                                        true => ui.colored_label(
                                            Color32::from_rgb_additive(255, 24, 25),
                                            format!("0x{:04X}", instruction.address),
                                        ),
                                        false => ui.label(format!("0x{:04X}", instruction.address)),
                                    };

                                    if label.clicked() {
                                        self.emulator.device.toggle_breakpoint(instruction.address);
                                    }
                                });
                                row.col(|ui| {
                                    let target_name = match &instruction.target {
                                        Some(target) => format!("{}", target),
                                        None => "".to_string(),
                                    };
                                    let source_name = match &instruction.source {
                                        Some(source) => format!(", {}", source),
                                        None => "".to_string(),
                                    };
                                    let text = format!(
                                        "{} {} {}",
                                        instruction.instruction_type, target_name, source_name
                                    );
                                    ui.label(text);
                                });
                                row.col(|ui| {
                                    let byte = self.emulator.cartridge_data.cartridge.data
                                        [instruction.address as usize];
                                    ui.label(format!("0x{:02x}", byte));
                                });
                            }
                            Err((address, byte)) => {
                                row.col(|ui| {
                                    ui.label(format!("0x{:04X}", address));
                                });
                                row.col(|ui| {
                                    ui.label("");
                                });
                                row.col(|ui| {
                                    ui.label(format!("0x{:02x}", byte));
                                });
                            }
                        }
                    },
                )
            });

        ui.data_mut(|d| {
            d.insert_temp(last_scrolled_rowid_id, self.last_scrolled_rowid);
        });
    }
}
