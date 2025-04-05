use super::CPU;

impl CPU {
    pub fn jr(&mut self, offset: i8) {
        self.pc = self.pc.wrapping_add(offset as u16);
    }
}