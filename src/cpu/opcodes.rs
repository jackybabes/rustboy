use super::CPU;

use crate::memory::Memory;
use super::core::{Register, REGISTER_BC, REGISTER_DE, REGISTER_HL, Flag, REGISTER_AF};

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
                let byte = self.fetch_byte(memory);
                if byte != 0x00 {
                    panic!("STOP instruction expects null afterwards");
                }
                // println!("STOP instruction");
                self.stop();
                self.cycles += 4;
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
                self.daa(&Register::A);
                self.cycles += 4;
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
                self.set_carry_flag();
                self.cycles += 4;
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
                // panic!("HALT - 0x76 not implemented");
                self.halt();
                self.cycles += 4;
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
            0x98 => {
                self.sub_register_from_register_with_carry(&Register::A, &Register::B);
                self.cycles += 4;
            }, // SBC A,B - 0x98
            0x99 => {
                self.sub_register_from_register_with_carry(&Register::A, &Register::C);
                self.cycles += 4;
            }, // SBC A,C - 0x99
            0x9A => {
                self.sub_register_from_register_with_carry(&Register::A, &Register::D);
                self.cycles += 4;
            }, // SBC A,D - 0x9A
            0x9B => {
                self.sub_register_from_register_with_carry(&Register::A, &Register::E);
                self.cycles += 4;
            }, // SBC A,E - 0x9B
            0x9C => {
                self.sub_register_from_register_with_carry(&Register::A, &Register::H);
                self.cycles += 4;
            }, // SBC A,H - 0x9C
            0x9D => {
                self.sub_register_from_register_with_carry(&Register::A, &Register::L);
                self.cycles += 4;
            }, // SBC A,L - 0x9D
            0x9E => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.sub_u8_from_register_with_carry(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // SBC A,(HL) - 0x9E
            0x9F => {
                self.sub_register_from_register_with_carry(&Register::A, &Register::A);
                self.cycles += 4;
            }, // SBC A,A - 0x9F
            0xA0 => {
                self.and_register_with_register(&Register::A, &Register::B);
                self.cycles += 4;
            }, // AND A,B - 0xA0
            0xA1 => {
                self.and_register_with_register(&Register::A, &Register::C);
                self.cycles += 4;
            }, // AND A,C - 0xA1
            0xA2 => {
                self.and_register_with_register(&Register::A, &Register::D);
                self.cycles += 4;
            }, // AND A,D - 0xA2
            0xA3 => {
                self.and_register_with_register(&Register::A, &Register::E);
                self.cycles += 4;
            }, // AND A,E - 0xA3
            0xA4 => {
                self.and_register_with_register(&Register::A, &Register::H);
                self.cycles += 4;
            }, // AND A,H - 0xA4
            0xA5 => {
                self.and_register_with_register(&Register::A, &Register::L);
                self.cycles += 4;
            }, // AND A,L - 0xA5
            0xA6 => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.and_u8_with_register(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // AND A,(HL) - 0xA6
            0xA7 => {
                self.and_register_with_register(&Register::A, &Register::A);
                self.cycles += 4;
            }, // AND A,A - 0xA7
            0xA8 => {
                self.xor_register_with_register(&Register::A, &Register::B);
                self.cycles += 4;
            }, // XOR A,B - 0xA8
            0xA9 => {
                self.xor_register_with_register(&Register::A, &Register::C);
                self.cycles += 4;
            }, // XOR A,C - 0xA9
            0xAA => {
                self.xor_register_with_register(&Register::A, &Register::D);
                self.cycles += 4;
            }, // XOR A,D - 0xAA
            0xAB => {
                self.xor_register_with_register(&Register::A, &Register::E);
                self.cycles += 4;
            }, // XOR A,E - 0xAB
            0xAC => {
                self.xor_register_with_register(&Register::A, &Register::H);
                self.cycles += 4;
            }, // XOR A,H - 0xAC
            0xAD => {
                self.xor_register_with_register(&Register::A, &Register::L);
                self.cycles += 4;
            }, // XOR A,L - 0xAD
            0xAE => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.xor_u8_with_register(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // XOR A,(HL) - 0xAE
            0xAF => {
                self.xor_register_with_register(&Register::A, &Register::A);
                self.cycles += 4;
            }, // XOR A,A - 0xAF
            0xB0 => {
                self.or_register_with_register(&Register::A, &Register::B);
                self.cycles += 4;
            }, // OR A,B - 0xB0
            0xB1 => {
                self.or_register_with_register(&Register::A, &Register::C);
                self.cycles += 4;
            }, // OR A,C - 0xB1
            0xB2 => {
                self.or_register_with_register(&Register::A, &Register::D);
                self.cycles += 4;
            }, // OR A,D - 0xB2
            0xB3 => {
                self.or_register_with_register(&Register::A, &Register::E);
                self.cycles += 4;
            }, // OR A,E - 0xB3
            0xB4 => {
                self.or_register_with_register(&Register::A, &Register::H);
                self.cycles += 4;
            }, // OR A,H - 0xB4
            0xB5 => {
                self.or_register_with_register(&Register::A, &Register::L);
                self.cycles += 4;
            }, // OR A,L - 0xB5
            0xB6 => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.or_u8_with_register(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // OR A,(HL) - 0xB6
            0xB7 => {
                self.or_register_with_register(&Register::A, &Register::A);
                self.cycles += 4;
            }, // OR A,A - 0xB7
            0xB8 => {
                self.compare_register_with_register(&Register::A, &Register::B);
                self.cycles += 4;
            }, // CP A,B - 0xB8
            0xB9 => {
                self.compare_register_with_register(&Register::A, &Register::C);
                self.cycles += 4;
            }, // CP A,C - 0xB9
            0xBA => {
                self.compare_register_with_register(&Register::A, &Register::D);
                self.cycles += 4;
            }, // CP A,D - 0xBA
            0xBB => {
                self.compare_register_with_register(&Register::A, &Register::E);
                self.cycles += 4;
            }, // CP A,E - 0xBB
            0xBC => {
                self.compare_register_with_register(&Register::A, &Register::H);
                self.cycles += 4;
            }, // CP A,H - 0xBC
            0xBD => {
                self.compare_register_with_register(&Register::A, &Register::L);
                self.cycles += 4;
            }, // CP A,L - 0xBD
            0xBE => {
                let address = self.read_register_pair(&REGISTER_HL);
                self.compare_u8_with_register(&Register::A, memory.read_byte(address));
                self.cycles += 8;
            }, // CP A,(HL) - 0xBE
            0xBF => {
                self.compare_register_with_register(&Register::A, &Register::A);
                self.cycles += 4;
            }, // CP A,A - 0xBF

            0xC0 => {
                if self.execute_if_flag_set_to_condition(false, &Flag::Z, |cpu, memory| cpu.ret(memory), memory) {
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                }
            }, // RET NZ - 0xC0

            0xC1 => {
                let value = self.pop_u16(memory);
                self.write_register_pair(&REGISTER_BC, value);
                self.cycles += 12;
            }, // POP BC - 0xC1

            0xC2 => {
                let address = self.fetch_word(&memory);
                if self.execute_if_flag_set_to_condition(false, &Flag::Z, |cpu, _memory| cpu.jump_to_address(address), memory) {
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                }
            }, // JP NZ,nn - 0xC2

            0xC3 => {
                let address = self.fetch_word(&memory);
                self.jump_to_address(address);
                self.cycles += 16;
            }, // JP nn - 0xC3

            0xC4 => {
                let address = self.fetch_word(&memory);
                if self.execute_if_flag_set_to_condition(false, &Flag::Z, |cpu, memory| cpu.call(memory, address), memory) {
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                }
            }, // CALL NZ,nn - 0xC4

            0xC5 => {
                self.push_u16(memory, self.read_register_pair(&REGISTER_BC));
                self.cycles += 16;
            }, // PUSH BC - 0xC5

            0xC6 => {
                let value = self.fetch_byte(&memory);
                self.add_u8_to_register(&Register::A, value);
                self.cycles += 8;
            }, // ADD A,n - 0xC6

            0xC7 => {
                self.pc = 0x00;
                self.cycles += 16;
            }, // RST 00H - 0xC7

            0xC8 => {
                if self.execute_if_flag_set_to_condition(true, &Flag::Z, |cpu, memory| cpu.ret(memory), memory) {
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                }
            }, // RET Z - 0xC8

            0xC9 => {
                self.ret(memory);
                self.cycles += 16;
            }, // RET - 0xC9

            0xCA => {
                let address= self.fetch_word(memory);
                if self.execute_if_flag_set_to_condition(true, &Flag::Z, |cpu, _memory| cpu.jump_to_address(address), memory) {
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                }
            }, // JP Z,nn - 0xCA

            0xCB => {
                // PREFIX CB
                let opcode = self.fetch_byte(memory);
                self.execute_cb_opcode(opcode, memory);
            }, // PREFIX CB - 0xCB

            0xCC => {
                let address = self.fetch_word(&memory);
                if self.execute_if_flag_set_to_condition(true, &Flag::Z, |cpu, memory| cpu.call(memory, address), memory) {
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                }
            }, // CALL Z,nn - 0xCC

            0xCD => {
                let address = self.fetch_word(&memory);
                self.call(memory, address);
                self.cycles += 24;
            }, // CALL nn - 0xCD

            0xCE => {
                let value = self.fetch_byte(&memory);
                self.add_u8_to_register_with_carry(&Register::A, value);
                self.cycles += 8;
            }, // ADC A,n - 0xCE


            0xCF => {
                self.call(memory, 0x08);
                self.cycles += 16;
            }, // RST 08H - 0xCF

            0xD0 => {
                if self.execute_if_flag_set_to_condition(false, &Flag::C, |cpu, memory| cpu.ret(memory), memory) {
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                }
            }, // RET NC - 0xD0
            0xD1 => {
                let value = self.pop_u16(memory);
                self.write_register_pair(&REGISTER_DE, value);
                self.cycles += 12;
            }, // POP DE - 0xD1

            0xD2 => {
                let address = self.fetch_word(&memory);
                if self.execute_if_flag_set_to_condition(false, &Flag::C, |cpu, _memory| cpu.jump_to_address(address), memory) {
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                }
            }, // JP NC,nn - 0xD2

            0xD3 => {
                panic!("0xD3 Unused");
            }, // Unused

            0xD4 => {
                let address = self.fetch_word(&memory);
                if self.execute_if_flag_set_to_condition(false, &Flag::C, |cpu, memory| cpu.call(memory, address), memory) {
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                }
            }, // CALL NC,nn - 0xD4

            0xD5 => {
                self.push_u16(memory, self.read_register_pair(&REGISTER_DE));
                self.cycles += 16;
            }, // PUSH DE - 0xD5

            0xD6 => {
                let value = self.fetch_byte(&memory);
                self.sub_u8_from_register(&Register::A, value);
                self.cycles += 8;
            }, // SUB A,n - 0xD6

            0xD7 => {
                self.call(memory, 0x10);
                self.cycles += 16;
            }, // RST 10H - 0xD7

            0xD8 => {
                if self.execute_if_flag_set_to_condition(true, &Flag::C, |cpu, memory| cpu.ret(memory), memory) {
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                }
            }, // RET C - 0xD8

            0xD9 => {
                self.reti(memory);
                self.cycles += 16;
            }, // RETI - 0xD9

            0xDA => {
                let address = self.fetch_word(&memory);
                if self.execute_if_flag_set_to_condition(true, &Flag::C, |cpu, _memory| cpu.jump_to_address(address), memory) {
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                }
            }, // JP C,nn - 0xDA

            0xDB => {
                panic!("0xDB Unused");
            }, // Unused

            0xDC => {
                let address = self.fetch_word(&memory);
                if self.execute_if_flag_set_to_condition(true, &Flag::C, |cpu, memory| cpu.call(memory, address), memory) {
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                }
            }, // CALL C,nn - 0xDC

            0xDD => {
                panic!("0xDD Unused");
            }, // Unused

            0xDE => {
                let value = self.fetch_byte(&memory);
                self.sub_u8_from_register_with_carry(&Register::A, value);
                self.cycles += 8;
            }, // SBC A,n - 0xDE

            0xDF => {
                self.call(memory, 0x18);
                self.cycles += 16;
            }, // RST 18H - 0xDF

            0xE0 => {
                let offset = self.fetch_byte(memory);
                let address = 0xFF00 as u16 + offset as u16;
                let value = self.read_register(&Register::A);
                memory.write_byte(address, value);
                self.cycles += 12;
            }, // LD (FF00+n),A - 0xE0

            0xE1 => {
                let value = self.pop_u16(memory);
                self.write_register_pair(&REGISTER_HL, value);
                self.cycles += 12;
            }, // POP HL - 0xE1

            0xE2 => {
                let address = 0xFF00 as u16 + self.read_register(&Register::C) as u16;
                let value =  self.read_register(&Register::A);
                memory.write_byte(address, value);
                self.cycles += 8;
            }, // LD (FF00+C),A - 0xE2

            0xE3 => {
                panic!("0xE3 Unused");
            }, // Unused

            0xE4 => {
                panic!("0xE4 Unused");
            }, // Unused

            0xE5 => {
                self.push_u16(memory, self.read_register_pair(&REGISTER_HL));
                self.cycles += 16;
            }, // PUSH HL - 0xE5

            0xE6 => {
                let value = self.fetch_byte(&memory);
                self.and_u8_with_register(&Register::A, value);
                self.cycles += 8;
            }, // AND n - 0xE6

            0xE7 => {
                self.call(memory, 0x20);
                self.cycles += 16;
            }, // RST 20H - 0xE7

            0xE8 => {
                let offset = self.fetch_byte(&memory);
                self.sp = self.add_i8_to_sp(offset, self.sp);
                self.cycles += 12;
            }, // ADD SP,i8 - 0xE8

            0xE9 => {
                self.jump_to_address(self.read_register_pair(&REGISTER_HL));
                self.cycles += 4;
            }, // JP (HL) - 0xE9

            0xEA => {
                let address = self.fetch_word(&memory);
                let value = self.read_register(&Register::A);
                memory.write_byte(address, value);
                self.cycles += 16;
            }, // LD (nn),A - 0xEA

            0xEB => {
                panic!("0xEB Unused");
            },

            0xEC => {
                panic!("0xEC Unused");
            },

            0xED => {
                panic!("0xED Unused");
            },

            0xEE => {
                let value = self.fetch_byte(memory);
                self.xor_u8_with_register(&Register::A, value);
                self.cycles += 8;
            }, // XOR n - 0xEE

            0xEF => {
                self.call(memory, 0x28);
                self.cycles += 16;
            }, // RST 28H - 0xEF

            0xF0 => {
                let offset = self.fetch_byte(memory);
                let address = 0xFF00 as u16 + offset as u16;
                let value = memory.read_byte(address);
                self.write_register(&Register::A, value);
                self.cycles += 12;
            }, // LD A,(FF00+n) - 0xF0

            0xF1 => {
                let value = self.pop_u16(memory);
                // bottom 4 bits of value are ignored
                let masked_value = value & 0xFFF0;
                self.write_register_pair(&REGISTER_AF, masked_value);
                self.cycles += 12;
            }, // POP AF - 0xF1

            0xF2 => {
                let address = 0xFF00 as u16 + self.read_register(&Register::C) as u16;
                let value = self.read_register(&Register::A);
                memory.write_byte(address, value);
                self.cycles += 8;
            }, // LD A,(FF00+C) - 0xF2

            0xF3 => {
                self.disable_interrupts();
                self.cycles += 4;
            }, 

            0xF4 => {
                panic!("0xF4 Unused");
            },

            0xF5 => {
                self.push_u16(memory, self.read_register_pair(&REGISTER_AF));
                self.cycles += 16;
            }, // PUSH AF - 0xF5

            0xF6 => {
                let value = self.fetch_byte(&memory);
                self.or_u8_with_register(&Register::A, value);
                self.cycles += 8;
            }, // OR n - 0xF6

            0xF7 => {
                self.call(memory, 0x30);
                self.cycles += 16;
            }, // RST 30H - 0xF7

            0xF8 => {
                let offset = self.fetch_byte(&memory);
                let value = self.add_i8_to_sp(offset, self.sp);
                self.write_register_pair(&REGISTER_HL, value);
                self.cycles += 12;
            }, // LD HL,SP+i8 - 0xF8

            0xF9 => {
                self.sp = self.read_register_pair(&REGISTER_HL);
                self.cycles += 8;
            }, // LD SP,HL - 0xF9

            0xFA => {
                let address = self.fetch_word(&memory);
                let value = memory.read_byte(address);
                self.write_register(&Register::A, value);
                self.cycles += 16;
            }, // LD A,(nn) - 0xFA

            0xFB => {
                self.enable_interrupts();
                self.cycles += 4;
            },

            0xFC => {
                panic!("0xFC Unused");
            },

            0xFD => {
                panic!("0xFD Unused");
            },

            0xFE => {
                let value = self.fetch_byte(&memory);
                self.compare_u8_with_register(&Register::A, value);
                self.cycles += 8;
            }, // CP n - 0xFE

            0xFF => {
                self.call(memory, 0x38);
                self.cycles += 16;
            }, // RST 38H - 0xFF
        }
    }
}