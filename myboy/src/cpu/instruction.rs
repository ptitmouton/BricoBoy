use std::{cmp::max, fmt::Display};

use super::{
    addressing_mode::{AddressingMode, ImplicitOpCodeSize},
    condition::Condition,
};
use crate::cpu::register_set::{ByteRegister, WordRegister};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Instruction {
    pub(crate) instruction_type: InstructionType,
    pub(crate) opcode: u8,
    pub(crate) address: u16,
    pub(crate) condition: Option<Condition>,
    pub(crate) target: Option<AddressingMode>,
    pub(crate) source: Option<AddressingMode>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum InstructionType {
    Nop,
    LoadByte,
    LoadWord,
    LoadHigh,
    IncByte,
    DecByte,
    IncWord,
    DecWord,
    AddByte,
    AddWord,
    AddWithCarry,
    Sub,
    SubWithCarry,
    And,
    Xor,
    Or,
    Cp,
    RollLeft,
    RollRight,
    RollLeftThroughCarry,
    RollRightThroughCarry,
    ShiftLeftArithmetically,
    ShiftRightArithmetically,
    Swap,
    ShiftLeftLogically,
    DecimalAdjustAccumulator,
    CompareAccumulator,
    SetCarryFlag,
    ComplementCarryFlag,
    Jump,
    JumpRelative,
    Push,
    Pop,
    Return,
    ReturnInterrupt,
    Call,
    Restart,
    Stop,
    Halt,
    DisableInterrupts,
    EnableInterrupts,
    TestBit,
    ResetBit,
    SetBit,
}

impl Instruction {
    pub(crate) fn new(opcode: u8, address: u16) -> Instruction {
        let (instruction_type, (target, source), condition) =
            InstructionType::create_instruction_type(opcode);

        Instruction {
            instruction_type,
            opcode,
            address,
            condition,
            source,
            target,
        }
    }

    pub fn size(&self) -> u8 {
        if self.opcode == 0xcb {
            return 2;
        }

        let target_size = match &self.target {
            Some(target) => target.size(),
            None => 1,
        };
        let source_size = match &self.source {
            Some(source) => source.size(),
            None => 1,
        };

        max(target_size, source_size)
    }
}

impl InstructionType {
    fn create_instruction_type(
        opcode: u8,
    ) -> (
        InstructionType,
        (Option<AddressingMode>, Option<AddressingMode>),
        Option<Condition>,
    ) {
        // /
        //
        // BLOCK: 0
        //
        //
        // /
        if opcode == 0x00 {
            return (InstructionType::Nop, (None, None), None);
        }
        if (opcode & 0b1100_1111) == 0b0000_0001 {
            // LD r16, imm16
            return (
                InstructionType::LoadWord,
                (
                    Some(AddressingMode::WordRegister(
                        AddressingMode::get_word_register(opcode >> 4),
                    )),
                    Some(AddressingMode::ImmediateWord),
                ),
                None,
            );
        }
        if (opcode & 0b1100_1111) == 0b0000_0010 {
            // 0x32
            // LD (r16mem), A
            return (
                InstructionType::LoadByte,
                (
                    Some(AddressingMode::RegisterPointer(
                        AddressingMode::get_mem_word_register(opcode >> 4),
                    )),
                    Some(AddressingMode::ByteRegister(ByteRegister::A)),
                ),
                None,
            );
        }
        if (opcode & 0b1100_1111) == 0b0000_1010 {
            // 0x0A
            // LD A, (r16mem)
            return (
                InstructionType::LoadByte,
                (
                    Some(AddressingMode::RegisterPointer(
                        AddressingMode::get_mem_word_register(opcode >> 4),
                    )),
                    Some(AddressingMode::ByteRegister(ByteRegister::A)),
                ),
                None,
            );
        }
        if opcode == 0b0000_1000 {
            // 0x08
            // LD (imm16), SP
            return (
                InstructionType::LoadWord,
                (
                    Some(AddressingMode::ImmediatePointer),
                    Some(AddressingMode::WordRegister(WordRegister::SP)),
                ),
                None,
            );
        }

        if (opcode & 0b1100_1111) == 0b0000_0011 {
            // INC r16
            return (
                InstructionType::IncByte,
                (
                    Some(AddressingMode::get_r16_addressing_mode(opcode >> 4)),
                    None,
                ),
                None,
            );
        }
        if (opcode & 0b1100_1111) == 0b0000_1011 {
            // DEC r16
            return (
                InstructionType::DecWord,
                (
                    AddressingMode::get_r16_addressing_mode(opcode >> 4).into(),
                    None,
                ),
                None,
            );
        }
        if opcode & 0b1100_1111 == 0b0000_1001 {
            // ADD HL, r16
            return (
                InstructionType::AddWord,
                (
                    AddressingMode::WordRegister(WordRegister::HL).into(),
                    AddressingMode::get_r16_addressing_mode(opcode >> 4).into(),
                ),
                None,
            );
        }
        if opcode & 0b1100_0111 == 0b0000_0100 {
            // INC r8
            return (
                InstructionType::IncByte,
                (
                    AddressingMode::get_r8_adressing_mode(opcode >> 3).into(),
                    None,
                ),
                None,
            );
        }
        if opcode & 0b1100_0111 == 0b0000_0101 {
            // DEC r8
            return (
                InstructionType::DecByte,
                (
                    AddressingMode::get_r8_adressing_mode(opcode >> 3).into(),
                    None,
                ),
                None,
            );
        }
        if opcode & 0b1100_0111 == 0b0000_0110 {
            // LD r8, imm8
            return (
                InstructionType::LoadByte,
                (
                    AddressingMode::get_r8_adressing_mode(opcode >> 3).into(),
                    AddressingMode::ImmediateByte.into(),
                ),
                None,
            );
        }
        if opcode == 0b0000_0111 {
            // 0x07
            return (
                InstructionType::RollLeft,
                (AddressingMode::ByteRegister(ByteRegister::A).into(), None),
                None,
            );
        }
        if opcode == 0b0000_1111 {
            // 0x0F
            return (
                InstructionType::RollRight,
                (AddressingMode::ByteRegister(ByteRegister::A).into(), None),
                None,
            );
        }
        if opcode == 0b0001_0111 {
            // 0x17
            return (
                InstructionType::RollLeftThroughCarry,
                (AddressingMode::ByteRegister(ByteRegister::A).into(), None),
                None,
            );
        }
        if opcode == 0b0001_1111 {
            // 0x1F
            return (
                InstructionType::RollRightThroughCarry,
                (AddressingMode::ByteRegister(ByteRegister::A).into(), None),
                None,
            );
        }
        if opcode == 0b0010_0111 {
            // 0x27
            return (
                InstructionType::DecimalAdjustAccumulator,
                (None, None),
                None,
            );
        }
        if opcode == 0b0010_1111 {
            // 0x2F
            return (InstructionType::CompareAccumulator, (None, None), None);
        }
        if opcode == 0b0011_0111 {
            // 0x37
            return (InstructionType::SetCarryFlag, (None, None), None);
        }
        if opcode == 0b0011_1111 {
            // 0x3F
            return (InstructionType::ComplementCarryFlag, (None, None), None);
        }
        if opcode == 0b0001_1000 {
            // 0x18
            // JR imm8
            return (
                InstructionType::JumpRelative,
                (AddressingMode::ImmediateByte.into(), None),
                None,
            );
        }
        if opcode & 0b1110_0111 == 0b0010_0000 {
            // JR cond, imm8
            return (
                InstructionType::JumpRelative,
                (AddressingMode::ImmediateByte.into(), None),
                Condition::get_condition(opcode >> 3).into(),
            );
        }
        if opcode == 0b0001_0000 {
            // STOP
            return (InstructionType::Stop, (None, None), None);
        }

        // /
        //
        // BLOCK: 1
        //
        //
        // /
        if opcode == 0b0111_0110 {
            // HALT
            return (InstructionType::Halt, (None, None), None);
        }

        if opcode & 0b1100_0000 == 0b0100_0000 {
            // LD r8, r8
            return (
                InstructionType::LoadByte,
                (
                    AddressingMode::get_r8_adressing_mode(opcode >> 3).into(),
                    AddressingMode::get_r8_adressing_mode(opcode).into(),
                ),
                None,
            );
        }

        // /
        //
        // BLOCK: 2
        //
        //
        // /
        if opcode & 0b1100_0000 == 0b1000_0000 {
            match opcode & 0b0011_1000 {
                0b00_0000 => {
                    // ADD A, r8
                    return (
                        InstructionType::AddByte,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                0b00_1000 => {
                    // ADC A, r8
                    return (
                        InstructionType::AddWithCarry,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                0b01_0000 => {
                    // SUB A, r8
                    return (
                        InstructionType::Sub,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                0b01_1000 => {
                    // SBC A, r8
                    return (
                        InstructionType::SubWithCarry,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                0b10_0000 => {
                    // AND A, r8
                    return (
                        InstructionType::And,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                0b10_1000 => {
                    // XOR A, r8
                    return (
                        InstructionType::Xor,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                0b11_0000 => {
                    // OR A, r8
                    return (
                        InstructionType::Or,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                0b11_1000 => {
                    // CP A, r8
                    return (
                        InstructionType::Cp,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::get_r8_adressing_mode(opcode).into(),
                        ),
                        None,
                    );
                }
                _ => panic!("Invalid opcode {:02X}", opcode),
            }
        }

        // /
        //
        // BLOCK: 3
        //
        //
        // /
        if (opcode & 0b1100_0111) == 0b1100_0110 {
            match (opcode & 0b0011_1000) >> 3 {
                0b000 => {
                    // ADD A, imm8
                    return (
                        InstructionType::AddByte,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b001 => {
                    // ADC A, imm8
                    return (
                        InstructionType::AddWithCarry,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b010 => {
                    // SUB A, imm8
                    return (
                        InstructionType::Sub,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b011 => {
                    // SBC A, imm8
                    return (
                        InstructionType::SubWithCarry,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b100 => {
                    // AND A, imm8
                    return (
                        InstructionType::And,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b101 => {
                    // XOR A, imm8
                    return (
                        InstructionType::Xor,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b110 => {
                    // OR A, imm8
                    return (
                        InstructionType::Or,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b111 => {
                    // CP A, imm8
                    return (
                        InstructionType::Cp,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                _ => panic!("Invalid opcode {:02X}", opcode),
            }
        }

        if (opcode & 0b1110_0111) == 0b1100_0000 {
            // RET cond
            return (
                InstructionType::Return,
                (None, None),
                Condition::get_condition(opcode >> 3).into(),
            );
        }

        if opcode == 0b1100_1001 {
            // RET
            return (InstructionType::Return, (None, None), None);
        }

        if opcode == 0b1101_1001 {
            // RETI
            return (InstructionType::ReturnInterrupt, (None, None), None);
        }

        if (opcode & 0b1110_0111) == 0b1100_0100 {
            // JP cond, imm16
            return (
                InstructionType::Jump,
                (AddressingMode::ImmediateWord.into(), None),
                Condition::get_condition(opcode >> 3),
            );
        }

        if opcode == 0b1100_0011 {
            // JP imm16
            return (
                InstructionType::Jump,
                (AddressingMode::ImmediateWord.into(), None),
                None,
            );
        }

        if opcode == 0b1110_1001 {
            // JP HL
            return (
                InstructionType::Jump,
                (AddressingMode::WordRegister(WordRegister::HL).into(), None),
                None,
            );
        }

        if (opcode & 0b1110_0111) == 0b1100_0100 {
            // CALL cond, imm16
            return (
                InstructionType::Call,
                (AddressingMode::ImmediateWord.into(), None),
                Condition::get_condition(opcode >> 3),
            );
        }

        if opcode == 0b1100_1101 {
            // CALL imm16
            return (
                InstructionType::Call,
                (AddressingMode::ImmediateWord.into(), None),
                None,
            );
        }

        if (opcode & 0b1100_0111) == 0b1100_0111 {
            // RST tgt3
            return (
                InstructionType::Restart,
                (
                    AddressingMode::Value((opcode & 0b0011_1000) >> 3).into(),
                    None,
                ),
                None,
            );
        }

        if (opcode & 0b1100_1111) == 0b1100_0001 {
            // POP r16
            return (
                InstructionType::Pop,
                (
                    AddressingMode::WordRegister(AddressingMode::get_stack_word_register(
                        opcode >> 4,
                    ))
                    .into(),
                    None,
                ),
                None,
            );
        }

        if (opcode & 0b1100_1111) == 0b1100_0101 {
            // PUSH r16
            return (
                InstructionType::Push,
                (
                    AddressingMode::WordRegister(AddressingMode::get_stack_word_register(
                        opcode >> 4,
                    ))
                    .into(),
                    None,
                ),
                None,
            );
        }

        // /
        //
        // CB PREFIX
        //
        //
        // /

        if opcode == 0xcb {
            // CB PREFIX
            let opcode_pointer: *const u8 = &opcode;
            let next_byte_pointer = unsafe { opcode_pointer.add(1) };
            let next_opcode = unsafe { *next_byte_pointer };
            if (next_opcode & 0b1100_0000) == 0b0000_0000 {
                // first two bits are 0
                match next_opcode & 0b0011_1000 {
                    0b00_0000 => {
                        // RLC r8
                        return (
                            InstructionType::RollLeft,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    0b00_0100 => {
                        // RRC r8
                        return (
                            InstructionType::RollRight,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    0b00_1000 => {
                        // RL r8
                        return (
                            InstructionType::RollLeftThroughCarry,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    0b00_1100 => {
                        // RR r8
                        return (
                            InstructionType::RollRightThroughCarry,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    0b01_0000 => {
                        // SLA r8
                        return (
                            InstructionType::ShiftLeftArithmetically,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    0b01_0100 => {
                        // SRA r8
                        return (
                            InstructionType::ShiftRightArithmetically,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    0b01_1000 => {
                        // SWAP r8
                        return (
                            InstructionType::Swap,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    0b01_1100 => {
                        // SRL r8
                        return (
                            InstructionType::ShiftLeftLogically,
                            (
                                AddressingMode::get_r8_adressing_mode(next_opcode >> 3).into(),
                                None,
                            ),
                            None,
                        );
                    }
                    _ => panic!("Invalid opcode {:02X}", opcode),
                }
            }
            match next_opcode & 0b1100_0000 {
                0b0100_0000 => {
                    // BIT b3, r8
                    return (
                        InstructionType::TestBit,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b1000_0000 => {
                    // RES b3, r8
                    return (
                        InstructionType::ResetBit,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                0b1100_0000 => {
                    // SET b3, r8
                    return (
                        InstructionType::SetBit,
                        (
                            AddressingMode::ByteRegister(ByteRegister::A).into(),
                            AddressingMode::ImmediateByte.into(),
                        ),
                        None,
                    );
                }
                _ => panic!("Invalid opcode {:02X}", opcode),
            }
        }

        // /
        //
        // BLOCK: 3 -- continuation
        //
        //
        // /
        match opcode {
            0b1110_0010 => {
                // LDH (c), A
                return (
                    InstructionType::LoadHigh,
                    (
                        AddressingMode::RegisterPointerHigh(ByteRegister::C).into(),
                        AddressingMode::ByteRegister(ByteRegister::A).into(),
                    ),
                    None,
                );
            }
            0b1110_0000 => {
                // LDH (imm8), A
                return (
                    InstructionType::LoadHigh,
                    (
                        AddressingMode::ByteRegister(ByteRegister::A).into(),
                        AddressingMode::ImmediatePointerHigh.into(),
                    ),
                    None,
                );
            }
            0b1110_1010 => {
                // LD (imm16), A
                return (
                    InstructionType::LoadByte,
                    (
                        AddressingMode::ImmediatePointer.into(),
                        AddressingMode::ByteRegister(ByteRegister::A).into(),
                    ),
                    None,
                );
            }
            0b1111_0010 => {
                // LDH A, (c)
                return (
                    InstructionType::LoadHigh,
                    (
                        AddressingMode::ByteRegister(ByteRegister::A).into(),
                        AddressingMode::RegisterPointerHigh(ByteRegister::C).into(),
                    ),
                    None,
                );
            }
            0b1111_0000 => {
                // LDH A, (imm8)
                return (
                    InstructionType::LoadHigh,
                    (
                        AddressingMode::ByteRegister(ByteRegister::A).into(),
                        AddressingMode::ImmediatePointerHigh.into(),
                    ),
                    None,
                );
            }
            0b1111_1010 => {
                // LD A, (imm16)
                return (
                    InstructionType::LoadByte,
                    (
                        AddressingMode::ByteRegister(ByteRegister::A).into(),
                        AddressingMode::ImmediatePointer.into(),
                    ),
                    None,
                );
            }
            0b1110_1000 => {
                // ADD sp, imm8
                return (
                    InstructionType::AddWord,
                    (
                        AddressingMode::WordRegister(WordRegister::SP).into(),
                        AddressingMode::ImmediateByte.into(),
                    ),
                    None,
                );
            }
            0b1111_1000 => {
                // LD HL, sp+imm8
                // TODO: Find a solution, this is not correct
                return (
                    InstructionType::AddWord,
                    (
                        AddressingMode::WordRegister(WordRegister::HL).into(),
                        AddressingMode::ImmediateByte.into(),
                    ),
                    None,
                );
            }
            0b1111_1001 => {
                // LD SP, HL
                return (
                    InstructionType::LoadWord,
                    (
                        AddressingMode::WordRegister(WordRegister::SP).into(),
                        AddressingMode::WordRegister(WordRegister::HL).into(),
                    ),
                    None,
                );
            }
            0b1111_0011 => {
                // DI
                return (InstructionType::DisableInterrupts, (None, None), None);
            }
            0b1111_1011 => {
                // EI
                return (InstructionType::EnableInterrupts, (None, None), None);
            }
            _ => panic!("Invalid opcode {:02X}", opcode),
        }
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionType::Nop => write!(f, "NOP"),
            InstructionType::LoadByte => write!(f, "LD"),
            InstructionType::LoadWord => write!(f, "LD"),
            InstructionType::LoadHigh => write!(f, "LDH"),
            InstructionType::IncByte => write!(f, "INC"),
            InstructionType::IncWord => write!(f, "INC"),
            InstructionType::DecByte => write!(f, "DEC"),
            InstructionType::DecWord => write!(f, "DEC"),
            InstructionType::AddByte => write!(f, "ADD"),
            InstructionType::AddWord => write!(f, "ADD"),
            InstructionType::AddWithCarry => write!(f, "ADC"),
            InstructionType::Sub => write!(f, "SUB"),
            InstructionType::SubWithCarry => write!(f, "SBC"),
            InstructionType::And => write!(f, "AND"),
            InstructionType::Xor => write!(f, "XOR"),
            InstructionType::Or => write!(f, "OR"),
            InstructionType::Cp => write!(f, "CP"),
            InstructionType::RollLeft => write!(f, "RLC"),
            InstructionType::RollRight => write!(f, "RRC"),
            InstructionType::RollLeftThroughCarry => write!(f, "RL"),
            InstructionType::RollRightThroughCarry => write!(f, "RR"),
            InstructionType::ShiftLeftArithmetically => write!(f, "SLA"),
            InstructionType::ShiftRightArithmetically => write!(f, "SRA"),
            InstructionType::Swap => write!(f, "SWAP"),
            InstructionType::ShiftLeftLogically => write!(f, "SRL"),
            InstructionType::DecimalAdjustAccumulator => write!(f, "DAA"),
            InstructionType::CompareAccumulator => write!(f, "CP"),
            InstructionType::SetCarryFlag => write!(f, "SCF"),
            InstructionType::ComplementCarryFlag => write!(f, "CCF"),
            InstructionType::Jump => write!(f, "JP"),
            InstructionType::JumpRelative => write!(f, "JR"),
            InstructionType::Push => write!(f, "PUSH"),
            InstructionType::Pop => write!(f, "POP"),
            InstructionType::Return => write!(f, "RET"),
            InstructionType::ReturnInterrupt => write!(f, "RETI"),
            InstructionType::Call => write!(f, "CALL"),
            InstructionType::Restart => write!(f, "RST"),
            InstructionType::Stop => write!(f, "STOP"),
            InstructionType::Halt => write!(f, "HALT"),
            InstructionType::DisableInterrupts => write!(f, "DI"),
            InstructionType::EnableInterrupts => write!(f, "EI"),
            InstructionType::TestBit => write!(f, "BIT"),
            InstructionType::ResetBit => write!(f, "RES"),
            InstructionType::SetBit => write!(f, "SET"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = match &self.source {
            Some(source) => format!("{}", source),
            None => "-".to_string(),
        };
        let target = match &self.target {
            Some(target) => format!("{}", target),
            None => " ".to_string(),
        };
        write!(
            f,
            "0x{:04X}:    {:^8} : {:^10} < {:^10} | (0x{:02x}) .. {}",
            self.address,
            self.instruction_type,
            target,
            source,
            self.opcode,
            self.size()
        )
    }
}
