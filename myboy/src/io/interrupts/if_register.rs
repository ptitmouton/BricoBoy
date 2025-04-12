pub enum InterruptType {
    VBlank = 0x01,
    LCDStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

pub struct IFRegister(pub u8);

impl IFRegister {
    pub fn new() -> IFRegister {
        IFRegister(0x00)
    }

    pub fn is_requested(&self, interrupt_type: InterruptType) -> bool {
        self.0 & interrupt_type as u8 != 0
    }

    pub fn request_interrupt(&mut self, interrupt_type: InterruptType) {
        self.0 |= interrupt_type as u8;
    }

    pub fn clear_request(&mut self, interrupt_type: InterruptType) {
        self.0 &= !(interrupt_type as u8);
    }
}

impl<'a> Into<&'a u8> for &'a IFRegister {
    fn into(self) -> &'a u8 {
        &self.0
    }
}

impl<'a> Into<&'a mut u8> for &'a mut IFRegister {
    fn into(self) -> &'a mut u8 {
        &mut self.0
    }
}
