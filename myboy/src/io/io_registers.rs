use crate::memory::generic_memory::OffsetMemory;

use super::{
    ie_register::IERegister,
    if_register::{IFRegister, InterruptType},
    lcdc::LCDCRegister,
};

pub struct IORegisters {
    data: [u8; 256],

    pub ie_register: IERegister,
    pub if_register: IFRegister,
    pub lcdc_register: LCDCRegister,
}

impl IORegisters {
    pub fn new() -> IORegisters {
        let ie_register = IERegister::new();
        let if_register = IFRegister::new();
        let lcdc_register = LCDCRegister::new();

        IORegisters {
            data: [0; 256],
            ie_register,
            if_register,
            lcdc_register,
        }
    }

    pub fn get_timer_div(&self) -> u8 {
        self.read_byte(0xff04)
    }

    pub fn inc_timer_div(&mut self) {
        let current_div = self.read_byte(0xff04);
        match current_div.overflowing_add(1) {
            (new_val, false) => {
                self.write_byte(0xff04, new_val);
            }
            (_, true) => {
                // Reset to 0
                self.write_byte(0xff04, 0);
                self.get_if_register()
                    .request_interrupt(InterruptType::Timer);
            }
        }
        self.write_byte(0xff04, self.read_byte(0xff04).wrapping_add(1));
    }

    pub fn get_lcdl_register(&self) -> LCDCRegister {
        LCDCRegister(self.read_byte(0xff40))
    }

    pub fn get_if_register(&self) -> IFRegister {
        IFRegister(self.read_byte(0xff0f))
    }
    pub fn set_if_register(&mut self, value: IFRegister) {
        self.write_byte(0xff0f, value.0);
    }

    pub fn get_lcd_ly(&self) -> u8 {
        0x90
        // self.read_byte(0xff44)
    }

    pub fn set_lcd_ly(&mut self, line: u8) {
        self.write_byte(0xff44, line);
    }

    pub fn get_lcdstat(&self) -> u8 {
        self.read_byte(0xff41)
    }

    #[inline]
    pub fn read_byte(&self, address: u16) -> u8 {
        if address == 0xff44 {
            return 0x90;
        }
        let translated_address: usize = (address - self.offset() as u16).into();
        self.data[translated_address]
    }

    #[inline]
    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address == 0xff44 {
            return;
        }
        let translated_address: usize = (address - self.offset() as u16).into();
        self.data[translated_address] = value;
    }

    pub fn init_defaults(&mut self) {
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
        self.write_byte(0xff44, 0x90);
        self.write_byte(0xff46, 0xff);
        self.write_byte(0xff47, 0xfc);
    }
}

impl OffsetMemory for IORegisters {
    fn offset(&self) -> usize {
        0xff00
    }
}
