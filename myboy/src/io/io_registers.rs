use crate::memory::generic_memory::OffsetMemory;

use super::{if_register::IFRegister, lcldc_register::LCDCRegister};

pub(crate) struct IORegisters {
    data: [u8; 256],
}

impl IORegisters {
    pub fn new() -> IORegisters {
        IORegisters { data: [0; 256] }
    }

    pub fn get_timer_div(&self) -> u8 {
        self.read_byte(0xff04)
    }

    pub fn inc_timer_div(&mut self) {
        println!("Incrementing timer divider");
        self.write_byte(0xff04, self.read_byte(0xff04).wrapping_add(1));
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

    pub fn get_lcd_ly(&self) -> u8 {
        self.read_byte(0xff44)
    }

    pub fn set_lcd_ly(&mut self, line: u8) {
        self.write_byte(0xff44, line);
    }

    pub fn get_lcdstat(&self) -> u8 {
        self.read_byte(0xff41)
    }

    pub fn set_lcdstat(&mut self, lcdstat: u8) {
        self.write_byte(0xff41, lcdstat);
    }

    #[inline]
    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        let translated_address: usize = (address - self.offset() as u16).into();
        let result = self.data[translated_address];
        // println!("Read value from: 0x{:4x} ==> 0x{:2x}", address, result);
        result
    }

    #[inline]
    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        println!("Writing to IO register: 0x{:2x}", address);
        let translated_address: usize = (address - self.offset() as u16).into();
        self.data[translated_address] = value;
    }

    pub(crate) fn init_defaults(&mut self) {
        self.write_byte(0xff00, 0xcf);
        self.write_byte(0xff02, 0x7e);
        self.write_byte(0xff04, 0x18);
        self.write_byte(0xff07, 0xf8);
        self.write_byte(0xff0f, 0xe1);
        self.write_byte(0xff10, 0x80);
        self.write_byte(0xff11, 0xbf);
        self.write_byte(0xff12, 0xf3);
        self.write_byte(0xff13, 0xff);
        self.write_byte(0xff14, 0xbf);
        self.write_byte(0xff16, 0x3f);
        self.write_byte(0xff18, 0xff);
        self.write_byte(0xff19, 0xbf);
        self.write_byte(0xff1a, 0x7f);
        self.write_byte(0xff1b, 0xff);
        self.write_byte(0xff1c, 0x9f);
        self.write_byte(0xff1d, 0xff);
        self.write_byte(0xff1e, 0xbf);
        self.write_byte(0xff20, 0xff);
        self.write_byte(0xff23, 0xbf);
        self.write_byte(0xff24, 0x77);
        self.write_byte(0xff25, 0xf3);
        self.write_byte(0xff26, 0xf1);
        self.write_byte(0xff40, 0x91);
        self.write_byte(0xff41, 0x81);
        self.write_byte(0xff44, 0x91);
        self.write_byte(0xff46, 0xff);
        self.write_byte(0xff47, 0xfc);
    }
}

impl OffsetMemory for IORegisters {
    fn offset(&self) -> usize {
        0xff00
    }
}
