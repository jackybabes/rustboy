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
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, true);
        self.set_flag(&Flag::H, (value & 0x0F) == 0x0F);
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

    pub fn add_register_to_register(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.add_u8_to_register(lhs, rhs_value);
    }

    pub fn add_register_to_register_with_carry(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        let carry = self.get_flag(&Flag::C);
        self.add_u8_to_register(lhs, rhs_value + carry as u8);
    }

}
