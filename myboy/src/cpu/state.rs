use crate::cpu::{ByteRegister, WordRegister};
use crate::device::mem_map::MemMap;
use std::fmt::Debug;

use super::{CPU, RegisterSet};

#[derive(Clone, Copy)]
pub struct CPUState {
    pub register_set: RegisterSet,
    pub current_instruction_bytes: [u8; 4],
}

impl Debug for CPUState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pc = self.register_set.pc();
        let regset = &self.register_set;

        // let format1: &str = "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})";
        // let format2: &str = "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}";

        write!(
            f,
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            regset.get_b(ByteRegister::A),
            regset.get_b(ByteRegister::F),
            regset.get_b(ByteRegister::B),
            regset.get_b(ByteRegister::C),
            regset.get_b(ByteRegister::D),
            regset.get_b(ByteRegister::E),
            regset.get_b(ByteRegister::H),
            regset.get_b(ByteRegister::L),
            regset.get_w(WordRegister::SP),
            pc,
            self.current_instruction_bytes[0],
            self.current_instruction_bytes[1],
            self.current_instruction_bytes[2],
            self.current_instruction_bytes[3],
        )
    }
}

impl CPUState {
    pub fn new(cpu: &CPU, mem_map: &MemMap) -> Self {
        let register_set = cpu.register_set.clone();

        let pc = (*cpu.register_set.pc()).clone();
        let current_bytes = mem_map.read_word(pc).to_le_bytes();
        let next_bytes = mem_map.read_word(pc + 2).to_le_bytes();

        let current_instruction_bytes = [
            current_bytes[0],
            current_bytes[1],
            next_bytes[0],
            next_bytes[1],
        ];

        CPUState {
            register_set,
            current_instruction_bytes,
        }
    }
}
