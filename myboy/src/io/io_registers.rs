use crate::memory::generic_memory::OffsetMemory;

use super::{if_register::IFRegister, lcldc_register::LCDCRegister};

pub(crate) struct IORegisters {
    data: [u8; 0xff],
}

impl IORegisters {
    pub fn new() -> IORegisters {
        IORegisters { data: [0; 0xff] }
    }

    pub fn get_lcdl_register(&self) -> LCDCRegister {
        LCDCRegister(self.read_byte(0xff40))
    }

    pub fn set_lcdl_register(&mut self, value: LCDCRegister) {
        self.write_byte(0xff40, value.0);
    }

    pub fn get_if_register(&self) -> IFRegister {
        IFRegister(self.read_byte(0xff0f))
    }
    pub fn set_if_register(&mut self, value: IFRegister) {
        self.write_byte(0xff0f, value.0);
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        println!("Reading from IO register: 0x{:2x}", address);
        let translated_address: usize = (address - self.offset() as u16).into();
        let result = self.data[translated_address];
        println!("Read value: 0x{:2x}", result);
        result
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        println!("Writing to IO register: 0x{:2x}", address);
        let translated_address: usize = (address - self.offset() as u16).into();
        self.data[translated_address] = value;
    }
}

impl OffsetMemory for IORegisters {
    fn offset(&self) -> usize {
        0xff00
    }
}
