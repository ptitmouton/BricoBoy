pub struct LCDCRegister(pub u8);

impl LCDCRegister {
    pub fn new() -> LCDCRegister {
        LCDCRegister(0x91)
    }

    pub fn lcd_enabled(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    /**
     * Returns 0 (9800–9BFF) or 1 (9C00–9FFF)
     */
    pub fn window_tile_map_bank(&self) -> u8 {
        (self.0 & 0b0100_0000) >> 6
    }

    pub fn window_enabled(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    /**
     * Returns 0 (8800–97FF) or 1 (8000–8FFF)
     */
    pub fn bgwin_tile_data_area(&self) -> u8 {
        (self.0 & 0b0001_0000) >> 4
    }

    /**
     * Returns 0 (9800–9BFF) or 1 (9C00–9FFF)
     */
    pub fn bg_tile_map_bank(&self) -> u8 {
        (self.0 & 0b0000_1000) >> 3
    }

    #[inline]
    pub fn sprite_pixel_size(&self) -> u8 {
        let multiplicator = (self.0 & 0b0000_0100 >> 2) + 1;
        multiplicator * 8
    }

    /*
     * Returns 0 (8x8) or 1 (8x16)
     */
    pub fn obj_size(&self) -> u8 {
        (self.0 & 0b0000_0100) >> 2
    }

    pub fn obj_enabled(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn bgwin_enabled(&self) -> bool {
        (self.0 & 0b0000_0001) != 0
    }
}

impl<'a> Into<&'a u8> for &'a LCDCRegister {
    fn into(self) -> &'a u8 {
        &self.0
    }
}

impl<'a> Into<&'a mut u8> for &'a mut LCDCRegister {
    fn into(self) -> &'a mut u8 {
        &mut self.0
    }
}
