// 0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
// 4000	7FFF	16 KiB ROM Bank 01–NN	From cartridge, switchable bank via mapper (if any)
// 8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
// A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
// C000	CFFF	4 KiB Work RAM (WRAM)
// D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1–7
// E000	FDFF	Echo RAM (mirror of C000–DDFF)	Nintendo says use of this area is prohibited.
// FE00	FE9F	Object attribute memory (OAM)
// FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited.
// FF00	FF7F	I/O Registers
// FF80	FFFE	High RAM (HRAM)
// FFFF	FFFF	Interrupt Enable register (IE)

use crate::{
    io::io::IO,
    memory::generic_memory::{GenericRam, ReadableMemory, WritableMemory},
};
use mygbcartridge::cartridge::Cartridge;

pub struct MemoryMap {
    pub cartridge: Cartridge,
    pub working_ram: GenericRam,
    pub video_ram: GenericRam,
    pub io: IO,
}

impl MemoryMap {
    pub fn new(cartridge: &Cartridge) -> MemoryMap {
        let cartridge = cartridge.clone();
        let io = IO::new();
        let working_ram = GenericRam::create(0x2000, 0xc000);
        let video_ram = GenericRam::create(0x2000, 0x8000);

        MemoryMap {
            cartridge,
            working_ram,
            video_ram,
            io,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_byte(address),
            0x8000..=0x9FFF => self.video_ram.read_byte(address),
            0xA000..=0xBFFF => todo!("Read from External RAM"),
            0xC000..=0xDFFF => self.working_ram.read_byte(address),
            0xE000..=0xFDFF => todo!("Read from Echo RAM"),
            0xFE00..=0xFE9F => todo!("Read from OAM"),
            0xFEA0..=0xFEFF => todo!("Read from Not Usable"),
            0xFF00..=0xFF7F => self.io.read_byte(address),
            0xFF80..=0xFFFE => todo!("Read from High RAM"),
            0xFFFF => self.io.read_byte(address),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_word(address),
            0x8000..=0x9FFF => self.video_ram.read_word(address),
            0xA000..=0xBFFF => todo!("Read from External RAM"),
            0xC000..=0xDFFF => self.working_ram.read_word(address),
            0xE000..=0xFDFF => todo!("Read from Echo RAM"),
            0xFE00..=0xFE9F => todo!("Read from OAM"),
            0xFEA0..=0xFEFF => todo!("Read from Not Usable"),
            0xFF00..=0xFF7F => panic!("Cannot read word from I/O registers"),
            0xFF80..=0xFFFE => todo!("Read from High RAM"),
            0xFFFF => todo!("Read from Interrupt Enable register"),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        println!(
            "Write to memory map: Address: {:#06x}, Value: {:#04x}",
            address, value
        );
        match address {
            0x8000..=0x9FFF => self.video_ram.write_byte(address, value),
            0xC000..=0xDFFF => self.working_ram.write_byte(address, value),
            0xFF00..=0xFF7F => self.io.write_byte(address, value),
            0xFFFF => self.io.write_byte(address, value),
            _ => todo!("Write to memory map"),
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        println!(
            "Write to memory map: Address: {:#06x}, Value: {:#04x}",
            address, value
        );
        match address {
            0x8000..=0x9FFF => self.video_ram.write_word(address, value),
            0xC000..=0xDFFF => self.working_ram.write_word(address, value),
            0xFF00..=0xFF7F => panic!("Cannot write word to I/O registers"),
            _ => todo!("Write to memory map"),
        }
    }

    pub fn io(&self) -> &IO {
        &self.io
    }
}
