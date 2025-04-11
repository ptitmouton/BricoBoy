mod cli;
mod cpu;
mod device;
mod io;
mod logging;
mod memory;
mod ppu;
mod ui;

use clap::Parser;
use cli::args::{Cli, Commands, Output};
use device::device::Device;
use logging::log::{ConsoleLogger, InMemoryLogger, Logger};
use mygbcartridge::cartridge::Cartridge;
use ppu::ppu::PPU;
use ui::{app::AppTemplate, emulator_view::run_emulator};

fn create_default_logger(cli: &Cli) -> Box<dyn Logger> {
    if cli.dbg_view.is_none_or(|v| v == true) {
        Box::new(InMemoryLogger::default())
    } else {
        Box::new(ConsoleLogger::default())
    }
}

fn create_device(
    cli: &Cli,
    default_logger: &'static mut dyn Logger,
) -> Option<&'static mut Device> {
    let mut result: Option<&'static mut Device> = None;
    match &cli.command {
        Some(Commands::Run {
            breakpoint,
            file,
            output,
        }) => {
            if let Some(path) = file {
                let cartridge = Cartridge::new(path);
                let mut dev = Box::new(Device::new(cartridge));
                // println!("Cartridge loaded: {:?}", dev.cartridge.get_title());
                if let Some(bp) = breakpoint {
                    dev.breakpoint = Some(*bp);
                }

                match output {
                    Some(Output::Serial) => dev.serial_logger = Some(default_logger),
                    Some(Output::CPUStates) => dev.cpu_logger = Some(default_logger),
                    None => {}
                }

                result.replace(Box::leak(dev));
            }
        }
        None => {
            panic!("No command provided");
        }
    };

    result
}

fn main() {
    let cli = Cli::parse();

    let default_logger = create_default_logger(&cli);

    let default_logger: &'static mut dyn Logger = Box::leak(default_logger);
    // default_logger.info(logging::log::Log::Msg(
    //     "Starting MyBoy Gameboy Emulator".to_string(),
    // ));

    let _ = run(&cli, create_device(&cli, default_logger));
}

fn run(cli: &Cli, emulator: Option<&'static mut Device>) -> eframe::Result {
    if cli.dbg_view.is_none_or(|v| v == true) {
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
            Box::new(|_cc| Ok(Box::new(AppTemplate::new(emulator)))),
        )
    } else {
        match run_emulator(emulator.unwrap()) {
            Err(msg) => panic!("{}", msg),
            Ok(handle) => {
                handle.join().unwrap();

                Result::Ok(())
            }
        }
    }
}
