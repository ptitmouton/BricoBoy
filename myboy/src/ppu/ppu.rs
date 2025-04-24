use crate::{
    device::mem_map::MemMap,
    io::{if_register::InterruptType, lcdc::LCDCRegister},
};

use super::oam::{OAM, OAMEntry};

pub(crate) struct PPU<'a> {
    current_line_cycle: u16,
    sprite_buffer: Vec<OAMEntry<'a>>,
    fifos: (Vec<u8>, Vec<u8>),
}

impl<'a> PPU<'a> {
    pub(crate) fn new() -> PPU<'a> {
        PPU {
            current_line_cycle: 0,
            sprite_buffer: Vec::with_capacity(10),
            fifos: (Vec::with_capacity(8), Vec::with_capacity(8)),
        }
    }

    pub(crate) fn cycle(&mut self, mem_map: &'a mut MemMap, _screen: &mut [u8]) {
        let current_line = mem_map.io_registers.get_lcd_ly() as usize;

        if self.current_line_cycle < 80 {
            self.cycle_mode2(
                &mem_map.object_attribute_memory,
                &mem_map.io_registers.lcdc_register,
                current_line as u8,
                self.current_line_cycle,
            );
        } else if self.current_line_cycle < 252 {
            // self.cycle_mode3(mem_map, screen, current_line as u8);
        } else {
            // mode 0
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
            // let screen_bindex = 4 * (current_line * 160 + self.current_column as usize);
            // screen[screen_bindex] = 0xAA;
            // screen[screen_bindex + 1] = 0xAA;
            // screen[screen_bindex + 2] = 0xAA;
            // screen[screen_bindex + 3] = 0xFF; // A
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

    pub fn cycle_mode2(
        &mut self,
        oam: &'a OAM,
        lcdc: &LCDCRegister,
        current_line: u8,
        current_line_cycle: u16,
    ) {
        if self.sprite_buffer.len() > 9 {
            // sprite buffer is full
            return;
        }
        if current_line_cycle & 0b1 == 1 {
            // we loiterers work only on even cycles
            return;
        }

        let sprite_index = (current_line_cycle / 2) as usize;
        let sprite = oam.get_sprite(sprite_index);
        let sprite_height = lcdc.sprite_pixel_size();

        if *sprite.x == 0 /* sprite is not visible */
        || current_line + 16 >= *sprite.y /* sprite is not visible */
        || current_line + 16 <= *sprite.y + sprite_height
        /* sprite is not visible */
        {
            return;
        }

        self.sprite_buffer.push(sprite);
    }
}
