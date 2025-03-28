use crate::memory::generic_memory::{ReadableMemory, WritableMemory};

pub(crate) struct IO {
    interrupts_flags: u8,
    interrupts_enables: u8,
}

impl ReadableMemory for IO {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF0F => self.interrupts_flags,
            0xFFFF => self.interrupts_enables,
            _ => unimplemented!(),
        }
    }

    fn read_fixed_bytes<const L: usize>(&self, address: u16, length: usize) -> &[u8; L] {
        unimplemented!()
    }

    fn read_word(&self, address: u16) -> u16 {
        unimplemented!()
    }
}
impl WritableMemory for IO {
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF0F => self.set_interrupt_flags(value),
            0xFFFF => self.set_interrupt_enables(value),
            _ => unimplemented!(),
        }
    }

    fn write_bytes(&mut self, address: u16, value: &[u8]) {
        unimplemented!()
    }

    fn write_word(&mut self, address: u16, value: u16) {
        unimplemented!()
    }
}

enum InterruptFlag {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

impl IO {
    pub(crate) fn new() -> IO {
        IO {
            interrupts_flags: 0,
            interrupts_enables: 0,
        }
    }

    pub(crate) fn set_interrupt_flags(&mut self, flags: u8) {
        self.interrupts_flags |= flags;
    }

    pub(crate) fn get_interrupt_flag(&mut self, flag: InterruptFlag) -> bool {
        let mask = match flag {
            InterruptFlag::VBlank => 0b0000_0001,
            InterruptFlag::LCDStat => 0b0000_0010,
            InterruptFlag::Timer => 0b0000_0100,
            InterruptFlag::Serial => 0b0000_1000,
            InterruptFlag::Joypad => 0b0001_0000,
        };

        (self.interrupts_flags & mask) != 0x0
    }
    pub(crate) fn set_interrupt_flag(&mut self, flag: InterruptFlag, value: bool) {
        let mask = match flag {
            InterruptFlag::VBlank => 0b0000_0001,
            InterruptFlag::LCDStat => 0b0000_0010,
            InterruptFlag::Timer => 0b0000_0100,
            InterruptFlag::Serial => 0b0000_1000,
            InterruptFlag::Joypad => 0b0001_0000,
        };

        if value {
            self.interrupts_flags |= mask;
        } else {
            self.interrupts_flags &= !mask;
        }
    }

    pub(crate) fn set_interrupt_enables(&mut self, enables: u8) {
        self.interrupts_enables |= enables;
    }

    pub(crate) fn get_interrupt_enable(&mut self, enable: InterruptFlag) -> bool {
        let mask = match enable {
            InterruptFlag::VBlank => 0b0000_0001,
            InterruptFlag::LCDStat => 0b0000_0010,
            InterruptFlag::Timer => 0b0000_0100,
            InterruptFlag::Serial => 0b0000_1000,
            InterruptFlag::Joypad => 0b0001_0000,
        };

        (self.interrupts_enables & mask) != 0x0
    }
    pub(crate) fn set_interrupt_enable(&mut self, enable: InterruptFlag, value: bool) {
        let mask = match enable {
            InterruptFlag::VBlank => 0b0000_0001,
            InterruptFlag::LCDStat => 0b0000_0010,
            InterruptFlag::Timer => 0b0000_0100,
            InterruptFlag::Serial => 0b0000_1000,
            InterruptFlag::Joypad => 0b0001_0000,
        };

        if value {
            self.interrupts_enables |= mask;
        } else {
            self.interrupts_enables &= !mask;
        }
    }
}
