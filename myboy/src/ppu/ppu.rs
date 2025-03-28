use std::{cell::RefCell, rc::Rc, thread::sleep};

use crate::{cpu::cpu::T_CYCLE_LENGTH, device::mem_map::MemMap};

pub(crate) struct PPU {
    mem_map: Rc<RefCell<MemMap>>,
}

impl PPU {
    pub(crate) fn new(mem_map: Rc<RefCell<MemMap>>) -> PPU {
        PPU { mem_map }
    }

    pub(crate) fn enabled(&self) -> bool {
        self.mem_map
            .borrow()
            .io_registers
            .get_lcdl_register()
            .lcd_enabled()
    }

    pub(crate) fn cycle(&self) {
        let current_line = self.mem_map.borrow().io_registers.get_lcd_ly() + 1;
        if (current_line as u16) == 153 {
            self.mem_map.borrow_mut().io_registers.set_lcd_ly(0);
            sleep(T_CYCLE_LENGTH);
            return;
        }
        if (current_line as u16) >= 144 {
            if current_line == 144 {
                // trigger vblank
            }
            self.draw_line(current_line, 1);
            sleep(T_CYCLE_LENGTH);
            return;
        }
        if (current_line as u16) < 144 {
            let lcdstat = self.mem_map.borrow().io_registers.get_lcdstat();
            let mode = lcdstat & 0b0000_0011;
            let next_mode = match mode {
                0 => 2,
                2 => 3,
                3 => 0,
                1 => 0,
                _ => panic!("Invalid mode"),
            };
            self.draw_line(current_line, next_mode);
            sleep(T_CYCLE_LENGTH);
        }
    }

    pub(crate) fn draw_line(&self, line: u8, mode: u8) {
        println!("Drawing line {}", line);
        let mut mem_map = self.mem_map.borrow_mut();
        mem_map.io_registers.set_lcd_ly(line);
        let lcdstat = mem_map.io_registers.get_lcdstat();
        mem_map
            .io_registers
            .set_lcdstat(lcdstat & 0b1111_1100 | mode);
        sleep(T_CYCLE_LENGTH);
    }
}
