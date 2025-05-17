use fixed_vec_deque::FixedVecDeque;

use crate::{
    device::mem_map::MemMap,
    io::{if_register::InterruptType, io_registers::IORegisters, lcdc::LCDCRegister},
    memory::generic_memory::GenericMemory,
};

use super::oam::{OAM, OAMEntry};

enum RenderMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

enum BGMode {
    Background = 0x00,
    Window = 0x01,
}
impl TryFrom<u16> for BGMode {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BGMode::Background),
            1 => Ok(BGMode::Window),
            _ => Err(String::from("Invalid RenderMode")),
        }
    }
}

pub(crate) struct PPU<'a> {
    last_render_mode: RenderMode,
    current_line_cycle: u16,
    current_x_pos: u8,
    current_window_line: u8,

    sprite_buffer: Vec<OAMEntry<'a>>,
    fifos: (FixedVecDeque<[u8; 16]>, FixedVecDeque<[u8; 16]>),
}

impl<'a> PPU<'a> {
    pub(crate) fn new() -> PPU<'a> {
        PPU {
            last_render_mode: RenderMode::Mode1,
            current_line_cycle: 0,
            current_x_pos: 0,
            current_window_line: 0,

            sprite_buffer: Vec::with_capacity(10),
            // first is OBJ, second is BG/WIN
            fifos: (FixedVecDeque::new(), FixedVecDeque::new()),
        }
    }

    fn goto_next_line(&mut self, io_registers: &mut IORegisters) {
        // end of line, reset x position and go to next line
        self.current_x_pos = 0;
        self.current_line_cycle = 0;
        io_registers.set_lcd_ly((io_registers.get_lcd_ly() + 1) % 154);
    }

    pub(crate) fn cycle(&mut self, mem_map: &'a mut MemMap, screen: &mut [u8]) {
        let current_line = mem_map.io_registers.get_lcd_ly() as usize;
        // println!(
        //     "Current pos: {}, line: {} | current line cycle: {}",
        //     self.current_x_pos, current_line, self.current_line_cycle
        // );

        if current_line >= 144 {
            if let RenderMode::Mode1 = self.last_render_mode {
                self.last_render_mode = RenderMode::Mode0;
                mem_map
                    .io_registers
                    .if_register
                    .request_interrupt(InterruptType::VBlank);
            }
            if self.current_line_cycle == 455 {
                self.goto_next_line(&mut mem_map.io_registers);
            }
        } else if self.current_line_cycle < 80 {
            self.last_render_mode = RenderMode::Mode2;
            self.cycle_mode2(
                &mem_map.object_attribute_memory,
                &mem_map.io_registers.lcdc_register,
                current_line as u8,
            );
        } else if self.current_x_pos < 160 {
            self.last_render_mode = RenderMode::Mode3;
            self.cycle_mode3(mem_map, current_line as u8, screen);
        } else {
            if let RenderMode::Mode3 = self.last_render_mode {
                self.last_render_mode = RenderMode::Mode0;
            }
            if self.current_line_cycle == 455 {
                self.goto_next_line(&mut mem_map.io_registers);
            }
        }

        self.current_line_cycle = self.current_line_cycle.wrapping_add(1)
    }

    pub fn cycle_mode2(&mut self, oam: &'a OAM, lcdc: &LCDCRegister, current_line: u8) {
        if self.sprite_buffer.len() > 9 {
            // sprite buffer is full
            return;
        }
        if self.current_line_cycle & 0b1 == 1 {
            // we loiterers work only on even cycles
            return;
        }

        let sprite_index = (self.current_line_cycle / 2) as usize;
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

    fn cycle_mode3(&mut self, mem_map: &mut MemMap, current_line: u8, screen: &mut [u8]) {
        self.push_sprite_pixel(mem_map, current_line);
        self.push_bgwin_pixel(mem_map, current_line, BGMode::Background);

        self.render_pixel(current_line, screen);
    }

    fn push_sprite_pixel(&mut self, mem_map: &mut MemMap, current_line: u8) {
        let ioregs = &mut mem_map.io_registers;
        let vram = &mem_map.video_ram;
        let lcdc = ioregs.get_lcdc_register();

        if !lcdc.obj_enabled() {
            println!("OBJ not enabled",);
            return;
        }

        let sprite = self.sprite_buffer.iter().find(|sprite| {
            if *sprite.x > (self.current_x_pos + 8) {
                return false;
            }

            return true;
        });

        if let Some(sprite) = sprite {
            if !lcdc.obj_enabled() {
                return;
            }
            let tile_index = *sprite.tile_index as u16;

            let sprite_size = lcdc.sprite_pixel_size() as u16;

            let mut tile_data_addr = 0x8000 + tile_index * 16;
            tile_data_addr += 2 * ((current_line as u16) % sprite_size);

            let tile_data = vram.read_word(tile_data_addr);

            let pixels = Self::bpp_to_pixelrow(tile_data);

            for pixel in pixels.iter() {
                *self.fifos.1.push_back() = *pixel;
            }
        }
    }

    fn push_bgwin_pixel(&mut self, mem_map: &mut MemMap, current_line: u8, render_mode: BGMode) {
        if self.fifos.1.len() != 0 {
            // background fifo is not empty
            return;
        }
        let ioregs = &mut mem_map.io_registers;
        let video_ram = &mem_map.video_ram;
        let lcdc = ioregs.get_lcdc_register();

        let pixel_row_2bpp = match render_mode {
            BGMode::Background => {
                if !lcdc.bgwin_enabled() {
                    None
                } else {
                    let tile_map_addr: u16 = if lcdc.bg_tile_map_bank() == 0 {
                        0x9800
                    } else {
                        0x9C00
                    };

                    let scy = ioregs.get_scy() as u16;

                    let mut tile_index = self.current_x_pos as u16 / 8u16;
                    tile_index += (ioregs.get_scx() / 8) as u16 & 0x1f; // offset by bg-scrollx
                    tile_index += 32 * (((current_line as u16) + scy & 0xff) / 8); // line-offset

                    let tile_number = mem_map.read_byte(tile_map_addr + tile_index) as u16;

                    let mut tile_data_addr = if lcdc.bgwin_tile_data_area() == 0 {
                        if tile_number < 128 {
                            0x9000 + tile_number * 16
                        } else {
                            0x8800 + (tile_number - 128) * 16
                        }
                    } else {
                        0x8000 + tile_number
                    };
                    tile_data_addr += 2 * ((current_line as u16 + scy) % 8);

                    let tile_data = video_ram.read_word(tile_data_addr);

                    Some(tile_data)
                }
            }
            BGMode::Window => {
                // TODO
                None
            }
        };

        if pixel_row_2bpp.is_none() {
            return;
        }

        let pixels = Self::bpp_to_pixelrow(pixel_row_2bpp.unwrap());

        for pixel in pixels.iter() {
            *self.fifos.1.push_back() = *pixel;
        }
    }

    fn bpp_to_pixelrow(bpp_data: u16) -> [u8; 8] {
        let [lower_byte, higher_byte] = bpp_data.to_le_bytes();

        [
            ((higher_byte & 0b1000_0000) >> 6) | ((lower_byte & 0b1000_0000) >> 7),
            ((higher_byte & 0b0100_0000) >> 5) | ((lower_byte & 0b0100_0000) >> 6),
            ((higher_byte & 0b0010_0000) >> 4) | ((lower_byte & 0b0010_0000) >> 5),
            ((higher_byte & 0b0001_0000) >> 3) | ((lower_byte & 0b0001_0000) >> 4),
            ((higher_byte & 0b0000_1000) >> 2) | ((lower_byte & 0b0000_1000) >> 3),
            ((higher_byte & 0b0000_0100) >> 1) | ((lower_byte & 0b0000_0100) >> 2),
            (higher_byte & 0b0000_0010) | ((lower_byte & 0b0000_0010) >> 1),
            ((higher_byte & 0b0000_0001) << 1) | (lower_byte & 0b0000_0001),
        ]
    }

    fn bpp_to_rgba(color: u8) -> (u8, u8, u8, u8) {
        match color {
            0 => (0xff, 0xff, 0xff, 0xff), // white
            1 => (0xab, 0xab, 0xab, 0xff), // black
            2 => (0x60, 0x60, 0x60, 0xff), // red
            3 => (0x00, 0x00, 0x00, 0xff), // green
            _ => (0x00, 0x00, 0x00, 0xff), // default to black
        }
    }

    fn render_pixel(&mut self, y: u8, screen: &mut [u8]) {
        let x = self.current_x_pos;
        let bg_color = self.fifos.1.pop_front();

        if bg_color.is_some() {
            // println!("Fifo length: {}", self.fifos.1.len());
            // TODO: Make better over cycles
            // self.current_line_cycle += self.fifos.1.len() as u16;

            let mut pixel_color = *bg_color.unwrap();
            if let Some(sprite_pixel) = self.fifos.0.pop_front() {
                if *sprite_pixel != 0 && (/* priority for gbc would go here */true) {
                    pixel_color = *sprite_pixel;
                }
            }

            let (r, g, b, a) = Self::bpp_to_rgba(pixel_color);

            // println!(
            //     "Rendering pixel at ({}, {}) with color: {} (= {}, {}, {}, {})",
            //     x, y, pixel_color, r, g, b, a
            // );

            let screen_pixel_index = (y as usize * 160 + x as usize) * 4; // Assuming 4 bytes per pixel (RGBA)
            screen[screen_pixel_index] = r;
            screen[screen_pixel_index + 1] = g;
            screen[screen_pixel_index + 2] = b;
            screen[screen_pixel_index + 3] = a;
        }
        self.current_x_pos += 1;
    }
}
