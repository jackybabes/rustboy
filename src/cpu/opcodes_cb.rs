use super::CPU;

use crate::memory::Memory;
use super::core::{Register, REGISTER_HL};

impl CPU {
    pub fn execute_cb_opcode(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00..=0x07 => {
                let offset= opcode & 0x07;
                if offset == 0x06 {
                    let address = self.read_register_pair(&REGISTER_HL);
                    self.rotate_left_circular_address(address, memory);
                    self.cycles += 16
                } else if let Some(register) = self.get_register(offset) {
                    self.rotate_left_circular_register(&register);
                    self.cycles += 8;
                }
            }, // Rotate left circular
            _ => panic!("Invalid CB opcode: {:#X}", opcode),
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
    // Similarly define `rl`, `sla`, `swap`, etc.

}
