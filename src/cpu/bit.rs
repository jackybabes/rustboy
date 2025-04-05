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
    
}