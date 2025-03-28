pub(crate) struct LCDCRegister(pub u8);

impl LCDCRegister {
    pub(crate) fn lcd_enabled(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    pub(crate) fn window_tile_map_bank(&self) -> u8 {
        self.0 & 0b0100_0000
    }

    pub(crate) fn window_enabled(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    pub(crate) fn bg_tile_map_bank(&self) -> u8 {
        self.0 & 0b0001_0000
    }

    pub(crate) fn bg_tile_data_bank(&self) -> u8 {
        self.0 & 0b0000_1000
    }

    pub(crate) fn obj_size(&self) -> u8 {
        self.0 & 0b0000_0100
    }

    pub(crate) fn obj_enabled(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub(crate) fn bg_enabled(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }
}

impl Into<u8> for LCDCRegister {
    fn into(self) -> u8 {
        self.0
    }
}
