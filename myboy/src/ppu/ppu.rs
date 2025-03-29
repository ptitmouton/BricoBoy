use crate::device::mem_map::MemMap;

pub(crate) struct PPU {}

impl PPU {
    pub(crate) fn new() -> PPU {
        PPU {}
    }

    pub(crate) fn cycle(&mut self, mem_map: &mut MemMap) {
        let current_line = mem_map.io_registers.get_lcd_ly() + 1;
        if (current_line as u16) == 153 {
            mem_map.io_registers.set_lcd_ly(0);
            return;
        }
        if (current_line as u16) >= 144 {
            if current_line == 144 {
                // trigger vblank
            }
            self.draw_line(mem_map, current_line, 1);
            return;
        }
        if (current_line as u16) < 144 {
            let lcdstat = mem_map.io_registers.get_lcdstat();
            let mode = lcdstat & 0b0000_0011;
            let next_mode = match mode {
                0 => 2,
                2 => 3,
                3 => 0,
                1 => 0,
                _ => panic!("Invalid mode"),
            };
            self.draw_line(mem_map, current_line, next_mode);
        }
    }

    pub(crate) fn draw_line(&mut self, mem_map: &mut MemMap, line: u8, mode: u8) {
        println!("Drawing line {}", line);

        let lcdstat = mem_map.io_registers.get_lcdstat();
        mem_map
            .io_registers
            .set_lcdstat(lcdstat & 0b1111_1100 | mode);
    }
}
