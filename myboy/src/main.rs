mod cli;
mod cpu;
mod device;
pub(crate) mod io;
mod logging;
mod memory;
mod ppu;
mod ui;

use clap::Parser;
use cli::args::{Cli, Commands};
use device::device::Device;
use logging::log::{ConsoleLogger, InMemoryLogger, Logger};
use mygbcartridge::cartridge::Cartridge;
use ppu::ppu::PPU;
use ui::{app::AppTemplate, emulator_view::run_emulator};

fn create_default_logger(cli: &Cli) -> Box<dyn Logger> {
    if cli.headless {
        Box::new(InMemoryLogger::default())
    } else {
        Box::new(ConsoleLogger::default())
    }
}

fn create_default_device(cli: &Cli) -> Option<Device> {
    match &cli.command {
        Some(Commands::Device {
            breakpoint,
            file,
            log_outputs,
        }) => {
            if let Some(path) = file {
                let mut logger = create_default_logger(cli);
                if let Some(log_outputs) = log_outputs {
                    logger.set_supported_outputs(log_outputs.clone());
                }

                let cartridge = Cartridge::new(path);
                let mut dev = Device::new(cartridge);
                dev.logger = logger;
                if let Some(bp) = breakpoint {
                    dev.breakpoint = Some(*bp);
                }

                return Some(dev);
            }

            return None;
        }
        None => None,
    }
}

fn main() {
    let cli = Cli::parse();

    let mut logger = create_default_logger(&cli);

    logger.info(logging::log::Log::Msg(
        "Starting MyBoy Gameboy Emulator".to_string(),
    ));

    let command = cli.command.clone().unwrap_or(Commands::Device {
        breakpoint: None,
        file: None,
        log_outputs: None,
    });

    match command {
        Commands::Device { .. } => {
            logger.info(logging::log::Log::Msg("Creating device".to_string()));
            let device = create_default_device(&cli);

            if cli.headless {
                if device.is_none() {
                    logger.error(logging::log::Log::Msg(
                        "No ROM file provided. Exiting.".to_string(),
                    ));
                    return;
                }
                _ = run_device_headless(device.unwrap());
            } else {
                _ = open_device_view(device);
            }
        }
    }
}

fn run_device_headless(device: Device) -> Result<(), String> {
    let device = Box::leak(Box::new(device));

    run_emulator(device)?
        .join()
        .map_err(|e| format!("Failed to run emulator in headless mode: {:?}", e))
}

fn open_device_view(device: Option<Device>) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 200.0]),
        ..eframe::NativeOptions::default()
    };
    // .with_icon(
    //     // NOTE: Adding an icon is optional
    //     eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
    //         .expect("Failed to load icon"),
    // ),

    eframe::run_native(
        "MyBoy Gameboy Emulator",
        native_options,
        Box::new(|_cc| Ok(Box::new(AppTemplate::new(device)))),
    )
}
