use super::CPU;
use super::core::{Register, Flag};

impl CPU {
    pub fn rotate_left_circular(&mut self, register: &Register) {
        let value = self.read_register(register);
        let carry = value & 0x80 != 0;
        let result = value.rotate_left(1);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, false);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, carry);
    }

    // pub fn rotate_left_circular_address(&mut self, address: u16, memory: &mut Memory) {
    //     let value = memory.read_byte(address);
    //     let carry = value & 0x80 != 0;
    //     let result = value.rotate_left(1);
    //     memory.write_byte(address, result);
    //     self.set_flag(&Flag::Z, false);
    //     self.set_flag(&Flag::N, false);
    //     self.set_flag(&Flag::H, false);
    //     self.set_flag(&Flag::C, carry); 
    // }


    pub fn rotate_right_circular(&mut self, register: &Register) {
        let value = self.read_register(register);
        let carry = value & 0x01 != 0;
        let result = value.rotate_right(1);
        self.write_register(register, result);
        self.set_flag(&Flag::Z, false);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, carry);
    }
    pub fn rotate_left_through_carry(&mut self, register: &Register) {
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
    pub fn rotate_right_through_carry(&mut self, register: &Register) {
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
    pub fn complement_register(&mut self, register: &Register) {
        let value = self.read_register(register);
        let result = !value;
        self.write_register(register, result);
        self.set_flag(&Flag::N, true);
        self.set_flag(&Flag::H, true);
    }

    pub fn complement_carry_flag(&mut self) {
        let carry = self.get_flag(&Flag::C);
        self.set_flag(&Flag::C, !carry);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
    }

    pub fn and_u8_with_register(&mut self, register: &Register, value: u8) {
        let current_value = self.read_register(register);
        let result = current_value & value;
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, true);
        self.set_flag(&Flag::C, false);
    }

    pub fn and_register_with_register(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.and_u8_with_register(lhs, rhs_value);
    }

    pub fn xor_u8_with_register(&mut self, register: &Register, value: u8) {
        let current_value = self.read_register(register);
        let result = current_value ^ value;
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, false);
    }

    pub fn xor_register_with_register(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.xor_u8_with_register(lhs, rhs_value);
    }

    pub fn or_u8_with_register(&mut self, register: &Register, value: u8) {
        let current_value = self.read_register(register);
        let result = current_value | value;
        self.write_register(register, result);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, false);
        self.set_flag(&Flag::H, false);
        self.set_flag(&Flag::C, false);
    }

    pub fn or_register_with_register(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.or_u8_with_register(lhs, rhs_value);
    }

    pub fn compare_u8_with_register(&mut self, register: &Register, value: u8) {
        let current_value = self.read_register(register);
        let result = current_value.wrapping_sub(value);
        self.set_flag(&Flag::Z, result == 0);
        self.set_flag(&Flag::N, true);
        self.set_flag(&Flag::H, (current_value & 0x0F) < (value & 0x0F));
        self.set_flag(&Flag::C, current_value < value);
    }

    pub fn compare_register_with_register(&mut self, lhs: &Register, rhs: &Register) {
        let rhs_value = self.read_register(rhs);
        self.compare_u8_with_register(lhs, rhs_value);
    }
    
}