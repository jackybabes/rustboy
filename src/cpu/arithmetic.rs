use super::CPU;
use super::core::{Register, RegisterPair, Flag};

use crate::memory::Memory;

impl CPU {
    pub fn increment_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let result = value.wrapping_add(1);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, (value & 0x0F) + 1 > 0x0F);
    }

    pub fn decrement_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let result = value.wrapping_sub(1);
        let half_carry = (value & 0x0F) == 0x00;
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, true);
        self.set_flag(&Flag::H, half_carry);
    }

    pub fn increment_register_pair(&mut self, register_pair: &RegisterPair) {
        let value = self.read_register_pair(register_pair);
        let result = value.wrapping_add(1);
        self.write_register_pair(register_pair, result);
    }

    pub fn decrement_register_pair(&mut self, register_pair: &RegisterPair) {
        let value = self.read_register_pair(register_pair);
        let result = value.wrapping_sub(1);
        self.write_register_pair(register_pair, result);
    }

    pub fn add_register_pair(&mut self, lhs: &RegisterPair, rhs: &RegisterPair) {
        let lhs_value = self.read_register_pair(lhs);
        let rhs_value = self.read_register_pair(rhs);
        let (result, carry) = lhs_value.overflowing_add(rhs_value);
        let half_carry = (lhs_value & 0x0FFF) + (rhs_value & 0x0FFF) > 0x0FFF;
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, half_carry);
        self.set_flag(&Flag::C, carry);
        self.write_register_pair(lhs, result);
    }

    pub fn increment_byte_at_address(&mut self, memory: &mut Memory, address: u16) {
        let value = memory.read_byte(address);
        let result = value.wrapping_add(1);
        memory.write_byte(address, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, (value & 0x0F) + 1 > 0x0F);
    }

    pub fn increment_byte_pointed_by_register_pair(&mut self, memory: &mut Memory, register_pair: &RegisterPair) {
        let address = self.read_register_pair(register_pair);
        self.increment_byte_at_address(memory, address);
    }

    pub fn decrement_byte_at_address(&mut self, memory: &mut Memory, address: u16) {
        let value = memory.read_byte(address);
        let result = value.wrapping_sub(1);
        memory.write_byte(address, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, true);
        self.set_flag(&Flag::H, (value & 0x0F) == 0x0F);
    }

    pub fn decrement_byte_pointed_by_register_pair(&mut self, memory: &mut Memory, register_pair: &RegisterPair) {
        let address = self.read_register_pair(register_pair);
        self.decrement_byte_at_address(memory, address);
    }

    pub fn add_u16_to_register_pair(&mut self, register_pair: &RegisterPair, value_to_add: u16) {
        let current_value = self.read_register_pair(register_pair);
        let (result, carry) = current_value.overflowing_add(value_to_add);
        let half_carry = (current_value & 0x0FFF) + (value_to_add & 0x0FFF) > 0x0FFF;
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, half_carry);
        self.set_flag(&Flag::C, carry);
        self.write_register_pair(register_pair, result);
    }

    pub fn add_u8_to_register(&mut self, register: &Register, value_to_add: u8) {
        let current_value = self.read_register(register);
        let (result, carry) = current_value.overflowing_add(value_to_add);
        let half_carry = (current_value & 0x0F) + (value_to_add & 0x0F) > 0x0F;
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, half_carry);
        self.set_flag(&Flag::C, carry);
        self.write_register(register, result);
    }

    pub fn add_u8_to_register_with_carry(&mut self, register: &Register, value_to_add: u8) {
        let carry = self.get_flag(&Flag::C);
        let value = value_to_add.wrapping_add(carry as u8);
        self.add_u8_to_register(register, value);
    }


    pub fn add_register_to_register(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.add_u8_to_register(lhs, rhs_value);
    }

    pub fn add_register_to_register_with_carry(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.add_u8_to_register_with_carry(lhs, rhs_value);
    }

    pub fn sub_u8_from_register(&mut self, register: &Register, value_to_sub: u8) {
        let current_value = self.read_register(register);
        let (result, carry) = current_value.overflowing_sub(value_to_sub);
        let half_carry = (current_value & 0x0F) < (value_to_sub & 0x0F);
        self.set_flag(&Flag::N, true);
        self.set_flag(&Flag::H, half_carry);
        self.set_flag(&Flag::C, carry);
        self.set_flag(&Flag::Z, result == 0);
        self.write_register(register, result);
    }

    pub fn sub_register_from_register(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.sub_u8_from_register(lhs, rhs_value);
    }

    pub fn sub_u8_from_register_with_carry(&mut self, register: &Register, value_to_sub: u8) {
        let carry = self.get_flag(&Flag::C);
        let value = value_to_sub.wrapping_add(carry as u8);
        self.sub_u8_from_register(register, value);
    }

    pub fn sub_register_from_register_with_carry(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.sub_u8_from_register_with_carry(lhs, rhs_value);
    }

    pub fn add_i8_to_sp(&mut self, offset: u8, sp: u16) -> u16 {
        let signed = offset as i8 as i16;
        let result = (sp as i16).wrapping_add(signed) as u16;
    
        // Flags are based on the lower byte
        let u_sp = sp;
        let u_val = signed as u16;
    
        self.set_flag(&Flag::Z, false);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, ((u_sp & 0xF) + (u_val & 0xF)) > 0xF);
        self.set_flag(&Flag::C, ((u_sp & 0xFF) + (u_val & 0xFF)) > 0xFF);
    
        result
    }
pub fn daa(&mut self, register: &Register) {
    let mut adjustment = 0;
    let value = self.read_register(register);

    // Check if the lower nibble (4 bits) is greater than 9 or if the H flag is set
    if (value & 0x0F) > 0x09 || self.get_flag(&Flag::H) {
        adjustment |= 0x06; // Add 6 to the lower nibble
    }

    // Check if the upper nibble (4 bits) is greater than 9 or if the C flag is set
    if (value & 0xF0) > 0x90 || self.get_flag(&Flag::C) {
        adjustment |= 0x60; // Add 6 to the upper nibble
        self.set_flag(&Flag::C, true); // Set the carry flag
    } else {
        self.set_flag(&Flag::C, false); // Clear the carry flag
    }

    // Apply the adjustment
    self.write_register(register, value.wrapping_add(adjustment));

    // Set the Z flag if the result is zero
    self.set_flag(&Flag::Z, self.read_register(register) == 0);
    // Clear the N flag
    self.set_flag(&Flag::N, false);
    // Clear the H flag
    self.set_flag(&Flag::H, false);
}
    

}
