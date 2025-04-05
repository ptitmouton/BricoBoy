use mygbcartridge::cartridge::Cartridge;

use crate::cpu::instruction::Instruction;

#[derive(Clone)]
pub struct CartridgeData {
    pub cartridge: Cartridge,
    pub instructions: Vec<Result<Instruction, (u16, u8)>>,
}

impl CartridgeData {
    pub fn from_cartridge(cartridge: &Cartridge) -> Self {
        let instructions = CartridgeData::read_instructions(&cartridge);
        let cartridge = cartridge.clone();
        Self {
            cartridge,
            instructions,
        }
    }

    pub fn read_instructions(cartridge: &Cartridge) -> Vec<Result<Instruction, (u16, u8)>> {
        let mut instructions = Vec::new();
        let mut next_address = Some(0x0000);
        while let Some(address) = next_address {
            let opcode = cartridge.read_byte(address);
            if let Ok(instruction) = Instruction::create(address, &cartridge.data) {
                instructions.push(Result::Ok(instruction));
            } else {
                instructions.push(Result::Err((address, opcode)));
            }
            let wannabe_next = address + 1;
            if wannabe_next < cartridge.size() as u16 {
                next_address = Some(wannabe_next);
            } else {
                next_address = None;
            }
        }
        instructions
    }
}
