use super::CPU;
use super::core::Flag;

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
}