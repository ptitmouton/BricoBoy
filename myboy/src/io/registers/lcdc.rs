pub struct LCDCRegister(pub u8);

impl LCDCRegister {
    pub fn new() -> LCDCRegister {
        LCDCRegister(0x00)
    }

    pub fn lcd_enabled(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    pub fn window_tile_map_bank(&self) -> u8 {
        self.0 & 0b0100_0000
    }

    pub fn window_enabled(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    pub fn bg_tile_map_bank(&self) -> u8 {
        self.0 & 0b0001_0000
    }

    pub fn bg_tile_data_bank(&self) -> u8 {
        self.0 & 0b0000_1000
    }

    pub fn obj_size(&self) -> u8 {
        self.0 & 0b0000_0100
    }

    pub fn obj_enabled(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn bg_enabled(&self) -> bool {
        self.0 & 0b0000_0001 != 0
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
