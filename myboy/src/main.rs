mod cli;
mod cpu;
mod device;
pub(crate) mod io;
mod logging;
mod memory;
mod ppu;
mod screen;
mod ui;

use clap::Parser;
use cli::args::{Cli, Commands};
use device::device::Device;
use logging::log::{ConsoleLogger, Logger};
use mygbcartridge::cartridge::Cartridge;
use ppu::ppu::PPU;
use screen::open_gamescreen;
use ui::emulator_view::run_emulator;

fn create_default_logger(cli: &Cli) -> Box<dyn Logger> {
    let disabled_logtypes = &cli.disable_logtypes.clone();
    let mut logger = Box::new(ConsoleLogger::default());

    if let Some(disabled_logtypes) = disabled_logtypes {
        logger.set_disabled_outputs(disabled_logtypes.clone());
    }

    logger
}

fn create_default_device<'a>(cli: Cli) -> Result<Device<'a>, String> {
    let command = &cli.command.as_ref().ok_or_else(|| "No command provided")?;
    match command {
        Commands::Play { file, .. } => {
            let cartridge = Cartridge::new(file.as_path());
            let device = Device::new(cartridge);

            Ok(device)
        }
        Commands::Debug {
            file, breakpoint, ..
        } => {
            let cartridge = Cartridge::new(file.as_path());
            let mut device = Device::new(cartridge);

            if let Some(_) = breakpoint {
                device.breakpoint = *breakpoint;
            }

            Ok(device)
        }
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let mut logger = create_default_logger(&cli);

    logger.info(logging::log::Log::Msg(
        "Starting MyBoy Gameboy Emulator".to_string(),
    ));

    let command = cli.command.clone().unwrap();

    let device = create_default_device(cli)?;

    // // As this is in ogni case eternal
    // let device = Box::new(device);

    match command {
        Commands::Play { headless, .. } => {
            if headless {
                logger.info(logging::log::Log::Msg(
                    "Running device in headless mode".to_string(),
                ));
                let _ = run_device_headless(device)
                    .map_err(|e| format!("Failed to run device in headless mode: {}", e))?;

                Ok(())
            } else {
                let _ = open_gamescreen(device)
                    .map_err(|e| format!("Failed to open game screen: {}", e))?;

                Ok(())
            }
        }
        Commands::Debug { .. } => {
            // open_native_app(device).map_err(|e| format!("Failed to open native app: {}", e))?;

            Ok(())
        }
    }
}

fn run_device_headless(mut device: Device) -> Result<(), String> {
    run_emulator(&mut device)?
        .join()
        .map_err(|e| format!("Failed to run emulator in headless mode: {:?}", e))
}

// fn open_native_app(mut device: Device) -> Result<(), String> {
//     let native_options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default()
//             .with_inner_size([400.0, 300.0])
//             .with_min_inner_size([300.0, 200.0]),
//         ..eframe::NativeOptions::default()
//     };
//     // .with_icon(
//     //     // NOTE: Adding an icon is optional
//     //     eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
//     //         .expect("Failed to load icon"),
//     // ),
//
//     let _ = run_emulator(&mut device)
//         .map_err(|e| format!("Failed to run emulator in debug mode: {}", e))?;
//
//     let app = AppTemplate::new(device);
//
//     eframe::run_native(
//         "MyBoy Gameboy Emulator",
//         native_options,
//         Box::new(|_cc| Ok(Box::new(app))),
//     )
//     .map_err(|e| format!("Failed to run native app: {}", e))
// }
