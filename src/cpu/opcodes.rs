use super::CPU;

use crate::memory::Memory;
use super::core::{Register, REGISTER_AF, REGISTER_BC, REGISTER_DE, REGISTER_HL};

impl CPU {
    pub fn execute(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00 => {}, // NOP - Do nothing
            0x01 => {
                let word = self.fetch_word(memory);
                self.write_register_pair(&REGISTER_BC, word);
                self.cycles += 12;
            }, // LD BC,d16
            0x02 => {
                let address = self.read_register_pair(&REGISTER_BC);
                memory.write_byte(address, self.a);
                self.cycles += 8;
            }, // LD (BC),A - 0x02
            0x03 => {
                self.increment_register_pair(&REGISTER_BC);
                self.cycles += 8;
            }, // INC BC - 0x03
            0x04 => {
                self.increment_register(&Register::B);
                self.cycles += 4;
            }, // INC B - 0x04
            0x05 => {
                self.decrement_register(&Register::B);
                self.cycles += 4;
            }, // DEC B - 0x05
            0x06 => {
                let byte = self.fetch_byte(memory);
                self.write_register(&Register::B, byte);
                self.cycles += 8;
            }, //LD B,u8 - 0x06
            0x07 => {
                self.rotate_left_circular(&Register::A);
                self.cycles += 4;
            }, // RLC A - 0x07
            0x08 => {
                let address = self.fetch_word(memory);
                memory.write_word(address, self.sp);
                self.cycles += 20;
            }, // LD (u16), SP (Opcode 0x08) – Load Stack Pointer into Memory
            0x09 => {
                self.add_register_pair(&REGISTER_HL, &REGISTER_BC);
                self.cycles += 8;
            }, // ADD HL, BC (Opcode 0x09) – Add BC to HL
            0x0A => {
                let address = self.read_register_pair(&REGISTER_BC);
                self.a = memory.read_byte(address);
                self.cycles += 8;
            }, // LD A,(BC) - 0x0A
            0x0B => {
                self.decrement_register_pair(&REGISTER_BC);
                self.cycles += 8;
            }, // DEC BC - 0x0B
            0x0C => {
                self.increment_register(&Register::C);
                self.cycles += 4;
            }, // INC C - 0x0C
            0x0D => {
                self.decrement_register(&Register::C);
                self.cycles += 4;
            }, // DEC C - 0x0D
            0x0E => {
                let byte = self.fetch_byte(memory);
                self.write_register(&Register::C, byte);
                self.cycles += 8;
            }, // LD C,u8 - 0x0E
            0x0F => {
                self.rotate_right_circular(&Register::A);
                self.cycles += 4;
            }, // RRC A - 0x0F
            0x10 => {
                panic!("STOP instruction not implemented");
            }, // STOP - 0x10
            0x11 => {
                let word = self.fetch_word(memory);
                self.write_register_pair(&REGISTER_DE, word);
                self.cycles += 12;
            }, // LD DE,d16 - 0x11
            0x12 => {
                let address = self.read_register_pair(&REGISTER_DE);
                memory.write_byte(address, self.a);
                self.cycles += 8;
            }, // LD (DE),A - 0x12
            0x13 => {
                self.increment_register_pair(&REGISTER_DE);
                self.cycles += 8;
            }, // INC DE - 0x13
            0x14 => {
                self.increment_register(&Register::D);
                self.cycles += 4;
            }, // INC D - 0x14
            0x15 => {
                self.decrement_register(&Register::D);
                self.cycles += 4;
            }, // DEC D - 0x15
            0x16 => {
                let byte = self.fetch_byte(memory);
                self.write_register(&Register::D, byte);
                self.cycles += 8;
            }, // LD D,u8 - 0x16
            0x17 => {
                self.rotate_left_through_carry(&Register::A);
                self.cycles += 4;
            }, // RL A - 0x17
            0x18 => {
                let offset = self.fetch_byte(memory) as i8;
                self.jump_relative(offset);
                self.cycles += 12;
            }, // JR - 0x18
            0x19 => {
                self.add_register_pair(&REGISTER_HL, &REGISTER_DE);
                self.cycles += 8;
            }, // ADD HL, DE - 0x19 
            0x1A => {
                let address = self.read_register_pair(&REGISTER_DE);
                self.a = memory.read_byte(address);
                self.cycles += 8;
            }, // LD A,(DE) - 0x1A
            0x1B => {
                self.decrement_register_pair(&REGISTER_DE);
                self.cycles += 8;
            }, // DEC DE - 0x1B
            0x1C => {
                self.increment_register(&Register::E);
                self.cycles += 4;
            }, // INC E - 0x1C
            0x1D => {
                self.decrement_register(&Register::E);
                self.cycles += 4;
            }, // DEC E - 0x1D
            0x1E => {
                let byte = self.fetch_byte(memory);
                self.write_register(&Register::E, byte);
                self.cycles += 8;
            }, // LD E,u8 - 0x1E
            0x1F => {
                self.rotate_right_through_carry(&Register::A);
                self.cycles += 4;
            }, // RR A - 0x1F
            0x20 => {
                let offset = self.fetch_byte(memory) as i8;
                if self.jump_relative_if_not_zero(offset) {
                    self.cycles += 12;
                } else {
                    self.cycles += 8;
                }
            }, // JR NZ,u8 - 0x20
            0x21 => {
                let word = self.fetch_word(memory);
                self.write_register_pair(&REGISTER_HL, word);
                self.cycles += 12;
            }, // LD HL,u16 - 0x21
            0x22 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.a);
                self.increment_register_pair(&REGISTER_HL);
                self.cycles += 8;
            }, // LD (HL+),A - 0x22
            0x23 => {
                self.increment_register_pair(&REGISTER_HL);
                self.cycles += 8;
            }, // INC HL - 0x23
            0x24 => {
                self.increment_register(&Register::H);
                self.cycles += 4;
            }, // INC H - 0x24
            0x25 => {
                self.decrement_register(&Register::H);
                self.cycles += 4;
            }, // DEC H - 0x25
            0x26 => {
                let byte = self.fetch_byte(memory);
                self.write_register(&Register::H, byte);
                self.cycles += 8;
            }, // LD H,u8 - 0x26
            0x27 => {
                panic!("DA A - 0x27 not implemented");
            }, // DA A - 0x27
            0x28 => {
                let offset = self.fetch_byte(memory) as i8;
                if self.jump_relative_if_zero(offset) {
                    self.cycles += 12;
                } else {
                    self.cycles += 8;
                }
            }, // JR Z,u8 - 0x28
            0x29 => {
                self.add_register_pair(&REGISTER_HL, &REGISTER_HL);
                self.cycles += 8;
            }, // ADD HL,HL - 0x29
            0x2A => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.a = memory.read_byte(address);
                self.increment_register_pair(&REGISTER_HL);
                self.cycles += 8;
            }, // LD A,(HL+) - 0x2A
            0x2B => {
                self.decrement_register_pair(&REGISTER_HL);
                self.cycles += 8;
            }, // DEC HL - 0x2B
            0x2C => {
                self.increment_register(&Register::L);
                self.cycles += 4;
            }, // INC L - 0x2C
            0x2D => {
                self.decrement_register(&Register::L);
                self.cycles += 4;
            }, // DEC L - 0x2D
            0x2E => {
                let byte = self.fetch_byte(memory);
                self.write_register(&Register::L, byte);
                self.cycles += 8;
            }, // LD L,u8 - 0x2E
            0x2F => {
                self.complement_carry(&Register::A);
                self.cycles += 4;
            }
            
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }
}

