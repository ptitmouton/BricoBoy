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
                let address = 0xff00 + (mem_map.read_byte(instruction.address + 1) as u16);
                let content = mem_map.read_byte(address);
                content
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = (0xff00 as u16) & (*self.register_set.get_b(register)) as u16;
                mem_map.read_byte(address)
            }
            _ => panic!("No source provided for instruction"),
        }
    }

    pub(super) fn get_source_word(&self, mem_map: &MemMap, instruction: &Instruction) -> u16 {
        match instruction.source {
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
                let address = (0xff00 as u16) & (mem_map.read_byte(instruction.address + 1) as u16);
                mem_map.read_byte(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register) as u16);
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
                let address = 0xff00 & mem_map.read_word(instruction.address + 1);
                mem_map.read_word(address)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register) as u16);
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
                let address = (0xff00) & (mem_map.read_byte(instruction.address + 1)) as u16;
                mem_map.write_byte(address, value)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register)) as u16;
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
                let address = (0xff00) & (mem_map.read_byte(instruction.address + 1)) as u16;
                mem_map.write_word(address, value)
            }
            Some(AddressingMode::RegisterPointerHigh(register)) => {
                let address = 0xff00 & (*self.register_set.get_b(register) as u16);
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
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::LoadHigh => {
                let source = self.get_source_byte(mem_map, instruction) as u16;
                self.write_target_word(mem_map, instruction, source);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                if (source & 0xff00) != 0 {
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
                let target = self.get_target_byte(mem_map, instruction);
                let a_value = *self.register_set.get_b(ByteRegister::A);
                let sub_result = target.wrapping_sub(a_value);
                self.register_set.set_flag(Flag::Zero, sub_result == 0);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0xf) < (a_value & 0xf));
                self.register_set.set_flag(Flag::Carry, target < a_value);
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
                self.write_target_byte(mem_map, instruction, source + target);
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
                match instruction.target {
                    Some(AddressingMode::WordRegister(WordRegister::SP)) => {
                        self.register_set.set_flag(Flag::Zero, false);
                    }
                    _ => {}
                }
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set
                    .set_flag(Flag::HalfCarry, (source & 0xf) + (target & 0xf) > 0xf);
                self.register_set
                    .set_flag(Flag::Carry, (source as u32) + (target as u32) > 0xff);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 2;
            }
            InstructionType::Sub => {
                let source = self.get_source_byte(mem_map, instruction);
                let target = self.get_target_byte(mem_map, instruction);
                self.write_target_byte(mem_map, instruction, source - target);
                self.register_set.set_flag(Flag::Zero, source == target);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (source & 0xf) < (target & 0xf));
                self.register_set.set_flag(Flag::Carry, source < target);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::IncByte => {
                let target = self.get_target_byte(mem_map, instruction);
                self.write_target_byte(mem_map, instruction, target.wrapping_add(1));
                self.register_set.set_flag(Flag::Zero, target == 0);
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
                self.register_set.set_flag(Flag::Zero, target == 0);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0x0f) == 0x0f);
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
                self.register_set.set_flag(Flag::Zero, target == 0);
                self.register_set.set_flag(Flag::Subtract, true);
                self.register_set
                    .set_flag(Flag::HalfCarry, (target & 0x0f) == 0x0f);
                self.register_set
                    .set_w(WordRegister::PC, instruction.address + (op_size as u16));
                return 2;
            }
            InstructionType::Jump => {
                let target = self.get_target_word(mem_map, instruction);
                self.register_set.set_w(WordRegister::PC, target);
                return 4;
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
            InstructionType::Restart => {
                let address = self.get_target_word(mem_map, instruction);
                self.push_to_stack(mem_map, instruction.address + (instruction.size() as u16));
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
            InstructionType::RollLeft => {
                let target = self.get_target_byte(mem_map, instruction);
                let result = (target << 1) | (target >> 7);
                self.write_target_byte(mem_map, instruction, result);
                self.register_set.set_flag(Flag::Zero, false);
                self.register_set.set_flag(Flag::Subtract, false);
                self.register_set.set_flag(Flag::HalfCarry, false);
                self.register_set.set_flag(Flag::Carry, target & 0x1 == 1);
                self.register_set.set_w(
                    WordRegister::PC,
                    instruction.address + (instruction.size() as u16),
                );
                return 1;
            }
            InstructionType::RollRight => {
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
                return 1;
            }
            InstructionType::RollRightThroughCarry => {
                let target = self.get_target_byte(mem_map, instruction);
                let carry = self.register_set.get_flag(Flag::Carry) as u8;
                let result = (target >> 1) | (carry << 7);
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
            InstructionType::Return => {
                // push current PC to stack
                let mut stack_pointer = *self.register_set.sp();
                println!("Stack pointer before return: 0x{:04x}", stack_pointer);

                let return_address = mem_map.read_word(stack_pointer);
                println!("Return address: 0x{:04x}", return_address);

                stack_pointer += 2;
                self.register_set.set_w(WordRegister::SP, stack_pointer);
                println!("Stack pointer after return: 0x{:04x}", stack_pointer);

                self.register_set.set_w(WordRegister::PC, return_address);
                return 4;
            }
            InstructionType::Call => {
                let address = self.get_target_word(mem_map, instruction);

                // push current PC to stack
                let mut stack_pointer = *self.register_set.sp();
                stack_pointer -= 2;
                self.register_set.set_w(WordRegister::SP, stack_pointer);
                mem_map.write_word(
                    stack_pointer,
                    instruction.address + (instruction.size() as u16),
                );
                println!(
                    "Pushed PC to stack: 0x{:04x} => SP now 0x{:04x}",
                    instruction.address, stack_pointer
                );
                self.register_set.set_w(WordRegister::PC, address);
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
