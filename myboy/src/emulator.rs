use std::path::Path;

use mygbcartridge::cartridge::Cartridge;

use crate::{device::device::Device, ui::util::cartridge_data::CartridgeData};

pub struct EmulatorInstance {
    pub device: Device,
    pub cartridge_data: CartridgeData,

    cartridge: Cartridge,
}

impl EmulatorInstance {
    pub fn from_path(rom_path: &Path) -> EmulatorInstance {
        let cartridge = Cartridge::new(rom_path);
        let device = Device::new(&cartridge);
        let cartridge_data = CartridgeData::from_cartridge(&cartridge);

        EmulatorInstance {
            device,
            cartridge_data,

            cartridge,
        }
    }
}
