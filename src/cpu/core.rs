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


pub struct Interrupts {
    pub ie: u8,                   // Interrupt Enable Register (0xFFFF)
    pub if_: u8,                  // Interrupt Flag Register (0xFF0F)
    pub ime: bool,                // Interrupt Master Enable flag
    pub enable_ime_next: bool,    // Delayed EI effect
}

// Define CPU registers
pub struct CPU {
    pub a: u8, pub f: u8, // Accumulator & Flags
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
    pub cycles: u64, // Cycles
    pub interrupts: Interrupts,
    pub is_halted: bool,
    pub is_stopped: bool,
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CPU {{ A: {}, F: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {} }} SP: {}, PC: {}, Cycles: {} FLAGS {{ Z: {}, N: {}, H: {}, C: {} }}", 
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc, self.cycles,
            self.get_flag(&Flag::Z) as u8, self.get_flag(&Flag::N) as u8, self.get_flag(&Flag::H) as u8, self.get_flag(&Flag::C) as u8)
    }
}

impl CPU {
    pub fn read_register(&self, register: &Register) -> u8 {
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
    pub fn write_register(&mut self, register: &Register, value: u8) {
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
    pub fn read_register_pair(&self, register_pair: &RegisterPair) -> u16 {
        let high_byte = self.read_register(&register_pair.first);
        let low_byte = self.read_register(&register_pair.second);
        ((high_byte as u16) << 8) | (low_byte as u16)
    }
    pub fn write_register_pair(&mut self, register_pair: &RegisterPair, value: u16) {
        let high = (value >> 8) as u8;
        let low = value as u8;
        self.write_register(&register_pair.first, high);
        self.write_register(&register_pair.second, low);
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
            is_halted: false,
            is_stopped: false,
            interrupts: Interrupts {
                ie: 0,
                if_: 0,
                ime: false,
                enable_ime_next: false,
            },
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

    pub fn pop_u16(&mut self, memory: &mut Memory) -> u16 {
        let low_byte = memory.read_byte(self.sp);
        self.sp += 1;
        let high_byte = memory.read_byte(self.sp);
        self.sp += 1;
    
        ((high_byte as u16) << 8) | (low_byte as u16)
    }

    pub fn push_u16(&mut self, memory: &mut Memory, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = value as u8;
    
        self.sp -= 1;
        memory.write_byte(self.sp, high_byte); // High byte goes first
        self.sp -= 1;
        memory.write_byte(self.sp, low_byte);  // Then low byte
    }

    // Serial I/O emulation
// Called when memory[0xFF02] == 0x81
    pub fn handle_serial_for_test_rom(&mut self, memory: &mut Memory) {
        let control = memory.read_byte(0xFF02);
        if control == 0x81 {
            let byte = memory.read_byte(0xFF01);
            print!("{}", byte as char); // Output to console
            memory.write_byte(0xFF02, 0x00); // Reset
        }
    }

    pub fn print_gameboy_doc_output(&mut self, memory: &mut Memory) {
        println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}", 
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, 
            self.sp, self.pc, 
            memory.read_byte(self.pc),
            memory.read_byte(self.pc + 1),
            memory.read_byte(self.pc + 2),
            memory.read_byte(self.pc + 3));
    }

    pub fn set_varibles_for_gb_doc(&mut self) {
        self.a = 0x01;
        self.f = 0xB0;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.h = 0x01;
        self.l = 0x4D;
        self.sp = 0xFFFE;
        self.pc = 0x0100;
    }

}