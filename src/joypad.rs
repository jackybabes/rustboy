//! Joypad register (0xFF00 / P1).
//!
//! The CPU selects a button group by driving bit 4 (d-pad) or bit 5 (action
//! buttons) low, then reads the pressed state in bits 0-3 (0 = pressed).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

#[derive(Default)]
pub struct Joypad {
    /// One bit per `Button` (see `mask`), 1 = currently pressed.
    pressed: u8,
    /// Group-select bits (4 and 5) as last written by the CPU.
    select: u8,
}

fn mask(b: Button) -> u8 {
    1 << (b as u8)
}

impl Joypad {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, b: Button, down: bool) {
        if down {
            self.pressed |= mask(b);
        } else {
            self.pressed &= !mask(b);
        }
    }

    /// CPU write to 0xFF00 — only the group-select bits are writable.
    pub fn write_select(&mut self, value: u8) {
        self.select = value & 0x30;
    }

    /// CPU read of 0xFF00.
    pub fn read(&self) -> u8 {
        let mut lower = 0x0F;

        if self.select & 0x10 == 0 {
            // d-pad selected
            if self.pressed & mask(Button::Right) != 0 {
                lower &= !0x01;
            }
            if self.pressed & mask(Button::Left) != 0 {
                lower &= !0x02;
            }
            if self.pressed & mask(Button::Up) != 0 {
                lower &= !0x04;
            }
            if self.pressed & mask(Button::Down) != 0 {
                lower &= !0x08;
            }
        }
        if self.select & 0x20 == 0 {
            // action buttons selected
            if self.pressed & mask(Button::A) != 0 {
                lower &= !0x01;
            }
            if self.pressed & mask(Button::B) != 0 {
                lower &= !0x02;
            }
            if self.pressed & mask(Button::Select) != 0 {
                lower &= !0x04;
            }
            if self.pressed & mask(Button::Start) != 0 {
                lower &= !0x08;
            }
        }

        0xC0 | (self.select & 0x30) | lower
    }
}
