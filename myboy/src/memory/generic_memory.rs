pub trait ReadableMemory<const SIZE: usize> {
    fn read_byte(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
}

pub trait WritableMemory<const SIZE: usize> {
    fn write_byte(&mut self, address: u16, value: u8);
    fn write_word(&mut self, address: u16, value: u16);
}

pub trait OffsetMemory {
    fn offset(&self) -> u16;
    fn map_address(&self, address: u16) -> u16 {
        address - self.offset()
    }
}

impl<const SIZE: usize> OffsetMemory for [u8; SIZE] {
    fn offset(&self) -> u16 {
        0
    }
}

impl<const SIZE: usize> ReadableMemory<SIZE> for [u8; SIZE]
where
    Self: OffsetMemory,
{
    fn read_byte(&self, address: u16) -> u8 {
        let mapped_address = self.map_address(address);
        self[mapped_address as usize]
    }

    fn read_word(&self, address: u16) -> u16 {
        let mapped_address = self.map_address(address);
        return u16::from_le_bytes(
            self[(mapped_address as usize)..(mapped_address as usize) + 2]
                .try_into()
                .unwrap(),
        );
    }
}

impl<const SIZE: usize> WritableMemory<SIZE> for [u8; SIZE]
where
    Self: OffsetMemory,
{
    fn write_byte(&mut self, address: u16, value: u8) {
        let mapped_address = self.map_address(address);
        self[mapped_address as usize] = value;
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let mapped_address = self.map_address(address) as usize;
        let values = value.to_le_bytes();
        self[mapped_address] = values[0];
        self[mapped_address + 1] = values[1];
    }
}

pub trait GenericMemory<const SIZE: usize> {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
    fn read_word(&self, address: u16) -> u16;
    fn write_word(&mut self, address: u16, value: u16);
}
