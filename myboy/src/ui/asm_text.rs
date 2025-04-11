use std::fmt::Display;

use egui::{
    Align, Color32, FontSelection, Id, RichText, Style, TextStyle, Widget, text::LayoutJob,
};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::{
    Device,
    cpu::{addressing_mode::AddressingMode, instruction::Instruction, register_set::RegisterSet},
    device::mem_map::MemMap,
};

pub struct AsmTextTable<'a> {
    pub device: &'a mut Device,

    last_scrolled_rowid: Option<u16>,
}

impl<'a> Widget for AsmTextTable<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading(&self.device.cartridge.get_title());
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
    pub fn new(device: &mut Device) -> AsmTextTable<'_> {
        let last_scrolled_rowid = None;
        AsmTextTable {
            device,
            last_scrolled_rowid,
        }
    }

    fn asm_text_table(&mut self, ui: &mut egui::Ui) {
        let last_scrolled_rowid_id = Id::from("last_scrolled_rowid");
        let mut last_scrolled_rowid = ui.data_mut(|d| {
            *d.get_temp_mut_or_insert_with(last_scrolled_rowid_id, || self.last_scrolled_rowid)
        });

        let body_text_size = TextStyle::Body.resolve(ui.style()).size;

        let current_address = *self.device.cpu.register_set.pc();

        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .column(Column::auto().at_least(40.0).at_most(60.0).clip(true))
            .column(
                Column::remainder()
                    .at_least(200.0)
                    .at_most(400.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto().at_least(100.0).resizable(true));

        if last_scrolled_rowid.is_none_or(|rowid| rowid != current_address) {
            last_scrolled_rowid.replace(current_address);
            self.last_scrolled_rowid.replace(current_address);
            table = table.scroll_to_row(current_address as usize, None);
        }
        table = table.scroll_to_row(current_address as usize, None);

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
                body.rows(body_text_size + 5.0, 0xffff, |mut row| {
                    let rowid = row.index();
                    if let Ok(instruction) = Instruction::create(rowid as u16, &self.device.mem_map)
                    {
                        if *self.device.cpu.register_set.pc() == instruction.address {
                            row.set_selected(true);
                        }
                        if let Some(ref addr) = self.device.breakpoint {
                            if *addr == instruction.address {
                                row.set_selected(true);
                            }
                        }
                        let is_breakpoint = self.device.breakpoint == Some(instruction.address);

                        row.col(|ui| {
                            let label = match is_breakpoint {
                                true => ui.colored_label(
                                    Color32::from_rgb_additive(255, 24, 25),
                                    format!("0x{:04X}", instruction.address),
                                ),
                                false => ui.label(format!("0x{:04X}", instruction.address)),
                            };

                            if label.clicked() {
                                self.device.toggle_breakpoint(instruction.address);
                            }
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", &instruction.instruction_type));

                                if let Some(condition) = &instruction.condition {
                                    ui.label(format!(" {}", condition));
                                }

                                if let Some(target) = &instruction.target {
                                    AsmTextTable::render_source_or_target(
                                        ui,
                                        target,
                                        instruction.address,
                                        &self.device.mem_map,
                                        &self.device.cpu.register_set,
                                    );
                                }
                                if let Some(source) = &instruction.source {
                                    AsmTextTable::render_source_or_target(
                                        ui,
                                        source,
                                        instruction.address,
                                        &self.device.mem_map,
                                        &self.device.cpu.register_set,
                                    );
                                }
                            });
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                for i in 0..instruction.size() {
                                    ui.label(AsmTextTable::byte_text(
                                        self.device
                                            .mem_map
                                            .read_byte(instruction.address + (i as u16)),
                                    ));
                                }
                            });
                        });
                    } else {
                        row.col(|ui| {
                            ui.label(format!("0x{:04X}", rowid));
                        });
                        row.col(|ui| {
                            ui.label("Invalid instruction");
                        });
                        row.col(|ui| {
                            ui.label("Invalid instruction");
                        });
                    }
                })
            });

        ui.data_mut(|d| {
            d.insert_temp(last_scrolled_rowid_id, self.last_scrolled_rowid);
        });
    }

    fn render_source_or_target(
        ui: &mut egui::Ui,
        addressing_mode: &AddressingMode,
        instr_address: u16,
        memory: &MemMap,
        register_set: &RegisterSet,
    ) {
        match addressing_mode {
            AddressingMode::ImmediateByte => {
                ui.label(AsmTextTable::byte_text(memory.read_byte(instr_address + 1)));
            }
            AddressingMode::ImmediateWord => {
                ui.label(AsmTextTable::word_text(memory.read_word(instr_address + 1)));
            }
            AddressingMode::ImmediatePointer => {
                let pointer_address = memory.read_word(instr_address + 1);
                ui.label(AsmTextTable::pointer_text(pointer_address))
                    .on_hover_text(AsmTextTable::byte_or_word_text(
                        memory.read_byte(pointer_address),
                        memory.read_byte(pointer_address + 1),
                    ));
            }
            AddressingMode::ImmediatePointerHigh => {
                let low_byte = memory.read_byte(instr_address + 1);
                let pointer_address = 0xff00 + low_byte as u16;
                ui.label(AsmTextTable::pointerh_text(low_byte))
                    .on_hover_text(AsmTextTable::byte_or_word_text(
                        memory.read_byte(pointer_address),
                        memory.read_byte(pointer_address.wrapping_add(1)),
                    ));
            }
            AddressingMode::ByteRegister(register) => {
                ui.label(AsmTextTable::register_text(register))
                    .on_hover_text(AsmTextTable::byte_text(*register_set.get_b(*register)));
            }
            AddressingMode::WordRegister(register) => {
                ui.label(AsmTextTable::register_text(register))
                    .on_hover_text(AsmTextTable::word_text(register_set.get_w(*register)));
            }
            AddressingMode::Target(target_addr) => {
                let pointer_address = *target_addr;
                ui.label(AsmTextTable::address_text(pointer_address))
                    .on_hover_text(AsmTextTable::byte_or_word_text(
                        memory.read_byte(pointer_address),
                        memory.read_byte(pointer_address + 1),
                    ));
            }
            AddressingMode::RegisterPointer(register) => {
                let pointer_address = register_set.get_w(*register);
                let style = Style::default();
                let mut hover_text = LayoutJob::default();
                AsmTextTable::word_text(pointer_address).append_to(
                    &mut hover_text,
                    &style,
                    FontSelection::Default,
                    Align::Min,
                );
                RichText::new(" -> ").append_to(
                    &mut hover_text,
                    &style,
                    FontSelection::Default,
                    Align::Center,
                );
                AsmTextTable::byte_or_word_text(
                    memory.read_byte(pointer_address),
                    memory.read_byte(pointer_address.wrapping_add(1)),
                )
                .append_to(
                    &mut hover_text,
                    &style,
                    FontSelection::Default,
                    Align::Min,
                );
                ui.label(AsmTextTable::register_ptr_text(register))
                    .on_hover_text(hover_text);
            }
            AddressingMode::RegisterPointerHigh(register) => {
                let pointer_address = 0xff00 + (*register_set.get_b(*register) as u16);
                ui.label(AsmTextTable::register_ptrh_text(register))
                    .on_hover_text(AsmTextTable::byte_text(memory.read_byte(pointer_address)));
            }
        }
    }

    fn byte_text(byte: u8) -> RichText {
        RichText::new(format!("${:02x}", byte))
            .color(Color32::from_rgb(255, 25, 0))
            .monospace()
    }

    fn word_text(word: u16) -> RichText {
        RichText::new(format!("${:04x}", word))
            .color(Color32::from_rgb(255, 25, 0))
            .monospace()
    }

    fn byte_or_word_text(byte1: u8, byte2: u8) -> RichText {
        RichText::new(format!("${:02x} ${:02x}", byte1, byte2))
            .color(Color32::from_rgb(255, 25, 0))
            .monospace()
    }

    fn pointer_text(address: u16) -> RichText {
        RichText::new(format!("(${:04x})", address))
            .color(Color32::from_rgb(0, 255, 25))
            .monospace()
    }

    fn pointerh_text(low_byte: u8) -> RichText {
        RichText::new(format!("($ff00 + ${:02x})", low_byte))
            .color(Color32::from_rgb(0, 255, 25))
            .monospace()
    }

    fn address_text(address: u16) -> RichText {
        RichText::new(format!("(${:04x})", address))
            .color(Color32::from_rgb(25, 255, 0))
            .monospace()
    }

    fn register_text<T: Display>(register: T) -> RichText {
        RichText::new(format!("{}", register))
            .color(Color32::from_rgb(0, 255, 25))
            .monospace()
    }

    fn register_ptr_text<T: Display>(register: T) -> RichText {
        RichText::new(format!("({})", register))
            .color(Color32::from_rgb(0, 255, 25))
            .monospace()
    }

    fn register_ptrh_text<T: Display>(register: T) -> RichText {
        RichText::new(format!("(0xff00 + {})", register))
            .color(Color32::from_rgb(0, 255, 25))
            .monospace()
    }
}
