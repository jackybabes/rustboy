use crate::memory::Memory;

const FLAG_Z: u8 = 0b1000_0000; // Zero flag
const FLAG_N: u8 = 0b0100_0000; // Subtraction flag
const FLAG_H: u8 = 0b0010_0000; // Half Carry flag
const FLAG_C: u8 = 0b0001_0000; // Carry flag

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

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }


    // Getters
    pub fn get_z_flag(&self) -> bool {
        self.f & FLAG_Z != 0
    }

    pub fn get_n_flag(&self) -> bool {
        self.f & FLAG_N != 0
    }

    pub fn get_h_flag(&self) -> bool {
        self.f & FLAG_H != 0
    }

    pub fn get_c_flag(&self) -> bool {
        self.f & FLAG_C != 0
    }

    // Setters
    pub fn set_z_flag(&mut self, value: bool) {
        if value {
            self.f |= FLAG_Z;
        } else {
            self.f &= !FLAG_Z;
        }
    }

    pub fn set_n_flag(&mut self, value: bool) {
        if value {
            self.f |= FLAG_N;
        } else {
            self.f &= !FLAG_N;
        }
    }

    pub fn set_h_flag(&mut self, value: bool) {
        if value {
            self.f |= FLAG_H;
        } else {
            self.f &= !FLAG_H;
        }
    }

    pub fn set_c_flag(&mut self, value: bool) {
        if value {
            self.f |= FLAG_C;
        } else {
            self.f &= !FLAG_C;
        }
    }
    

    pub fn fetch(&mut self, memory: &Memory) -> u8 {
        let opcode = memory.read(self.pc);
        self.pc += 1;
        opcode
    }

    pub fn execute(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00 => {}, // NOP - Do nothing
            0x01 => {
                // Get next two bytes from memory
                let low_byte = self.fetch(memory);
                let high_byte = self.fetch(memory);

                // Set BC to the value of the next two bytes
                self.c = low_byte;
                self.b = high_byte;
                self.cycles += 12;
            }, // LD BC,d16
            0x02 => {
                // This instruction stores the value in register A into the memory location pointed to by the BC register pair.
                let address = self.get_bc();
                memory.write(address, self.a);
                self.cycles += 8;
            }, // LD (BC),A - 0x02
            0x03 => {
                self.c = self.c.wrapping_add(1);
                if self.c == 0 {
                    self.b = self.b.wrapping_add(1);
                }
                self.cycles += 8;
            }, // INC BC - 0x03
            0x04 => {
                self.b = self.b.wrapping_add(1);
                self.cycles += 4;
                self.set_z_flag(self.b == 0);
                self.set_n_flag(false);
                // & (bitwise AND) extracts only the lower 4 bits of B.
                self.set_h_flag((self.b & 0x0F) == 0);
                // carry uneffected
            }, // INC B - 0x04
            0x05 => {
                self.b = self.b.wrapping_sub(1);
                self.cycles += 4;
                self.set_z_flag(self.b == 0);
                self.set_n_flag(true);

                //Set Half Carry flag if there was a borrow from bit 4
                self.set_h_flag((self.b & 0x0F) == 0x0F);
                
            }, // DEC B - 0x05
            0x06 => {
                let byte = self.fetch(memory);
                self.b = byte;
                self.cycles += 8;
            }, //LD B,u8 - 0x06
            0x07 => {
                let carry = self.a & 0x80 != 0;
                self.set_c_flag(carry);
                self.a = self.a << 1;
                if self.get_c_flag() {
                    self.a |= 0x01;
                }
                self.set_z_flag(false);
                self.set_n_flag(false);
                self.set_h_flag(false);
                self.cycles += 4;
            }, // RLC A - 0x07
            0x08 => {
                // Get next two bytes from memory
                let low_byte = self.fetch(memory);
                let high_byte = self.fetch(memory);

                // Set the address to the value of the next two bytes
                let address = ((high_byte as u16) << 8) | (low_byte as u16);

                // Store the value of the stack pointer into the memory location
                // write first byte
                memory.write(address, self.sp as u8);
                // write second byte
                memory.write(address + 1, (self.sp >> 8) as u8);

                self.cycles += 20;
            }, // LD (u16), SP (Opcode 0x08) – Load Stack Pointer into Memory
            0x09 => {
                let hl = self.get_hl();
                let bc = self.get_bc();
                let (result, carry) = hl.overflowing_add(bc);
                self.set_hl(result);
                self.set_n_flag(false);
                // check this
                self.set_h_flag((hl & 0x0FFF) + (bc & 0x0FFF) > 0x0FFF);
                self.set_c_flag(carry);
                self.cycles += 8;
            }, // ADD HL, BC (Opcode 0x09) – Add BC to HL
            




            0x3C => self.a = self.a.wrapping_add(1), // INC A
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }
}

