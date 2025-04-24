use crate::memory::generic_memory::OffsetMemory;

pub struct Timers {
    pub sys: u16,
    pub last_sys: u16,

    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
}

impl OffsetMemory for Timers {
    fn offset(&self) -> u16 {
        0xff04
    }
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            sys: 0xAB00,
            last_sys: 0xAB00,
            tima: 0x00,
            tma: 0x00,
            tac: 0xf8,
        }
    }

    pub fn get_div_byte(&self) -> u8 {
        (self.sys >> 8) as u8
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xff04 => self.get_div_byte(),
            0xff05 => self.tima,
            0xff06 => self.tma,
            0xff07 => self.tac,
            _ => panic!("Invalid address for Timers: {:#X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xff04 => {
                self.sys = 0x00; // DIV is reset when trying to write to it
            }
            0xff05 => {
                self.tima = value;
            }
            0xff06 => {
                self.tma = value;
            }
            0xff07 => {
                self.tac |= value & 0b0000_1111; // only the lower 3 bits are used
            }
            _ => panic!("Invalid address for Timers: {:#X}", address),
        }
    }

    /**
     * This should be called every 4 cycles (so every m-cycle) by the CPU.
     *
     * When this function returns true, it means that the TIMA register has overflowed.
     * and a timer interrupt should be requested.
     */
    pub fn tick(&mut self) -> bool {
        self.last_sys = self.sys;
        self.sys = self.sys.wrapping_add(1);
        let tac_enabled = self.tac & 0x04 != 0;
        if tac_enabled {
            let tac_clock_divider = self.tac & 0b11;
            match tac_clock_divider {
                0b00 => {
                    // 4096 Hz
                    if self.sys & 0b00000010_00000000 != self.last_sys & 0b00000010_00000000 {
                        return self.inc_tima();
                    }
                }
                0b01 => {
                    // 262144 Hz
                    if self.sys & 0b00000000_00001000 != self.last_sys & 0b00000000_00001000 {
                        return self.inc_tima();
                    }
                }
                0b10 => {
                    // 65536 Hz
                    if self.sys & 0b00000000_00100000 != self.last_sys & 0b00000000_00100000 {
                        return self.inc_tima();
                    }
                }
                0b11 => {
                    // 16384 Hz
                    if self.sys & 0b00000000_10000000 != self.last_sys & 0b00000000_10000000 {
                        return self.inc_tima();
                    }
                }
                _ => unreachable!(),
            }
        }

        false
    }

    pub fn inc_tima(&mut self) -> bool {
        self.tima = self.tima.wrapping_add(1);
        if self.tima == 0x00 {
            // if TIMA overflows, set it to TMA
            self.tima = self.tma;
            return true;
        }
        false
    }
}
