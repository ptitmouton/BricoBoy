use crate::{device::mem_map::MemMap, io::if_register::InterruptType};

pub(crate) struct PPU {
    current_column: u8,
}

impl PPU {
    pub(crate) fn new() -> PPU {
        PPU { current_column: 0 }
    }

    pub(crate) fn cycle(&mut self, mem_map: &mut MemMap, screen: &mut [u8]) {
        let mut current_line = mem_map.io_registers.get_lcd_ly() as usize;
        self.current_column += 1;
        if self.current_column >= 160 {
            self.current_column = 0;
            current_line += 1;
            mem_map.io_registers.set_lcd_ly(current_line as u8);
        }
        if current_line == 153 {
            // got to the end
            mem_map.io_registers.set_lcd_ly(0);
            return;
        }
        if current_line >= 144 {
            if current_line == 144 {
                mem_map
                    .io_registers
                    .if_register
                    .request_interrupt(InterruptType::VBlank);
            }
            if current_line == 160 {
                mem_map
                    .io_registers
                    .if_register
                    .clear_request(InterruptType::VBlank);
            }
            // in vblank
            // do something or so, I have no idea
            return;
        }
        if current_line < 144 {
            let lcdstat = mem_map.io_registers.get_lcdstat();
            let _mode = lcdstat & 0b11;
            let screen_bindex = 4 * (current_line * 160 + self.current_column as usize);
            screen[screen_bindex] = 0xAA;
            screen[screen_bindex + 1] = 0xAA;
            screen[screen_bindex + 2] = 0xAA;
            screen[screen_bindex + 3] = 0xFF; // A
            // let next_mode = match mode {
            //     0 => 2,
            //     2 => 3,
            //     3 => 0,
            //     1 => 0,
            //     _ => panic!("Invalid mode"),
            // };
            // draw something or so
        }
    }
}
