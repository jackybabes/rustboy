use crate::cartridge::Cartridge;
use crate::data::HardwareRegister;
use crate::joypad::Joypad;

// Memory map (https://gbdev.io/pandocs/Memory_Map.html)
// 0000-3FFF  ROM bank 00                (cartridge)
// 4000-7FFF  ROM bank 01-NN             (cartridge, switchable)
// 8000-9FFF  Video RAM (VRAM)
// A000-BFFF  External RAM               (cartridge, switchable)
// C000-DFFF  Work RAM (WRAM)
// E000-FDFF  Echo RAM (mirror of C000-DDFF)
// FE00-FE9F  Object attribute memory (OAM)
// FEA0-FEFF  Not usable
// FF00-FF7F  I/O registers
// FF80-FFFE  High RAM (HRAM)
// FFFF       Interrupt Enable register (IE)

const VRAM_SIZE: usize = 0x2000;
const WRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0x00A0;
const IO_SIZE: usize = 0x0080;
const HRAM_SIZE: usize = 0x007F;

pub struct Memory {
    cartridge: Option<Cartridge>,
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
    io: [u8; IO_SIZE],
    hram: [u8; HRAM_SIZE],
    ie: u8,
    pub joypad: Joypad,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            cartridge: None,
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            ie: 0,
            joypad: Joypad::new(),
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }

    pub fn cartridge(&self) -> Option<&Cartridge> {
        self.cartridge.as_ref()
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self
                .cartridge
                .as_ref()
                .map_or(0xFF, |c| c.read_rom(address)),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self
                .cartridge
                .as_ref()
                .map_or(0xFF, |c| c.read_ram(address)),
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize], // echo
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF, // not usable
            0xFF00 => self.joypad.read(),
            0xFF01..=0xFF7F => self.io[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {
                if let Some(c) = self.cartridge.as_mut() {
                    c.write_rom(address, value);
                }
            }
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => {
                if let Some(c) = self.cartridge.as_mut() {
                    c.write_ram(address, value);
                }
            }
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value, // echo
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {} // not usable
            0xFF00 => self.joypad.write_select(value),
            0xFF46 => {
                self.io[0x46] = value;
                self.oam_dma(value);
            }
            0xFF01..=0xFF45 | 0xFF47..=0xFF7F => {
                self.io[(address - 0xFF00) as usize] = value
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
        }
    }

    /// OAM DMA: copy 0xXX00-0xXX9F into OAM. Real hardware takes 160 M-cycles;
    /// we do it instantly, which no test or game depends on being slower.
    fn oam_dma(&mut self, high: u8) {
        let src = (high as u16) << 8;
        for i in 0..OAM_SIZE as u16 {
            self.oam[i as usize] = self.read_byte(src + i);
        }
    }

    /// Raise interrupt request bits in IF (0xFF0F). Used by the PPU/timer.
    pub fn request_interrupt(&mut self, bits: u8) {
        self.io[0x0F] |= bits;
    }

    /// Direct VRAM view for the renderer (0x8000-0x9FFF).
    pub fn vram(&self) -> &[u8] {
        &self.vram
    }

    /// Direct OAM view for the renderer (0xFE00-0xFE9F).
    pub fn oam(&self) -> &[u8] {
        &self.oam
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn read_hardware_register(&self, register: HardwareRegister) -> u8 {
        self.read_byte(register as u16)
    }

    pub fn write_hardware_register(&mut self, register: HardwareRegister, value: u8) {
        self.write_byte(register as u16, value);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
