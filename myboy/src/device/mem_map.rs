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
    io::io_registers::IORegisters,
    memory::generic_memory::{RWMemory, ReadableMemory, WritableMemory},
};
use mygbcartridge::cartridge::Cartridge;

pub struct MemMap {
    pub cartridge: Option<Cartridge>,
    pub working_ram: RWMemory,
    pub video_ram: RWMemory,
    pub io_registers: IORegisters,
    pub hram: RWMemory,
    pub interrupts: u8,
}

impl MemMap {
    pub fn new() -> MemMap {
        let working_ram = RWMemory::create(0x2000, 0xc000);
        let video_ram = RWMemory::create(0x2000, 0x8000);
        let io_registers = IORegisters::new();
        let hram = RWMemory::create(0x7f, 0xff80);
        let interrupts = 0x00;

        MemMap {
            cartridge: None,
            working_ram,
            video_ram,
            io_registers,
            hram,
            interrupts,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.as_ref().unwrap().read_byte(address),
            0x8000..=0x9FFF => self.video_ram.read_byte(address),
            0xA000..=0xBFFF => todo!("Read from External RAM"),
            0xC000..=0xDFFF => self.working_ram.read_byte(address),
            0xE000..=0xFDFF => todo!("Read from Echo RAM"),
            0xFE00..=0xFE9F => todo!("Read from OAM"),
            0xFEA0..=0xFEFF => todo!("Read from Not Usable"),
            0xFF00..=0xFF7F => self.io_registers.read_byte(address),
            0xFF80..=0xFFFE => self.hram.read_byte(address),
            0xFFFF => self.interrupts,
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        match address {
            0x0000..=0x7FFF => self.cartridge.as_ref().unwrap().read_word(address),
            0x8000..=0x9FFF => self.video_ram.read_word(address),
            0xA000..=0xBFFF => todo!("Read from External RAM"),
            0xC000..=0xDFFF => self.working_ram.read_word(address),
            0xE000..=0xFDFF => todo!("Read from Echo RAM"),
            0xFE00..=0xFE9F => todo!("Read from OAM"),
            0xFEA0..=0xFEFF => todo!("Read from Not Usable"),
            0xFF00..=0xFF7F => panic!("Cannot read word from I/O registers"),
            0xFF80..=0xFFFE => self.hram.read_word(address),
            0xFFFF => panic!("Cannot read words from interrupts"),
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
            0xFF00..=0xFF7F => self.io_registers.write_byte(address, value),
            0xFF80..=0xFFFE => self.hram.write_byte(address, value),
            0xFFFF => {
                self.interrupts = value;
            }
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
            0xFF80..=0xFFEF => self.hram.write_word(address, value),
            0xFFFF => panic!("Cannot write word to interrupts"),
            _ => todo!("Write to memory map"),
        }
    }

    pub fn set_cartridge(&mut self, cartridge: &Cartridge) {
        self.cartridge = Some(cartridge.clone());
    }
}
