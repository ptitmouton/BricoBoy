pub(crate) struct IFRegister(pub u8);

impl IFRegister {
    pub(crate) fn is_vblank(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    pub(crate) fn is_lcd_stat(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub(crate) fn is_timer(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub(crate) fn is_serial(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub(crate) fn is_joypad(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }
}

impl Into<u8> for IFRegister {
    fn into(self) -> u8 {
        self.0
    }
}
