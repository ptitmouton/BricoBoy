mod cpu;
mod device;
mod io;
pub mod logger;
mod memory;
mod ppu;

use std::path::Path;

use device::device::Device;
use ppu::ppu::PPU;

fn main() {
    let path = Path::new("/Users/arinaldoni/Downloads/tetris.gb");

    let mut device = Device::new();

    device.load_path(path);
}
