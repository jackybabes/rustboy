use super::CPU;

use crate::memory::Memory;
use super::core::{Register, REGISTER_HL, Flag};

impl CPU {
    pub fn execute_cb_opcode(&mut self, opcode: u8, memory: &mut Memory) {
        let index = opcode & 0x07;
        let bit = (opcode & 0b111000) >> 3;
        match opcode {
            0x00..=0x07 => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.rotate_left_circular_cb(value));
            }, // Rotate left circular
            0x08..=0x0F => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.rotate_right_circular_cb(value));
            }, // Rotate right circular
            0x10..=0x17 => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.rotate_left_carry_cb(value));
            }, // Rotate left carry
            0x18..=0x1F => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.rotate_right_carry_cb(value));
            }, // Rotate right carry
            0x20..=0x27 => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.shift_left_arithmetic_cb(value));
            }, // Shift left arithmetic
            0x28..=0x2F => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.shift_right_arithmetic_cb(value));
            }, // Shift right arithmetic
            0x30..=0x37 => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.swap_nibbles_cb(value));
            }, // Swap nibbles
            0x38..=0x3F => {
                self.cycles += self.run_operation_on_index(memory, index, |cpu, value| cpu.shift_right_logical_cb(value));
            }, // Shift right logical
            0x40..=0x7F => {
                self.cycles += self.run_bit_operation_on_index(
                    memory, index, bit, |cpu, bit, value| cpu.test_bit_cb(bit, value), 12
                );
            }, // Check Bit
            0x80..=0xBF => {
                self.cycles += self.run_res_set_operation_on_index(
                    memory, index, bit, |cpu, bit, value| cpu.reset_bit_cb(bit, value), 16
                );
            }, // Reset Bit
            0xC0..=0xFF => {
                self.cycles += self.run_res_set_operation_on_index(
                    memory, index, bit, |cpu, bit, value| cpu.set_bit_cb(bit, value), 16
                );
            }, // Set Bit
        }
    }

    fn get_register(&mut self, i: u8) -> Option<Register> {
        match i {
            0x00 => Some(Register::B),
            0x01 => Some(Register::C),
            0x02 => Some(Register::D),
            0x03 => Some(Register::E),
            0x04 => Some(Register::H),
            0x05 => Some(Register::L),
            0x06 => None,
            0x07 => Some(Register::A),
            _ => panic!("Invalid register index: {:#X}", i),
        }
    }


    fn run_operation_on_index<F>(&mut self, memory: &mut Memory, index: u8, mut operation: F) -> u16
    where
        F: FnMut(&mut Self, u8) -> u8,
    {
        if let Some(register) = self.get_register(index) {
            let value = self.read_register(&register);
            let result = operation(self, value);
            self.write_register(&register, result);
            return 8;
        } else {
            let address = self.read_register_pair(&REGISTER_HL);
            let value = memory.read_byte(address);
            let result = operation(self, value);
            memory.write_byte(address, result);
            return 16;
        }
    }

    fn run_res_set_operation_on_index<F>(&mut self, memory: &mut Memory, index: u8, bit: u8, mut operation: F, hl_cycles: u16) -> u16
    where
        F: FnMut(&mut Self, u8, u8) -> u8,
    {
        if let Some(register) = self.get_register(index) {
            let value = self.read_register(&register);
            let result = operation(self, bit, value);
            self.write_register(&register, result);
            return 8;
        } else {
            let address = self.read_register_pair(&REGISTER_HL);
            let value = memory.read_byte(address);
            let result = operation(self, bit, value);
            memory.write_byte(address, result);
            return hl_cycles
        }
    }

    fn run_bit_operation_on_index<F>(&mut self, memory: &mut Memory, index: u8, bit: u8, mut operation: F, hl_cycles: u16) -> u16
    where
        F: FnMut(&mut Self, u8, u8),
    {
        if let Some(register) = self.get_register(index) {
            let value = self.read_register(&register);
            operation(self, bit, value);
            return 8;
        } else {
            let address = self.read_register_pair(&REGISTER_HL);
            let value = memory.read_byte(address);
            operation(self, bit, value);
            return hl_cycles
        }
    }

    fn rotate_left_circular_cb(&mut self, value: u8) -> u8{
        let bit7 = (value & 0x80) != 0;
        let result = value.rotate_left(1);
    
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, bit7);

        result
    }

    fn rotate_right_circular_cb(&mut self, value: u8) -> u8{
        let bit0 = (value & 0x01) != 0;
        let result = value.rotate_right(1);

        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, bit0);

        result
    }

    fn rotate_left_carry_cb(&mut self, value: u8) -> u8{
        let bit7 = (value & 0x80) != 0;
        let result = (value << 1 ) | self.get_flag(&Flag::C) as u8;
    
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, bit7);

        result
    }

    fn rotate_right_carry_cb(&mut self, value: u8) -> u8{
        let bit0 = (value & 0x01) != 0;
        let result = (value >> 1) | (self.get_flag(&Flag::C) as u8) << 7;
    
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, bit0);

        result
    }

    fn shift_left_arithmetic_cb(&mut self, value: u8) -> u8 {
        let bit7 = (value & 0x80) != 0;
        let result = value << 1;
    
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, bit7);
    
        result
    }

    fn shift_right_arithmetic_cb(&mut self, value: u8) -> u8 {
        let bit0 = (value & 0x01) != 0;
        let bit7 = value & 0x80;
        let result = (value >> 1) | (bit7 as u8);
    
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, bit0);
    
        result
    }
    
    fn swap_nibbles_cb(&mut self, value: u8) -> u8 {
        let result = (value >> 4) | (value << 4);
    
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, false);
    
        result
    }

    fn shift_right_logical_cb(&mut self, value: u8) -> u8 {
        let bit0 = (value & 0x01) != 0;
        let result = value >> 1;
    
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, bit0);
    
        result
    }

    fn test_bit_cb(&mut self, bit: u8, value: u8){
        let result = value & (1 << bit);

        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, true);
    }

    fn reset_bit_cb(&mut self, bit: u8, value: u8) -> u8{
        let result = value & !(1 << bit);
        result
    }

    fn set_bit_cb(&mut self, bit: u8, value: u8) -> u8{
        let result = value | (1 << bit);
        result
    }
    
    

}
