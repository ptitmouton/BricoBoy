use std::fmt::Display;

pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

impl CartridgeType {
    pub fn from_u8(code: &u8) -> Option<CartridgeType> {
        match code {
            0x00 => Some(CartridgeType::RomOnly),
            0x01 => Some(CartridgeType::Mbc1),
            0x02 => Some(CartridgeType::Mbc1Ram),
            0x03 => Some(CartridgeType::Mbc1RamBattery),
            0x05 => Some(CartridgeType::Mbc2),
            0x06 => Some(CartridgeType::Mbc2Battery),
            0x08 => Some(CartridgeType::RomRam),
            0x09 => Some(CartridgeType::RomRamBattery),
            0x0B => Some(CartridgeType::Mmm01),
            0x0C => Some(CartridgeType::Mmm01Ram),
            0x0D => Some(CartridgeType::Mmm01RamBattery),
            0x0F => Some(CartridgeType::Mbc3TimerBattery),
            0x10 => Some(CartridgeType::Mbc3TimerRamBattery),
            0x11 => Some(CartridgeType::Mbc3),
            0x12 => Some(CartridgeType::Mbc3Ram),
            0x13 => Some(CartridgeType::Mbc3RamBattery),
            0x19 => Some(CartridgeType::Mbc5),
            0x1A => Some(CartridgeType::Mbc5Ram),
            0x1B => Some(CartridgeType::Mbc5RamBattery),
            0x1C => Some(CartridgeType::Mbc5Rumble),
            0x1D => Some(CartridgeType::Mbc5RumbleRam),
            0x1E => Some(CartridgeType::Mbc5RumbleRamBattery),
            0x20 => Some(CartridgeType::Mbc6),
            0x22 => Some(CartridgeType::Mbc7SensorRumbleRamBattery),
            0xFC => Some(CartridgeType::PocketCamera),
            0xFD => Some(CartridgeType::BandaiTama5),
            0xFE => Some(CartridgeType::HuC3),
            0xFF => Some(CartridgeType::HuC1RamBattery),
            _ => None,
        }
    }
}

impl Display for CartridgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CartridgeType::RomOnly => write!(f, "ROM ONLY"),
            CartridgeType::Mbc1 => write!(f, "MBC1"),
            CartridgeType::Mbc1Ram => write!(f, "MBC1+RAM"),
            CartridgeType::Mbc1RamBattery => write!(f, "MBC1+RAM+BATTERY"),
            CartridgeType::Mbc2 => write!(f, "MBC2"),
            CartridgeType::Mbc2Battery => write!(f, "MBC2+BATTERY"),
            CartridgeType::RomRam => write!(f, "ROM+RAM"),
            CartridgeType::RomRamBattery => write!(f, "ROM+RAM+BATTERY"),
            CartridgeType::Mmm01 => write!(f, "MMM01"),
            CartridgeType::Mmm01Ram => write!(f, "MMM01+RAM"),
            CartridgeType::Mmm01RamBattery => write!(f, "MMM01+RAM+BATTERY"),
            CartridgeType::Mbc3TimerBattery => write!(f, "MBC3+TIMER+BATTERY"),
            CartridgeType::Mbc3TimerRamBattery => write!(f, "MBC3+TIMER+RAM+BATTERY"),
            CartridgeType::Mbc3 => write!(f, "MBC3"),
            CartridgeType::Mbc3Ram => write!(f, "MBC3+RAM"),
            CartridgeType::Mbc3RamBattery => write!(f, "MBC3+RAM+BATTERY"),
            CartridgeType::Mbc5 => write!(f, "MBC5"),
            CartridgeType::Mbc5Ram => write!(f, "MBC5+RAM"),
            CartridgeType::Mbc5RamBattery => write!(f, "MBC5+RAM+BATTERY"),
            CartridgeType::Mbc5Rumble => write!(f, "MBC5+RUMBLE"),
            CartridgeType::Mbc5RumbleRam => write!(f, "MBC5+RUMBLE+RAM"),
            CartridgeType::Mbc5RumbleRamBattery => write!(f, "MBC5+RUMBLE+RAM+BATTERY"),
            CartridgeType::Mbc6 => write!(f, "MBC6"),
            CartridgeType::Mbc7SensorRumbleRamBattery => {
                write!(f, "MBC7+SENSOR+RUMBLE+RAM+BATTERY")
            }
            CartridgeType::PocketCamera => write!(f, "POCKET CAMERA"),
            CartridgeType::BandaiTama5 => write!(f, "BANDAI TAMA5"),
            CartridgeType::HuC3 => write!(f, "HuC3"),
            CartridgeType::HuC1RamBattery => write!(f, "HuC1+RAM+BATTERY"),
        }
    }
}
