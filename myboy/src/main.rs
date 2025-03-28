mod cpu;
mod io;
pub mod logger;
mod memory;

use std::{path::Path, thread::sleep, time::Duration};

use cpu::cpu::CPU;
use memory::memory_map::MemoryMap;
use mygbcartridge::cartridge::Cartridge;

fn main() {
    let path = Path::new("/Users/arinaldoni/Downloads/tetris.gb");

    let cartridge = Cartridge::new(path);
    let mut memory = MemoryMap::new(&cartridge);
    let mut cpu = CPU::new(&mut memory);

    println!("Title: {}", cartridge.get_title());
    println!("GBC Support: {}", cartridge.get_gbc_support());
    println!("Logo: {:#?}", cartridge.get_logo());
    println!("Licensee: {}", cartridge.get_licensee().unwrap());
    println!("CartridgeType: {}", cartridge.get_cartridge_type().unwrap());
    println!("ROM Banks: {}", cartridge.get_rom_bank_count());
    println!("ROM Size: {}", cartridge.get_rom_size());
    println!("Valid?: {}", cartridge.has_valid_checksum());

    logger::info!("CPU: {}", cpu);

    loop {
        cpu.execute();
        logger::info!("CPU: {}", cpu);

        sleep(Duration::from_millis(1));
    }
}
