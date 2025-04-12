use crate::{
    cpu::cpu::InterruptMasterEnableStatus, device::mem_map::MemMap, io::if_register::IFRegister,
};

use super::{
    addressing_mode::AddressingMode,
    condition::Condition,
    cpu::CPU,
    instruction::{Instruction, InstructionType},
    register_set::{ByteRegister, Flag, WordRegister},
};

impl CPU {
    pub(super) fn get_source_byte(&self, mem_map: &MemMap, instruction: &Instruction) -> u8 {
        match instruction.source {
            Some(AddressingMode::ImmediateByte) => mem_map.read_byte(instruction.address + 1),
            Some(AddressingMode::ByteRegister(register)) => *self.register_set.get_b(register),
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                mem_map.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = mem_map.read_word(instruction.address + 1);
                mem_map.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let offset = mem_map.read_byte(instruction.address + 1);
                let address = 0xff00 + (offset as u16);
                let content = mem_map.read_byte(address);
                content
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = (0xff00 as u16) + (*self.register_set.get_b(register)) as u16;
                mem_map.read_byte(address)
            }
            _ => panic!("No source provided for instruction"),
        }
    }

    pub(super) fn get_source_word(&self, mem_map: &MemMap, instruction: &Instruction) -> u16 {
        match instruction.source {
            Some(AddressingMode::ImmediateByte) => {
                mem_map.read_byte(instruction.address + 1) as u16
            }
            Some(AddressingMode::ImmediateWord) => mem_map.read_word(instruction.address + 1),
            Some(AddressingMode::WordRegister(register)) => self.register_set.get_w(register),
            Some(AddressingMode::ByteRegister(register)) => {
                *self.register_set.get_b(register) as u16
            }
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                mem_map.read_word(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = mem_map.read_word(instruction.address + 1);
                mem_map.read_word(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = 0xff00 & mem_map.read_byte(instruction.address + 1) as u16;
                mem_map.read_word(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & *self.register_set.get_b(register) as u16;
                mem_map.read_word(address)
            }
            _ => panic!("No source provided for instruction"),
        }
    }

    pub(super) fn get_target_byte(&self, mem_map: &MemMap, instruction: &Instruction) -> u8 {
        match instruction.target {
            Some(AddressingMode::ImmediateByte) => mem_map.read_byte(instruction.address + 1),
            Some(AddressingMode::ByteRegister(register)) => *self.register_set.get_b(register),
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                mem_map.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = mem_map.read_word(instruction.address + 1);
                mem_map.read_byte(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = (0xff00 as u16) + (mem_map.read_byte(instruction.address + 1) as u16);
                mem_map.read_byte(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 + (*self.register_set.get_b(register) as u16);
                mem_map.read_byte(address)
            }
            _ => panic!("No target provided for instruction"),
        }
    }

    pub(super) fn get_target_word(&self, mem_map: &MemMap, instruction: &Instruction) -> u16 {
        match instruction.target {
            Some(AddressingMode::Target(value)) => value,
            Some(AddressingMode::ImmediateWord) => mem_map.read_word(instruction.address + 1),
            Some(AddressingMode::WordRegister(register)) => self.register_set.get_w(register),
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                mem_map.read_word(address)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = mem_map.read_word(instruction.address + 1);
                mem_map.read_word(address)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = 0xff00 + mem_map.read_word(instruction.address + 1);
                mem_map.read_word(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 + (*self.register_set.get_b(register) as u16);
                mem_map.read_word(address)
            }
            _ => {
                panic!("No target provided for instruction: {}", instruction);
            }
        }
    }

    pub(super) fn write_target_byte(
        &mut self,
        mem_map: &mut MemMap,
        instruction: &Instruction,
        value: u8,
    ) {
        match instruction.target {
            Some(AddressingMode::ImmediateByte) => {
                mem_map.write_byte(instruction.address + 1, value)
            }
            Some(AddressingMode::ByteRegister(register)) => {
                self.register_set.set_b(register, value);
            }
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                mem_map.write_byte(address, value)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = mem_map.read_word(instruction.address + 1);
                mem_map.write_byte(address, value)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = 0xff00 + (mem_map.read_byte(instruction.address + 1)) as u16;
                mem_map.write_byte(address, value)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 + (*self.register_set.get_b(register)) as u16;
                mem_map.write_byte(address, value)
            }
            _ => panic!("No source provided for instruction"),
        }
    }

    pub(super) fn write_target_word(
        &mut self,
        mem_map: &mut MemMap,
        instruction: &Instruction,
        value: u16,
    ) {
        match instruction.target {
            Some(AddressingMode::ImmediateWord) => {
                mem_map.write_word(instruction.address + 1, value)
            }
            Some(AddressingMode::WordRegister(register)) => {
                self.register_set.set_w(register, value)
            }
            Some(AddressingMode::RegisterPointer(register)) => {
                let address = self.register_set.get_w(register);
                mem_map.write_word(address, value)
            }
            Some(AddressingMode::ImmediatePointer) => {
                let address = mem_map.read_word(instruction.address + 1);
                mem_map.write_word(address, value)
            }
            Some(AddressingMode::ImmediatePointerHigh) => {
                let address = 0xff00 + (mem_map.read_byte(instruction.address + 1)) as u16;
                mem_map.write_word(address, value)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 + (*self.register_set.get_b(register) as u16);
                mem_map.write_word(address, value)
            }
            // We can also use 8-bit targets in 16-bit operations, ops like LDH do require
            // it, so we need to handle it here.
            // Normally the 16bit source is an address, so writing to it is absolutely possible.
            Some(AddressingMode::ByteRegister(register)) => {
                self.register_set.set_b(register, value as u8);
            }
            _ => panic!(
                "No (u16) target provided for instruction: {}",
                instruction.target.unwrap()
            ),
        }
    }

    pub(super) fn run(&mut self, mem_map: &mut MemMap, instruction: &Instruction) -> u32 {
        self.current_instruction = Some(instruction.clone());
        self.register_set
            .set_w(WordRegister::PC, instruction.address);

        match instruction.instruction_type {
            InstructionType::Nop => {
                self.register_set
                    .set_w(WordRegister::PC, instruction.address + 1);
                return 1;
            }
            InstructionType::LoadByte => {
                let source = self.get_source_byte(mem_map, instruction);
                self.write_target_byte(mem_map, instruction, source);
                match instruction.target {
                    Some(AddressingMode::RegisterPointer(WordRegister::HLi)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) + 1,
                        );
                    }
                    Some(AddressingMode::RegisterPointer(WordRegister::HLd)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) - 1,
                        );
                    }
                    _ => {}
                }
                match instruction.source {
                    Some(AddressingMode::RegisterPointer(WordRegister::HLi)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) + 1,
                        );
                    }
                    Some(AddressingMode::RegisterPointer(WordRegister::HLd)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) - 1,
                        );
                    }
                    _ => {}
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::LoadWord => {
                let source = self.get_source_word(mem_map, instruction);
                self.write_target_word(mem_map, instruction, source);
                match instruction.target {
                    Some(AddressingMode::RegisterPointer(WordRegister::HLi)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) + 1,
                        );
                    }
                    Some(AddressingMode::RegisterPointer(WordRegister::HLd)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) - 1,
                        );
                    }
                    _ => {}
                }
                match instruction.source {
                    Some(AddressingMode::RegisterPointer(WordRegister::HLi)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) + 1,
                        );
                    }
                    Some(AddressingMode::RegisterPointer(WordRegister::HLd)) => {
                        self.register_set.set_w(
                            WordRegister::HL,
                            self.register_set.get_w(WordRegister::HL) - 1,
                        );
                    }
                    _ => {}
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::LoadHigh => {
                let source = self.get_source_byte(mem_map, instruction);
                self.write_target_byte(mem_map, instruction, source);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                if let Some(AddressingMode::RegisterPointerHigh(_)) = instruction.target {
                    return 3;
                }
                return 2;
            }
            InstructionType::Or => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                let value = source | target;
                self.write_target_byte(mem_map, instruction, value);
                self.register_set.set_flag(Flag::Zero, value == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::Cp => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                let result = target.wrapping_sub(source);
                self.register_set.set_flag(Flag::Zero, source == target);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0xf) < (source & 0xf));
                self.register_set.set_flag(Flag::Carry, result > target);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::And => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                self.write_target_byte(mem_map, instruction, source & target);
                self.register_set
                    .set_flag(Flag::Zero, (source & target) == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, true);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::Xor => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                self.write_target_byte(mem_map, instruction, source ^ target);
                self.register_set.set_flag(Flag::Zero, source == target);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::AddByte => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                let result = source.wrapping_add(target);
                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set
                    .set_flag(Flag::HalfCarry, (source & 0xf) + (target & 0xf) > 0xf);
                self.register_set
                    .set_flag(Flag::Carry, (source as u16) + (target as u16) > 0xff);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::AddWord => {
                let source = self.get_source_word(mem_map, instruction);
                let target = self.get_target_word(mem_map, instruction);
                let result = source.wrapping_add(target);
                self.write_target_word(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Subtract, false);
                // What the heck ...?
                // When adding two 16-bit registers, the half-carry is from bit 11->12
                // When adding immediate value to a 16-bit register, the half-carry is from bit
                // 3->4 ....
                // https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8
                // ADD SP, e: H from bit 3, C from bit 7 (flags from low byte op)
                // ADD HL, rr: H from bit 11, C from bit 15 (flags from high byte op)
                if matches!(instruction.source, Some(AddressingMode::WordRegister(_))) {
                    self.register_set.set_flag(
                        Flag::HalfCarry,
                        ((source & 0x0fff) + (target & 0x0fff)) > 0x0fff,
                    );
                } else {
                    self.register_set
                        .set_flag(Flag::HalfCarry, (source & 0x0f) + (target & 0x0f) > 0x0f);
                }
                if matches!(instruction.source, Some(AddressingMode::ImmediateByte)) {
                    let source_u8 = source as u8;
                    let target_u8 = target as u8;
                    self.register_set
                        .set_flag(Flag::Carry, source_u8.overflowing_add(target_u8).1);
                } else {
                    self.register_set
                        .set_flag(Flag::Carry, source.overflowing_add(target).1);
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::AddSPAdjusted => {
                let e = self.get_source_byte(mem_map, instruction);
                let signed_e = e as i8;

                let sp = self.get_target_word(mem_map, instruction);
                let result = sp.wrapping_add_signed(signed_e.into());

                self.write_target_word(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, false);
                self.register_set.set_flag(Flag::Subtract, false);
                // ADD SP, e: H from bit 3, C from bit 7 (flags from low byte op)
                self.register_set
                    .set_flag(Flag::HalfCarry, (((sp as u8) & 0x0f) + (e & 0x0f)) > 0x0f);
                self.register_set
                    .set_flag(Flag::Carry, (sp as u8).overflowing_add(e).1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::LoadHLAdjusted => {
                let e = self.get_source_byte(mem_map, instruction);
                let signed_e = e as i8;

                let sp = self.register_set.get_w(WordRegister::SP);
                let result = sp.wrapping_add_signed(signed_e.into());

                self.write_target_word(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, false);
                self.register_set.set_flag(Flag::Subtract, false);
                // ADD SP, e: H from bit 3, C from bit 7 (flags from low byte op)
                self.register_set
                    .set_flag(Flag::HalfCarry, (((sp as u8) & 0x0f) + (e & 0x0f)) > 0x0f);
                self.register_set
                    .set_flag(Flag::Carry, (sp as u8).overflowing_add(e).1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::AddWithCarry => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                let carry = self.register_set.get_flag(Flag::Carry) as u8;
                let result_u16 = (source as u16) + (target as u16) + (carry as u16);
                let result = result_u16 as u8;
                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(
                    Flag::HalfCarry,
                    ((source & 0xf) + (target & 0xf) + carry) > 0xf,
                );
                self.register_set.set_flag(Flag::Carry, result_u16 > 0xff);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                if let Some(AddressingMode::ByteRegister(_)) = &instruction.target {
                    return 1;
                }
                return 2;
            }
            InstructionType::SubWithCarry => {
                let source = self.get_source_byte(mem_map, instruction);
                let carry = self.register_set.get_flag(Flag::Carry) as u8;

                let target = self.get_target_byte(mem_map, instruction);

                let result_u16 = (target as u16)
                    .wrapping_sub(source as u16)
                    .wrapping_sub(carry as u16);
                let result = result_u16 as u8;

                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::Carry, result_u16 > target as u16);
                self.register_set.set_flag(
                    Flag::HalfCarry,
                    ((target & 0xf)
                        .wrapping_sub(source & 0xf)
                        .wrapping_sub(carry))
                        > 0xf,
                );
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                if let Some(AddressingMode::ByteRegister(_)) = &instruction.target {
                    return 1;
                }
                return 2;
            }
            InstructionType::Sub => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                let result = target.wrapping_sub(source);
                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, source == target);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0xf) < (source & 0xf));
                self.register_set.set_flag(Flag::Carry, result > target);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::IncByte => {
                let target = self.get_target_byte(mem_map, instruction);
                let new_target = target.wrapping_add(1);
                self.write_target_byte(mem_map, instruction, new_target);
                self.register_set.set_flag(Flag::Zero, new_target == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0x0f) == 0x0f);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::DecByte => {
                let target = self.get_target_byte(mem_map, instruction);
                let new_target = target.wrapping_sub(1);
                self.write_target_byte(mem_map, instruction, new_target);
                self.register_set.set_flag(Flag::Zero, new_target == 0);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (new_target & 0xf) == 0xf);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::IncWord => {
                let target = self.get_target_word(mem_map, instruction);
                self.write_target_word(mem_map, instruction, target.wrapping_add(1));
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::DecWord => {
                let target = self.get_target_word(mem_map, instruction);
                let new_target = target.wrapping_sub(1);
                self.write_target_word(mem_map, instruction, new_target);
                let op_size = instruction.size();
                self.register_set
                    .set_w(WordRegister::PC, instruction.address + (op_size as u16));
                return 2;
            }
            InstructionType::Jump => {
                let condition_met = self.condition_met(instruction);
                if condition_met {
                    let target = self.get_target_word(mem_map, instruction);
                    self.register_set.set_w(WordRegister::PC, target);
                    return 4;
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + instruction.size() as u16,
                );
                return 3;
            }
            InstructionType::JumpRelative => {
                let condition_met = self.condition_met(instruction);
                if condition_met {
                    let target = self.get_target_byte(mem_map, instruction);
                    let current =
                        self.register_set.get_w(WordRegister::PC) + instruction.size() as u16;
                    let result = current.wrapping_add(target as i8 as u16);
                    self.register_set.set_w(WordRegister::PC, result);
                    return 4;
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + instruction.size() as u16,
                );
                return 3;
            }
            InstructionType::Push => {
                let source = self.get_target_word(mem_map, instruction);
                self.push_to_stack(mem_map, source);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 4;
            }
            InstructionType::Pop => {
                let value = self.pop_from_stack(mem_map);
                self.write_target_word(mem_map, instruction, value);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 3;
            }
            InstructionType::Reset => {
                let address = self.get_target_word(mem_map, instruction);
                self.push_to_stack(mem_map, instruction.address + 1);
                self.register_set.set_w(WordRegister::PC, address);
                return 4;
            }
            InstructionType::DisableInterrupts => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Disabled;
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::EnableInterrupts => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Enabling;
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::RotateLeft => {
                let target = self.get_target_byte(mem_map, instruction);
                let carry = self.register_set.get_flag(Flag::Carry) as u8;
                let result = (target << 1) | (carry >> 7);
                self.write_target_byte(mem_map, instruction, result);
                // only cb-prefixed instructions set the zero-flag,
                // otherwise it is reset (yeah wonder who decided this ... 🤷‍♂️)
                if instruction.size() == 1 {
                    self.register_set.set_flag(Flag::Zero, false);
                } else {
                    self.register_set.set_flag(Flag::Zero, result == 0);
                }
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::RotateLeftCircular => {
                let target = self.get_target_byte(mem_map, instruction);
                let result = (target << 1) | (target >> 7);
                self.write_target_byte(mem_map, instruction, result);
                // only cb-prefixed instructions set the zero-flag,
                // otherwise it is reset (yeah wonder who decided this ... 🤷‍♂️)
                if instruction.size() == 1 {
                    self.register_set.set_flag(Flag::Zero, false);
                } else {
                    self.register_set.set_flag(Flag::Zero, result == 0);
                }
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::RotateRight => {
                let target = self.get_target_byte(mem_map, instruction);
                let carry = self.register_set.get_flag(Flag::Carry) as u8;
                let result = (target >> 1) | (carry << 7);
                self.write_target_byte(mem_map, instruction, result);
                // only cb-prefixed instructions set the zero-flag,
                // otherwise it is reset (yeah wonder who decided this ... 🤷‍♂️)
                if instruction.size() == 1 {
                    self.register_set.set_flag(Flag::Zero, false);
                } else {
                    self.register_set.set_flag(Flag::Zero, result == 0);
                }
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::RotateRightCircular => {
                let target = self.get_target_byte(mem_map, instruction);
                let result = (target >> 1) | (target << 7);
                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, false);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::Swap => {
                let target = self.get_target_byte(mem_map, instruction);
                let result = ((target & 0xf0) >> 4) | ((target & 0x0f) << 4);
                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::ShiftRightLogically => {
                let target = self.get_target_byte(mem_map, instruction);
                let result_u16 = (target as u16) >> 1;
                let result = result_u16 as u8;
                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set
                    .set_flag(Flag::Carry, (target & 0x1) == 0x1); // initial bit 0 is now carry
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::ComplementAccumulator => {
                let a_value = *self.register_set.get_b(ByteRegister::A);
                let result = !a_value;
                self.register_set.set_b(ByteRegister::A, result);
                self.register_set.set_flag(Flag::Zero, result == 0);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set.set_flag(Flag::HalfCarry, true);
                self.register_set.set_flag(Flag::Carry, false);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::SetCarryFlag => {
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, true);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::Return => {
                let condition_met = self.condition_met(instruction);
                if condition_met {
                    // push current PC to stack
                    let mut stack_pointer = *self.register_set.sp();

                    let return_address = mem_map.read_word(stack_pointer);

                    stack_pointer += 2;
                    self.register_set.set_w(WordRegister::SP, stack_pointer);

                    self.register_set.set_w(WordRegister::PC, return_address);
                    return 4 + instruction.condition.is_some() as u32;
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::ReturnInterrupt => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Enabled;
                // push current PC to stack
                let mut stack_pointer = *self.register_set.sp();

                let return_address = mem_map.read_word(stack_pointer);

                stack_pointer += 2;
                self.register_set.set_w(WordRegister::SP, stack_pointer);

                self.register_set.set_w(WordRegister::PC, return_address);
                return 4;
            }
            InstructionType::Call => {
                let address = self.get_target_word(mem_map, instruction);
                let condition_met = self.condition_met(instruction);

                if condition_met {
                    // push current PC to stack
                    let mut stack_pointer = *self.register_set.sp();
                    stack_pointer -= 2;
                    self.register_set.set_w(WordRegister::SP, stack_pointer);
                    mem_map.write_word(
                        stack_pointer,
                        instruction.address + (instruction.size() as u16),
                    );
                    self.register_set.set_w(WordRegister::PC, address);
                    return 6;
                }
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 3;
            }
            InstructionType::Stop => {
                self.interrupt_master_enable = InterruptMasterEnableStatus::Disabled;
                mem_map.io_registers.set_if_register(IFRegister(0x00));
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            instruction_type => {
                todo!(
                    "The instruction type {} is not implemented",
                    instruction_type
                );
            }
        };
    }

    fn condition_met(&mut self, instruction: &Instruction) -> bool {
        match instruction.condition {
            Some(condition) => match condition {
                Condition::NotZero => !self.register_set.get_flag(Flag::Zero),
                Condition::Zero => self.register_set.get_flag(Flag::Zero),
                Condition::NoCarry => !self.register_set.get_flag(Flag::Carry),
                Condition::Carry => self.register_set.get_flag(Flag::Carry),
            },
            _ => true,
        }
    }
}
