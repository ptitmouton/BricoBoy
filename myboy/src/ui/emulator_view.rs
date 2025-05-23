use std::thread::{self, JoinHandle};

use egui::{CentralPanel, CollapsingHeader, Response, RichText, SidePanel, Widget};

use crate::{device::device::Device, io::if_register::InterruptType};

use super::{asm_text::AsmTextTable, cpu_registers::CPURegisterView, io_registers::IORegisterView};

enum MainView {
    Program,
    Memory,
    Serial,
}

pub struct EmulatorView {
    device: Box<Device<'static>>,
    active_view: MainView,
    scrollfollowing: bool,
    perm_scrollfollowing: bool,
}

impl EmulatorView {
    pub fn new(device: Box<Device<'static>>) -> EmulatorView {
        EmulatorView {
            device,
            active_view: MainView::Program,
            scrollfollowing: false,
            perm_scrollfollowing: false,
        }
    }
}

pub fn run_emulator(device: &mut Device) -> Result<JoinHandle<()>, String> {
    if device.running {
        return Err("Emulator is already running".to_string());
    }
    unsafe {
        let raw_device_pointer = device as *mut Device as usize;
        Ok(thread::spawn(move || {
            let raw_device = raw_device_pointer as *mut Device;
            let _ = <*mut Device>::as_mut(raw_device).unwrap().run();
        }))
    }
}

impl Widget for EmulatorView {
    fn ui(mut self, ui: &mut egui::Ui) -> Response {
        if !self.perm_scrollfollowing {
            self.scrollfollowing = false
        } else {
            self.scrollfollowing = !self.device.cpu.is_busy();
        }
        ui.group(move |ui| {
            SidePanel::left("side_panel").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if self.device.running {
                        if ui.button("Pause").clicked() {
                            self.device.running = false;
                        }
                    } else {
                        if ui.button("Run").clicked() {
                            let _ = run_emulator(&mut self.device);
                        }
                        if ui.button("> Step").clicked() {
                            self.device.running = false;
                            self.device.step();
                            self.scrollfollowing = true
                        }
                    }

                    ui.menu_button(format!("{:.2}x", self.device.speed_multiplier), |ui| {
                        ui.label("speed multiplier");
                        ui.horizontal(|ui| {
                            ui.label("Speed: ");
                            ui.add_enabled_ui(self.device.speed_multiplier > 0.059, |ui| {
                                if ui.button("-").clicked() {
                                    self.device.speed_multiplier -= 0.05;
                                }
                            });
                            ui.label(format!("{:.2}x", self.device.speed_multiplier));
                            if ui.button("+").clicked() {
                                self.device.speed_multiplier += 0.05;
                            }
                        });
                    });
                });

                ui.separator();

                CollapsingHeader::new("CPU Registers")
                    .default_open(true)
                    .show(ui, |ui| {
                        CPURegisterView {
                            cpu: &self.device.cpu,
                        }
                        .ui(ui)
                    });

                CollapsingHeader::new("IO Registers")
                    .default_open(true)
                    .show(ui, |ui| {
                        IORegisterView {
                            registers: &self.device.mem_map.io_registers,
                        }
                        .ui(ui)
                    });

                CollapsingHeader::new("Interrupts")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(format!(
                                "IME: {:?}",
                                &self.device.cpu.interrupt_master_enable
                            ));
                            ui.label(format!(
                                "IE: {:#02X}",
                                &self.device.mem_map.io().ie_register.0
                            ));
                            ui.label(format!(
                                "IF: {:#02X}",
                                &self.device.mem_map.io().if_register.0
                            ));
                            ui.columns(3, |columns| {
                                columns[0].label("");
                                columns[1].label(RichText::new("enabled?").size(10.0));
                                columns[2].label(RichText::new("requested?").size(10.0));

                                columns[0].label("V-Blank: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .ie_register
                                        .is_vblank_handler_enabled()
                                ));
                                columns[2].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .if_register
                                        .is_requested(InterruptType::VBlank)
                                ));
                                columns[0].label("LCD Stat: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .ie_register
                                        .is_lcd_handler_enabled()
                                ));
                                columns[2].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .if_register
                                        .is_requested(InterruptType::LCDStat)
                                ));
                                columns[0].label("Timer: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .ie_register
                                        .is_timer_handler_enabled()
                                ));
                                columns[2].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .if_register
                                        .is_requested(InterruptType::Timer)
                                ));
                                columns[0].label("Serial: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .ie_register
                                        .is_serial_handler_enabled()
                                ));
                                columns[2].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .if_register
                                        .is_requested(InterruptType::Serial)
                                ));
                                columns[0].label("Joypad: ");
                                columns[1].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .ie_register
                                        .is_joypad_handler_enabled()
                                ));
                                columns[2].label(format!(
                                    "{}",
                                    self.device
                                        .mem_map
                                        .io()
                                        .if_register
                                        .is_requested(InterruptType::Joypad)
                                ));
                            });
                        });
                    });

                ui.separator();

                ui.checkbox(
                    &mut self.perm_scrollfollowing,
                    RichText::new("Scroll following").monospace(),
                );
            });

            CentralPanel::default().show_inside(ui, |ui| {
                let cartridge = &self.device.cartridge;
                ui.heading(cartridge.get_title());
                ui.horizontal(|ui| {
                    ui.label("Cartridge Type:");
                    ui.label(format!("{}", cartridge.get_cartridge_type().unwrap()));
                    ui.label("Licensee:");
                    ui.label(format!("{}", cartridge.get_licensee().unwrap()));
                    ui.label("ROM Size:");
                    ui.label(format!("{}", cartridge.get_rom_size()));
                    // ui.label("Manufacturer Code:");
                    // ui.label(format!("{}", cartridge.manufacturer_code()));
                    ui.label("ROM Bank Count:");
                    ui.label(format!("{}", cartridge.get_rom_bank_count()));
                    ui.label("GBC support:");
                    ui.label(format!("{}", cartridge.get_gbc_support()));
                });

                ui.horizontal_top(|ui| {
                    let program_button = ui.button("program");
                    let memory_button = ui.button("memory");
                    let serial_button = ui.button("serial");
                    match self.active_view {
                        MainView::Program => {
                            program_button.enabled();
                        }
                        MainView::Memory => {
                            memory_button.enabled();
                        }
                        MainView::Serial => {
                            serial_button.enabled();
                        }
                    };
                    if program_button.clicked() {
                        self.active_view = MainView::Program;
                    }
                    if memory_button.clicked() {
                        self.active_view = MainView::Memory;
                    }
                    if serial_button.clicked() {
                        self.active_view = MainView::Serial;
                    }
                });

                ui.add(AsmTextTable::new(&mut self.device, self.scrollfollowing));

                // match self.active_view {
                //     MainView::Program => {
                //         ui.add(AsmTextTable::new(self.emulator));
                //     }
                //     MainView::Memory => {
                //         ui.label("Memory");
                //     }
                //     MainView::Serial => {
                //         ui.label("Serial");

                //         ui.code(
                //             RichText::new(String::from_utf8_lossy(
                //                 &self.emulator.device.serial_buffer,
                //             ))
                //             .monospace(),
                //         );
                //     }
                // };
            });

            SidePanel::right("side_panel_r").show_inside(ui, move |ui| {
                ui.label("Serial Output");
                // ui.add_sized(
                //     ui.available_size().min(egui::Vec2 { x: 400.0, y: 600.0 }),
                //     SerialOutputView::new(&mut self.device),
                // );
            });
        })
        .response
    }
}
