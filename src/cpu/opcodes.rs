use super::CPU;

use crate::memory::Memory;
use super::core::{Register, REGISTER_BC, REGISTER_DE, REGISTER_HL};

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
                self.complement_register(&Register::A);
                self.cycles += 4;
            }, // CPL - 0x2F
            0x30 => {
                let offset = self.fetch_byte(memory) as i8;
                if self.jump_relative_if_not_carry(offset) {
                    self.cycles += 12;
                } else {
                    self.cycles += 8;
                }
            }, // JR NC,u8 - 0x30
            0x31 => {
                let word = self.fetch_word(memory);
                self.sp = word;
                self.cycles += 12;
            }, // LD SP,u16 - 0x31
            0x32 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.a);
                self.decrement_register_pair(&REGISTER_HL);
                self.cycles += 8;
            }, // LD (HL-),A - 0x32
            0x33 => {
                self.sp = self.sp.wrapping_add(1);
                self.cycles += 8;
            }, // INC SP - 0x33
            0x34 => {
                self.increment_byte_pointed_by_register_pair(memory, &REGISTER_HL);
                self.cycles += 12;
            }, // INC (HL) - 0x34
            0x35 => {
                self.decrement_byte_pointed_by_register_pair(memory, &REGISTER_HL);
                self.cycles += 12;
            }, // DEC (HL) - 0x35
            0x36 => {
                let byte = self.fetch_byte(memory);
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, byte);
                self.cycles += 12;
            }, // LD (HL),u8 - 0x36
            0x37 => {
                panic!("SCF - 0x37 not implemented");
            }, // SCF - 0x37
            0x38 => {
                let offset = self.fetch_byte(memory) as i8;
                if self.jump_relative_if_carry(offset) {
                    self.cycles += 12;
                } else {
                    self.cycles += 8;
                }
            }, // JR C,u8 - 0x38
            0x39 => {
                self.add_u16_to_register_pair(&REGISTER_HL, self.sp);
                self.cycles += 8;
            }, // ADD HL,SP - 0x39
            0x3A => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.a = memory.read_byte(address);
                self.decrement_register_pair(&REGISTER_HL);
                self.cycles += 8;
            }, // LD A,(HL-) - 0x3A
            0x3B => {
                self.sp = self.sp.wrapping_sub(1);
                self.cycles += 8;
            }, // DEC SP - 0x3B
            0x3C => {
                self.increment_register(&Register::A);
                self.cycles += 4;
            }, // INC A - 0x3C
            0x3D => {
                self.decrement_register(&Register::A);
                self.cycles += 4;
            }, // DEC A - 0x3D
            0x3E => {
                let byte = self.fetch_byte(memory);
                self.write_register(&Register::A, byte);
                self.cycles += 8;
            }, // LD A,u8 - 0x3E
            0x3F => {
                self.complement_carry_flag();
                self.cycles += 4;
            }, // CCF - 0x3F
            0x40 => {
                self.cycles += 4;
            }, // LD B,B - 0x40
            0x41 => {
                self.write_register(&Register::B, self.c);  
                self.cycles += 4;
            }, // LD B,C - 0x41
            0x42 => {
                self.write_register(&Register::B, self.d);
                self.cycles += 4;
            }, // LD B,D - 0x42
            0x43 => {
                self.write_register(&Register::B, self.e);
                self.cycles += 4;
            }, // LD B,E - 0x43
            0x44 => {
                self.write_register(&Register::B, self.h);
                self.cycles += 4;
            }, // LD B,H - 0x44
            0x45 => {
                self.write_register(&Register::B, self.l);
                self.cycles += 4;
            }, // LD B,L - 0x45
            0x46 => {
                let address = self.read_register_pair(&REGISTER_HL);
                let byte = memory.read_byte(address);
                self.write_register(&Register::B, byte);
                self.cycles += 8;
            }, // LD B,(HL) - 0x46
            0x47 => {
                self.write_register(&Register::B, self.a);
                self.cycles += 4;
            }, // LD B,A - 0x47
            0x48 => {
                self.write_register(&Register::C, self.b);
                self.cycles += 4;
            }, // LD C,B - 0x48
            0x49 => {
                self.cycles += 4;
            }, // LD C,C - 0x49
            0x4A => {
                self.write_register(&Register::C, self.d);
                self.cycles += 4;
            }, // LD C,D - 0x4A
            0x4B => {
                self.write_register(&Register::C, self.e);
                self.cycles += 4;
            }, // LD C,E - 0x4B
            0x4C => {
                self.write_register(&Register::C, self.h);
                self.cycles += 4;
            }, // LD C,H - 0x4C
            0x4D => {
                self.write_register(&Register::C, self.l);
                self.cycles += 4;
            }, // LD C,L - 0x4D
            0x4E => {
                let address = self.read_register_pair(&REGISTER_HL);
                let byte = memory.read_byte(address);
                self.write_register(&Register::C, byte);
                self.cycles += 8;
            }, // LD C,(HL) - 0x4E
            0x4F => {
                self.write_register(&Register::C, self.a);
                self.cycles += 4;
            }, // LD C,A - 0x4F
            0x50 => {
                self.write_register(&Register::D, self.b);
                self.cycles += 4;
            }, // LD D,B - 0x50
            0x51 => {
                self.write_register(&Register::D, self.c);
                self.cycles += 4;
            }, // LD D,C - 0x51
            0x52 => {
                self.cycles += 4;
            }, // LD D,D - 0x52
            0x53 => {
                self.write_register(&Register::D, self.e);
                self.cycles += 4;
            }, // LD D,E - 0x53
            0x54 => {
                self.write_register(&Register::D, self.h);
                self.cycles += 4;
            }, // LD D,H - 0x54
            0x55 => {
                self.write_register(&Register::D, self.l);
                self.cycles += 4;
            }, // LD D,L - 0x55
            0x56 => {
                let address = self.read_register_pair(&REGISTER_HL);
                let byte = memory.read_byte(address);
                self.write_register(&Register::D, byte);
                self.cycles += 8;
            }, // LD D,(HL) - 0x56
            0x57 => {
                self.write_register(&Register::D, self.a);
                self.cycles += 4;
            }, // LD D,A - 0x57
            0x58 => {
                self.write_register(&Register::E, self.b);
                self.cycles += 4;
            }, // LD E,B - 0x58
            0x59 => {
                self.write_register(&Register::E, self.c);
                self.cycles += 4;
            }, // LD E,C - 0x59
            0x5A => {
                self.write_register(&Register::E, self.d);
                self.cycles += 4;
            }, // LD E,D - 0x5A
            0x5B => {
                self.cycles += 4;
            }, // LD E,E - 0x5B
            0x5C => {
                self.write_register(&Register::E, self.h);
                self.cycles += 4;
            }, // LD E,H - 0x5C
            0x5D => {
                self.write_register(&Register::E, self.l);
                self.cycles += 4;
            }, // LD E,L - 0x5D
            0x5E => {
                let address = self.read_register_pair(&REGISTER_HL);
                let byte = memory.read_byte(address);
                self.write_register(&Register::E, byte);
                self.cycles += 8;
            }, // LD E,(HL) - 0x5E
            0x5F => {
                self.write_register(&Register::E, self.a);
                self.cycles += 4;
            }, // LD E,A - 0x5F
            0x60 => {
                self.write_register(&Register::H, self.b);
                self.cycles += 4;
            }, // LD H,B - 0x60
            0x61 => {
                self.write_register(&Register::H, self.c);
                self.cycles += 4;
            }, // LD H,C - 0x61
            0x62 => {
                self.write_register(&Register::H, self.d);
                self.cycles += 4;
            }, // LD H,D - 0x62
            0x63 => {
                self.write_register(&Register::H, self.e);
                self.cycles += 4;
            }, // LD H,E - 0x63
            0x64 => {
                self.cycles += 4;
            }, // LD H,H - 0x64
            0x65 => {
                self.write_register(&Register::H, self.l);
                self.cycles += 4;
            }, // LD H,L - 0x65
            0x66 => {
                let address = self.read_register_pair(&REGISTER_HL);
                let byte = memory.read_byte(address);
                self.write_register(&Register::H, byte);
                self.cycles += 8;
            }, // LD H,(HL) - 0x66
            0x67 => {
                self.write_register(&Register::H, self.a);
                self.cycles += 4;
            }, // LD H,A - 0x67
            0x68 => {
                self.write_register(&Register::L, self.b);
                self.cycles += 4;
            }, // LD L,B - 0x68
            0x69 => {
                self.write_register(&Register::L, self.c);
                self.cycles += 4;
            }, // LD L,C - 0x69
            0x6A => {
                self.write_register(&Register::L, self.d);
                self.cycles += 4;
            }, // LD L,D - 0x6A
            0x6B => {
                self.write_register(&Register::L, self.e);
                self.cycles += 4;
            }, // LD L,E - 0x6B
            0x6C => {
                self.write_register(&Register::L, self.h);
                self.cycles += 4;
            }, // LD L,H - 0x6C
            0x6D => {
                self.cycles += 4;
            }, // LD L,L - 0x6D
            0x6E => {
                let address = self.read_register_pair(&REGISTER_HL);
                let byte = memory.read_byte(address);
                self.write_register(&Register::L, byte);
                self.cycles += 8;
            }, // LD L,(HL) - 0x6E
            0x6F => {
                self.write_register(&Register::L, self.a);
                self.cycles += 4;
            }, // LD L,A - 0x6F
            0x70 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.b);
                self.cycles += 8;
            }, // LD (HL),B - 0x70
            0x71 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.c);
                self.cycles += 8;
            }, // LD (HL),C - 0x71
            0x72 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.d);
                self.cycles += 8;
            }, // LD (HL),D - 0x72
            0x73 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.e);
                self.cycles += 8;
            }, // LD (HL),E - 0x73
            0x74 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.h);
                self.cycles += 8;
            }, // LD (HL),H - 0x74
            0x75 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.l);
                self.cycles += 8;
            }, // LD (HL),L - 0x75
            0x76 => {
                panic!("HALT - 0x76 not implemented");
            }, // HALT - 0x76
            0x77 => {
                let address = self.read_register_pair(&REGISTER_HL);
                memory.write_byte(address, self.a);
                self.cycles += 8;
            }, // LD (HL),A - 0x77
            0x78 => {
                self.write_register(&Register::A, self.b);
                self.cycles += 4;
            }, // LD A,B - 0x78
            0x79 => {
                self.write_register(&Register::A, self.c);
                self.cycles += 4;
            }, // LD A,C - 0x79
            0x7A => {
                self.write_register(&Register::A, self.d);
                self.cycles += 4;
            }, // LD A,D - 0x7A
            0x7B => {
                self.write_register(&Register::A, self.e);
                self.cycles += 4;
            }, // LD A,E - 0x7B
            0x7C => {
                self.write_register(&Register::A, self.h);
                self.cycles += 4;
            }, // LD A,H - 0x7C
            0x7D => {
                self.write_register(&Register::A, self.l);
                self.cycles += 4;
            }, // LD A,L - 0x7D
            0x7E => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.a = memory.read_byte(address);
                self.cycles += 8;
            }, // LD A,(HL) - 0x7E
            0x7F => {
                self.cycles += 4;
            }, // LD A,A - 0x7F
            0x80 => {
                self.add_register_to_register(&Register::A, &Register::B);
                self.cycles += 4;
            }, // ADD A,B - 0x80
            0x81 => {
                self.add_register_to_register(&Register::A, &Register::C);
                self.cycles += 4;
            }, // ADD A,C - 0x81
            0x82 => {
                self.add_register_to_register(&Register::A, &Register::D);
                self.cycles += 4;
            }, // ADD A,D - 0x82
            0x83 => {
                self.add_register_to_register(&Register::A, &Register::E);
                self.cycles += 4;
            }, // ADD A,E - 0x83
            0x84 => {
                self.add_register_to_register(&Register::A, &Register::H);
                self.cycles += 4;
            }, // ADD A,H - 0x84
            0x85 => {
                self.add_register_to_register(&Register::A, &Register::L);
                self.cycles += 4;
            }, // ADD A,L - 0x85
            0x86 => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.add_u8_to_register(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // ADD A,(HL) - 0x86
            0x87 => {
                self.add_register_to_register(&Register::A, &Register::A);
                self.cycles += 4;
            }, // ADD A,A - 0x87
            0x88 => {
                self.add_register_to_register_with_carry(&Register::A, &Register::B);
                self.cycles += 4;
            }, // ADC A,B - 0x88
            0x89 => {
                self.add_register_to_register_with_carry(&Register::A, &Register::C);
                self.cycles += 4;
            }, // ADC A,C - 0x89
            0x8A => {
                self.add_register_to_register_with_carry(&Register::A, &Register::D);
                self.cycles += 4;
            }, // ADC A,D - 0x8A
            0x8B => {
                self.add_register_to_register_with_carry(&Register::A, &Register::E);
                self.cycles += 4;
            }, // ADC A,E - 0x8B
            0x8C => {
                self.add_register_to_register_with_carry(&Register::A, &Register::H);
                self.cycles += 4;
            }, // ADC A,H - 0x8C
            0x8D => {
                self.add_register_to_register_with_carry(&Register::A, &Register::L);
                self.cycles += 4;
            }, // ADC A,L - 0x8D
            0x8E => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.add_u8_to_register_with_carry(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // ADC A,(HL) - 0x8E
            0x8F => {
                self.add_register_to_register_with_carry(&Register::A, &Register::A);
                self.cycles += 4;
            }, // ADC A,A - 0x8F
            0x90 => {
                self.sub_register_from_register(&Register::A, &Register::B);
                self.cycles += 4;
            }, // SUB A,B - 0x90
            0x91 => {
                self.sub_register_from_register(&Register::A, &Register::C);
                self.cycles += 4;
            }, // SUB A,C - 0x91
            0x92 => {
                self.sub_register_from_register(&Register::A, &Register::D);
                self.cycles += 4;
            }, // SUB A,D - 0x92
            0x93 => {
                self.sub_register_from_register(&Register::A, &Register::E);
                self.cycles += 4;
            }, // SUB A,E - 0x93
            0x94 => {
                self.sub_register_from_register(&Register::A, &Register::H);
                self.cycles += 4;
            }, // SUB A,H - 0x94
            0x95 => {
                self.sub_register_from_register(&Register::A, &Register::L);
                self.cycles += 4;
            }, // SUB A,L - 0x95
            0x96 => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.sub_u8_from_register(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // SUB A,(HL) - 0x96
            0x97 => {
                self.sub_register_from_register(&Register::A, &Register::A);
                self.cycles += 4;
            }, // SUB A,A - 0x97

            
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }
}