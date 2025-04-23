use crate::memory::generic_memory::OffsetMemory;

use super::{
    ie_register::IERegister,
    if_register::{IFRegister, InterruptType},
    lcdc::LCDCRegister,
    timers::Timers,
};

pub struct IORegisters {
    data: [u8; 256],

    pub ie_register: IERegister,
    pub if_register: IFRegister,
    pub lcdc_register: LCDCRegister,
    pub timers: Timers,
}

impl IORegisters {
    pub fn new() -> IORegisters {
        let data = Self::default_data();

        let ie_register = IERegister::new();
        let if_register = IFRegister::new();
        let lcdc_register = LCDCRegister::new();
        let timers = Timers::new();

        IORegisters {
            data,
            ie_register,
            if_register,
            lcdc_register,
            timers,
        }
    }

    /*
     * Reports that an mcycle has passed.
     * This is used to update the DIV register and
     * the TIMA.
     * Every 64 mcycles, the DIV register is incremented
     * See https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
     * for details on how the timers work
     */
    pub fn update_timers(&mut self) {
        // true means it's time for a timer interrupt
        if self.timers.tick() == true {
            self.if_register.request_interrupt(InterruptType::Timer);
        }
    }

    pub fn get_lcdl_register(&self) -> LCDCRegister {
        LCDCRegister(self.read_byte(0xff40))
    }

    pub fn set_if_register(&mut self, value: u8) {
        self.if_register.0 = value & 0x1f;
    }

    pub fn get_lcd_ly(&self) -> u8 {
        self.data[0x44]
    }

    pub fn set_lcd_ly(&mut self, line: u8) {
        self.data[0x44] = line;
    }

    pub fn get_lcdstat(&self) -> u8 {
        self.read_byte(0xff41)
    }

    #[inline]
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            // TODO: ff01 and ff02 are the serial registers
            0xff04 | 0xff05 | 0xff06 | 0xff07 => return self.timers.read_byte(address),
            0xff0f => return self.if_register.read_byte(),
            0xffff => return self.ie_register.read_byte(),
            _ => {
                let translated_address: usize = (address - self.offset() as u16).into();
                self.data[translated_address]
            }
        }
    }

    #[inline]
    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address == 0xff44 {
            return;
        }
        match address {
            // TODO: ff01 and ff02 are the serial registers
            0xff04 | 0xff05 | 0xff06 | 0xff07 => return self.timers.write_byte(address, value),
            0xff0f => return self.if_register.write_byte(value),
            0xffff => return self.ie_register.write_byte(value),
            0xff44 => {
                // LY register is read-only
                // This is just for now ...
                return;
            }
            _ => {
                let translated_address: usize = (address - self.offset() as u16).into();
                self.data[translated_address] = value;
            }
        }
    }

    pub fn default_data() -> [u8; 256] {
        let mut data = [0; 256];
        data[0x00] = 0xcf;
        data[0x02] = 0x7e;
        data[0x0f] = 0xe1;
        data[0x10] = 0x80;
        data[0x11] = 0xbf;
        data[0x12] = 0xf3;
        data[0x13] = 0xff;
        data[0x14] = 0xbf;
        data[0x16] = 0x3f;
        data[0x18] = 0xff;
        data[0x19] = 0xbf;
        data[0x1a] = 0x7f;
        data[0x1b] = 0xff;
        data[0x1c] = 0x9f;
        data[0x1d] = 0xff;
        data[0x1e] = 0xbf;
        data[0x20] = 0xff;
        data[0x23] = 0xbf;
        data[0x24] = 0x77;
        data[0x25] = 0xf3;
        data[0x26] = 0xf1;
        data[0x40] = 0x91;
        data[0x41] = 0x81;
        data[0x44] = 0x90;
        data[0x46] = 0xff;
        data[0x47] = 0xfc;

        data
    }
}

impl OffsetMemory for IORegisters {
    fn offset(&self) -> usize {
        0xff00
    }
}
