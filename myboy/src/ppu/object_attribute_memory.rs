use crate::memory::generic_memory::OffsetMemory;

pub struct ObjectAttributeMemory {
    data: [u8; 160],
}

impl ObjectAttributeMemory {
    pub fn new() -> ObjectAttributeMemory {
        ObjectAttributeMemory { data: [0; 160] }
    }

    #[inline]
    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        let translated_address: usize = (address - self.offset() as u16).into();
        let result = self.data[translated_address];
        result
    }

    #[inline]
    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        println!("Writing to IO register: 0x{:2x}", address);
        let translated_address: usize = (address - self.offset() as u16).into();
        self.data[translated_address] = value;
    }

    #[inline]
    pub(crate) fn read_word(&self, address: u16) -> u16 {
        let translated_address: usize = (address - self.offset() as u16).into();
        let result = u16::from_le_bytes([
            self.data[translated_address],
            self.data[translated_address + 1],
        ]);
        result
    }

    #[inline]
    pub(crate) fn write_word(&mut self, address: u16, value: u16) {
        let translated_address: usize = (address - self.offset() as u16).into();
        self.data[translated_address] = (value & 0xff) as u8;
        self.data[translated_address + 1] = (value >> 8) as u8;
    }
}

impl OffsetMemory for ObjectAttributeMemory {
    fn offset(&self) -> usize {
        0xfe00
    }
}
