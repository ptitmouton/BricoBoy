use crate::memory::generic_memory::OffsetMemory;

pub trait OAMEntryFlags {
    fn bg_priority(&self) -> bool;
    fn y_flip(&self) -> bool;
    fn x_flip(&self) -> bool;
    fn palette(&self) -> bool;
}

impl OAMEntryFlags for u8 {
    #[inline]
    fn bg_priority(&self) -> bool {
        self & 0b1000_0000 != 0
    }

    #[inline]
    fn y_flip(&self) -> bool {
        self & 0b0100_0000 != 0
    }

    #[inline]
    fn x_flip(&self) -> bool {
        self & 0b0010_0000 != 0
    }

    #[inline]
    fn palette(&self) -> bool {
        self & 0b0001_0000 != 0
    }
}

pub struct OAMEntry<'b> {
    pub x: &'b u8,
    pub y: &'b u8,
    pub tile_index: &'b u8,
    pub flags: &'b u8,
}

impl<'a> OAMEntry<'a> {
    pub fn from_bytes(bytes: &'a [u8; 4]) -> OAMEntry<'a> {
        OAMEntry {
            x: &bytes[0],
            y: &bytes[1],
            tile_index: &bytes[2],
            flags: &bytes[3],
        }
    }
}

pub struct OAM {
    pub data: [u8; 160],
}

impl OAM {
    pub fn new() -> OAM {
        let data = [0u8; 160];

        OAM { data }
    }

    #[inline]
    pub(crate) fn get_sprite(&self, index: usize) -> OAMEntry {
        let byte_slice = &self.data[index * 4..(index + 1) * 4];
        OAMEntry::from_bytes(byte_slice.try_into().unwrap())
    }

    #[inline]
    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        let translated_address = address - self.offset();
        self.data[translated_address as usize]
    }

    #[inline]
    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        println!("Writing to IO register: 0x{:2x}", address);
        let translated_address = address - self.offset();
        self.data[translated_address as usize] = value;
    }

    #[inline]
    pub(crate) fn read_word(&self, address: u16) -> u16 {
        let translated_address = (address - self.offset()) as usize;
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

impl<'a> OffsetMemory for OAM {
    fn offset(&self) -> u16 {
        0xfe00
    }
}
