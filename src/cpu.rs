use crate::memory::Memory;

const FLAG_Z: u8 = 0b1000_0000; // Zero flag
const FLAG_N: u8 = 0b0100_0000; // Subtraction flag
const FLAG_H: u8 = 0b0010_0000; // Half Carry flag
const FLAG_C: u8 = 0b0001_0000; // Carry flag

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

pub enum Flag {
    Z,
    N,
    H,
    C
}

pub struct RegisterPair {
    pub first: Register,
    pub second: Register
}

pub const REGISTER_AF: RegisterPair = RegisterPair { first: Register::A, second: Register::F };
pub const REGISTER_HL: RegisterPair = RegisterPair { first: Register::H, second: Register::L };
pub const REGISTER_BC: RegisterPair = RegisterPair { first: Register::B, second: Register::C };
pub const REGISTER_DE: RegisterPair = RegisterPair { first: Register::D, second: Register::E }; 

// Define CPU registers
pub struct CPU {
    pub a: u8, pub f: u8, // Accumulator & Flags
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
    pub cycles: u32, // Cycles
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CPU {{ A: {}, F: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {}, SP: {}, PC: {}, Cycles: {} }}", 
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc, self.cycles)
    }
}

impl CPU {
    fn read_register(&self, register: &Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::H => self.h,
            Register::L => self.l,
        }
    }
    fn write_register(&mut self, register: &Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::F => self.f = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }
    fn read_register_pair(&self, register_pair: &RegisterPair) -> u16 {
        let first = self.read_register(&register_pair.first);
        let second = self.read_register(&register_pair.second);
        ((second as u16) << 8) | (first as u16)
    }
    fn write_register_pair(&mut self, register_pair: &RegisterPair, value: u16) {
        let first = (value >> 8) as u8;
        let second = value as u8;
        self.write_register(&register_pair.first, first);
        self.write_register(&register_pair.second, second);
    }
}

impl CPU {
    fn increment_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let result = value.wrapping_add(1);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, (value & 0x0F) + 1 > 0x0F);
    }
    fn decrement_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let result = value.wrapping_sub(1);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, true);
        self.set_flag(&Flag::H, (value & 0x0F) == 0x0F);
    }
    fn increment_register_pair(&mut self, register_pair: &RegisterPair) {
        let value = self.read_register_pair(register_pair);
        let result = value.wrapping_add(1);
        self.write_register_pair(register_pair, result);
    }
    fn decrement_register_pair(&mut self, register_pair: &RegisterPair) {
        let value = self.read_register_pair(register_pair);
        let result = value.wrapping_sub(1);
        self.write_register_pair(register_pair, result);
    }
    fn rlc_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let carry = value & 0x80 != 0;
        let result = value.rotate_left(1);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, false);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, carry);
    }
    fn rrc_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let carry = value & 0x01 != 0;
        let result = value.rotate_right(1);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, false);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, carry);
    }
    fn rl_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let old_carry = self.get_flag(&Flag::C);
        let new_carry = value & 0x80 != 0;
        let result = (value << 1) | (old_carry as u8);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, false);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, new_carry);
    }
    fn rr_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let old_carry = self.get_flag(&Flag::C);
        let new_carry = value & 0x01 != 0;
        let result = (value >> 1) | (old_carry as u8);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, false);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, new_carry);
    }
    fn add_register_pair(&mut self, lhs: &RegisterPair, rhs: &RegisterPair) {
        let lhs_value = self.read_register_pair(lhs);
        let rhs_value = self.read_register_pair(rhs);
        let (result, carry) = lhs_value.overflowing_add(rhs_value);
        let half_carry = ((lhs_value & 0x0FFF) + (rhs_value & 0x0FFF) > 0x0FFF);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, half_carry);
        self.set_flag(&Flag::C, carry);
        self.write_register_pair(lhs, result);
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0xFFFE,
            pc: 0x0100,
            cycles: 0,
        }
    }

    pub fn get_flag(&self, flag: &Flag) -> bool {
        match flag {
            Flag::Z => self.f & FLAG_Z != 0,
            Flag::N => self.f & FLAG_N != 0,
            Flag::H => self.f & FLAG_H != 0,
            Flag::C => self.f & FLAG_C != 0,
        }
    }

    pub fn set_flag(&mut self, flag: &Flag, value: bool) {
        let mask = match flag {
            Flag::Z => FLAG_Z,
            Flag::N => FLAG_N,
            Flag::H => FLAG_H,
            Flag::C => FLAG_C,
        };
        if value {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

    pub fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let opcode = memory.read_byte(self.pc);
        self.pc += 1;
        opcode
    }

    pub fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let low_byte = self.fetch_byte(memory);
        let high_byte = self.fetch_byte(memory);
        ((high_byte as u16) << 8) | (low_byte as u16)
    }

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
                self.rlc_register(&Register::A);
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
                self.rrc_register(&Register::A);
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
                self.rl_register(&Register::A);
                self.cycles += 4;
            }, // RL A - 0x17
            0x18 => {
                panic!("JR instruction not implemented");
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
                self.rr_register(&Register::A);
                self.cycles += 4;
            }, // RR A - 0x1F
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }
}

