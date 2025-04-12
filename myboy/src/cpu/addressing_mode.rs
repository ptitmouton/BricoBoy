use std::fmt::Display;

use super::register_set::{ByteRegister, WordRegister};

#[derive(Clone, Copy, Debug)]
pub(crate) enum AddressingMode {
    Target(u16),
    ByteRegister(ByteRegister),
    WordRegister(WordRegister),
    RegisterPointer(WordRegister),
    RegisterPointerHigh(ByteRegister),
    ImmediateByte,
    ImmediateWord,
    ImmediatePointer,
    ImmediatePointerHigh,
    BitPosition(u8),
}

pub(super) trait ImplicitOpCodeSize {
    fn size(&self) -> u8;
}

impl AddressingMode {
    pub(crate) fn get_r8_adressing_mode(opcode: u8) -> AddressingMode {
        match opcode & 0b0000_0111 {
            0b110 => AddressingMode::RegisterPointer(WordRegister::HL),
            _ => AddressingMode::ByteRegister(AddressingMode::get_byte_register(opcode)),
        }
    }

    pub(crate) fn get_bitmask_for_bitposition(addressing_mode: &AddressingMode) -> u8 {
        match addressing_mode {
            AddressingMode::BitPosition(0) => 0b0000_0001,
            AddressingMode::BitPosition(1) => 0b0000_0010,
            AddressingMode::BitPosition(2) => 0b0000_0100,
            AddressingMode::BitPosition(3) => 0b0000_1000,
            AddressingMode::BitPosition(4) => 0b0001_0000,
            AddressingMode::BitPosition(5) => 0b0010_0000,
            AddressingMode::BitPosition(6) => 0b0100_0000,
            AddressingMode::BitPosition(7) => 0b1000_0000,
            _ => panic!("Invalid AddressingMode -- bitposition required."),
        }
    }

    pub(crate) fn get_b3_adressing_mode(opcode: u8) -> AddressingMode {
        AddressingMode::BitPosition((opcode & 0b0011_1000) >> 3)
    }

    pub(crate) fn get_byte_register(opcode: u8) -> ByteRegister {
        match opcode & 0b0000_0111 {
            0b000 => ByteRegister::B,
            0b001 => ByteRegister::C,
            0b010 => ByteRegister::D,
            0b011 => ByteRegister::E,
            0b100 => ByteRegister::H,
            0b101 => ByteRegister::L,
            0b111 => ByteRegister::A,
            _ => panic!("Invalid word register"),
        }
    }
    pub(crate) fn get_word_register(opcode: u8) -> WordRegister {
        match opcode & 0b0000_0011 {
            0b00 => WordRegister::BC,
            0b01 => WordRegister::DE,
            0b10 => WordRegister::HL,
            0b11 => WordRegister::SP,
            _ => panic!("Invalid word register"),
        }
    }

    pub(crate) fn get_mem_word_register(opcode: u8) -> WordRegister {
        match opcode & 0b0000_0011 {
            0b00 => WordRegister::BC,
            0b01 => WordRegister::DE,
            0b10 => WordRegister::HLi,
            0b11 => WordRegister::HLd,
            _ => panic!("Invalid word register"),
        }
    }

    pub(crate) fn get_stack_word_register(opcode: u8) -> WordRegister {
        match opcode & 0b0000_0011 {
            0b00 => WordRegister::BC,
            0b01 => WordRegister::DE,
            0b10 => WordRegister::HL,
            0b11 => WordRegister::AF,
            _ => panic!("Invalid word register"),
        }
    }

    pub(crate) fn get_r16_addressing_mode(opcode: u8) -> AddressingMode {
        AddressingMode::WordRegister(AddressingMode::get_word_register(opcode))
    }
}

impl ImplicitOpCodeSize for AddressingMode {
    fn size(&self) -> u8 {
        match self {
            AddressingMode::ImmediateByte => 2,
            AddressingMode::ImmediateWord => 3,
            AddressingMode::ImmediatePointer => 3,
            AddressingMode::ImmediatePointerHigh => 2,
            _ => 1,
        }
    }
}

impl Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressingMode::Target(val) => write!(f, "{:#x}", val),
            AddressingMode::ByteRegister(register) => write!(f, "{}", register),
            AddressingMode::WordRegister(register) => write!(f, "{}", register),
            AddressingMode::RegisterPointer(register) => write!(f, "({})", register),
            AddressingMode::RegisterPointerHigh(register) => write!(f, "0xff00 & ({})", register),
            AddressingMode::ImmediateByte => write!(f, "imm8"),
            AddressingMode::ImmediateWord => write!(f, "imm16"),
            AddressingMode::ImmediatePointer => write!(f, "(imm16)"),
            AddressingMode::ImmediatePointerHigh => write!(f, "0xff00 & (imm8)"),
            AddressingMode::BitPosition(u8) => write!(f, "pos({})", u8),
        }
    }
}
