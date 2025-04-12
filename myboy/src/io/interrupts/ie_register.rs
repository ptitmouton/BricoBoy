pub struct IERegister(pub u8);

impl IERegister {
    pub fn new() -> IERegister {
        IERegister(0x00)
    }

    pub fn is_vblank_handler_enabled(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    pub fn is_lcd_handler_enabled(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn is_timer_handler_enabled(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub fn is_serial_handler_enabled(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn is_joypad_handler_enabled(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }
}

impl<'a> Into<&'a u8> for &'a IERegister {
    fn into(self) -> &'a u8 {
        &self.0
    }
}

impl<'a> Into<&'a mut u8> for &'a mut IERegister {
    fn into(self) -> &'a mut u8 {
        &mut self.0
    }
}
