pub(crate) struct IERegister(pub u8);

impl IERegister {
    pub(crate) fn new() -> IERegister {
        IERegister(0)
    }

    pub(crate) fn is_vblank_handler_enabled(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    pub(crate) fn is_lcd_handler_enabled(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub(crate) fn is_timer_handler_enabled(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub(crate) fn is_serial_handler_enabled(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub(crate) fn is_joypad_handler_enabled(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }
}

impl Into<u8> for IERegister {
    fn into(self) -> u8 {
        self.0
    }
}
