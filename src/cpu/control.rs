use super::CPU;
use super::core::Flag;
use crate::memory::Memory;

impl CPU {
    pub fn jump_relative(&mut self, offset: i8) {
        self.pc = self.pc.wrapping_add(offset as u16);
    }

    pub fn jump_relative_if_not_zero(&mut self, offset: i8) -> bool {
        if self.get_flag(&Flag::Z) {
            false
        } else {
            self.jump_relative(offset);
            true
        }
    }

    pub fn jump_relative_if_zero(&mut self, offset: i8) -> bool {
        if self.get_flag(&Flag::Z) {
            self.jump_relative(offset);
            true
        } else {
            false
        }
    }

    pub fn jump_relative_if_carry(&mut self, offset: i8) -> bool {
        if self.get_flag(&Flag::C) {
            self.jump_relative(offset);
            true
        } else {
            false
        }
    }

    pub fn jump_relative_if_not_carry(&mut self, offset: i8) -> bool {
        if self.get_flag(&Flag::C) {
            false
        } else {
            self.jump_relative(offset);
            true
        }
    }

    pub fn call(&mut self, memory: &mut Memory, address: u16) {
        self.push_u16(memory, self.pc);
        self.pc = address;
    }

    pub fn ret(&mut self, memory: &mut Memory) {
        self.pc = self.pop_u16(memory);
    }

    pub fn jump_to_address(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn execute_if_flag_set_to_condition<F>(&mut self, condition: bool, flag: &Flag, func: F, memory: &mut Memory) -> bool 
    where 
        F: FnOnce(&mut Self, &mut Memory),
    {
        if self.get_flag(flag) == condition {
            func(self, memory);
            return true;
        }
        false
    }
}