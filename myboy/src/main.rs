mod cpu;
mod device;
mod io;
mod memory;
mod ppu;
mod ui;
use ppu::ppu::PPU;
use ui::app::AppTemplate;

fn main() {
    run().expect("Failed to run eframe");
}

fn run() -> eframe::Result {
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
        Box::new(|_cc| Ok(Box::new(AppTemplate::default()))),
    )
}
