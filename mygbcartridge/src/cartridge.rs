use crate::enums::{
    cartridge_type::CartridgeType,
    gbc_support::GBCSupport,
    new_licensee_code::{get_name_for_new_licensee_code, get_name_for_old_licensee_code},
};
use std::{num::Wrapping, path::Path, slice::SliceIndex};

#[derive(Clone)]
pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn new(path: &Path) -> Cartridge {
        Cartridge {
            data: std::fs::read(path).unwrap(),
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read_byte(address), self.read_byte(address + 1)])
    }

    pub fn read_fixed_bytes<const L: usize>(&self, address: u16, length: usize) -> &[u8; L] {
        self.data[(address as usize)..(address as usize) + length]
            .try_into()
            .unwrap()
    }

    pub fn read_range<S>(&self, range: S) -> &[u8]
    where
        S: SliceIndex<[u8], Output = [u8]>,
    {
        &self.data[range]
    }

    pub fn read_bytes<S>(&self, address: u16, length: usize) -> &[u8] {
        &self.data[(address as usize)..(address as usize) + length]
    }

    pub fn get_logo(&self) -> &[u8; 48] {
        self.read_fixed_bytes(0x0104, 48)
    }

    pub fn get_title(&self) -> String {
        String::from_utf8_lossy(self.read_range(0x0134..0x142))
            .trim_end_matches(char::from(0))
            .to_string()
    }

    pub fn manufacturer_code(&self) -> &[u8; 4] {
        self.read_fixed_bytes(0x013F, 4)
    }

    pub fn get_gbc_support(&self) -> GBCSupport {
        match self.read_byte(0x0143) {
            0x8 => GBCSupport::Enhanced,
            0xc => GBCSupport::Required,
            _ => GBCSupport::None,
        }
    }

    pub fn get_licensee(&self) -> Option<&str> {
        match self.read_byte(0x014b) {
            0x33 => get_name_for_new_licensee_code(self.read_fixed_bytes(0x0144, 2)),
            code => get_name_for_old_licensee_code(code),
        }
    }

    pub fn sgb_support_code(&self) -> u8 {
        return self.read_byte(0x0146);
    }

    pub fn get_cartridge_type(&self) -> Option<CartridgeType> {
        return CartridgeType::from_u8(&self.read_byte(0x0147));
    }

    pub fn get_rom_bank_count(&self) -> u16 {
        match self.read_byte(0x0148) {
            0 => 2,
            n => 2u16.pow(n as u32),
        }
    }

    pub fn get_rom_size(&self) -> u16 {
        return self.get_rom_bank_count() * 16;
    }

    pub fn has_valid_checksum(&self) -> bool {
        let mut sum = Wrapping(0u8);
        for i in 0x0134..0x014d {
            sum = sum - Wrapping(self.read_byte(i)) - Wrapping(1)
        }

        return sum == Wrapping(self.read_byte(0x014d));
    }
}
