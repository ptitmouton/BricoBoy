pub enum InterruptType {
    VBlank = 0x01,
    LCDStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

pub struct IFRegister(pub u8);

#[inline(always)]
pub fn get_handler_address(interrupt_type: InterruptType) -> u16 {
    match interrupt_type {
        InterruptType::VBlank => 0x0040,
        InterruptType::LCDStat => 0x0048,
        InterruptType::Timer => 0x0050,
        InterruptType::Serial => 0x0058,
        InterruptType::Joypad => 0x0060,
    }
}

impl IFRegister {
    pub fn new() -> IFRegister {
        IFRegister(0xe1)
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

    pub fn read_byte(&self) -> u8 {
        self.0
    }

    pub fn write_byte(&mut self, value: u8) {
        self.0 = value;
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
