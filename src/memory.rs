use crate::data::HardwareRegister;

// Start	    End	Description	Notes
// 0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
// 4000	7FFF	16 KiB ROM Bank 01–NN	From cartridge, switchable bank via mapper (if any)
// 8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
// A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
// C000	CFFF	4 KiB Work RAM (WRAM)	
// D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1–7
// E000	FDFF	Echo RAM (mirror of C000–DDFF)	Nintendo says use of this area is prohibited.
// FE00	FE9F	Object attribute memory (OAM)	
// FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited.
// FF00	FF7F	I/O Registers	
// FF80	FFFE	High RAM (HRAM)	
// FFFF	FFFF	Interrupt Enable register (IE)	

pub struct Memory {
    data: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 0x10000] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        // For test rom
        if address == HardwareRegister::LY as u16 {
            return 0x90;
        }
        self.data[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.data[address as usize] = (value & 0xFF) as u8;
        self.data[address as usize + 1] = (value >> 8) as u8;
    }

    pub fn read_hardware_register(&self, register: HardwareRegister) -> u8 {
        self.data[register as usize]
    }

    pub fn write_hardware_register(&mut self, register: HardwareRegister, value: u8) {
        self.data[register as usize] = value;
    }

    pub fn load_test_rom(&mut self) {
        let rom = include_bytes!("/Users/jack/Code/rustboy/roms/gb-test-roms/mem_timing/mem_timing.gb");
        for (i, byte) in rom.iter().enumerate() {
            self.data[i] = *byte;
        }
    }
}