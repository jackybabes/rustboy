//! Game Pak: ROM image + optional cartridge RAM + memory bank controller.
//!
//! Supports the two mappers needed to run the blargg test suite and most early
//! commercial games: no-MBC (32 KiB flat) and MBC1.

/// Decoded cartridge header fields (see https://gbdev.io/pandocs/The_Cartridge_Header.html).
#[derive(Debug, Clone)]
pub struct Header {
    pub title: String,
    pub cart_type: u8,
    pub rom_banks: usize,
    pub ram_banks: usize,
}

impl Header {
    fn parse(rom: &[u8]) -> Header {
        let title_bytes = rom.get(0x0134..0x0144).unwrap_or(&[]);
        let title = title_bytes
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as char)
            .collect::<String>();

        let cart_type = rom.get(0x0147).copied().unwrap_or(0);

        // ROM size: 32 KiB << N  =>  (2 << N) banks of 16 KiB.
        let rom_banks = match rom.get(0x0148).copied().unwrap_or(0) {
            n @ 0x00..=0x08 => 2usize << n,
            _ => 2,
        };

        // RAM size code -> number of 8 KiB banks.
        let ram_banks = match rom.get(0x0149).copied().unwrap_or(0) {
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => 0,
        };

        Header { title, cart_type, rom_banks, ram_banks }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mapper {
    None,
    Mbc1,
}

const ROM_BANK_SIZE: usize = 0x4000; // 16 KiB
const RAM_BANK_SIZE: usize = 0x2000; // 8 KiB

pub struct Cartridge {
    pub header: Header,
    mapper: Mapper,
    rom: Vec<u8>,
    ram: Vec<u8>,

    ram_enabled: bool,
    /// Lower 5 bits of the ROM bank number (MBC1 register at 0x2000-0x3FFF).
    rom_bank_lo: u8,
    /// 2-bit register at 0x4000-0x5FFF: ROM bank high bits, or RAM bank.
    bank_hi: u8,
    /// Banking mode select (0 = simple ROM banking, 1 = RAM / advanced).
    advanced_banking: bool,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Cartridge {
        let header = Header::parse(&rom);

        let mapper = match header.cart_type {
            0x00 | 0x08 | 0x09 => Mapper::None,
            0x01..=0x03 => Mapper::Mbc1,
            other => {
                eprintln!(
                    "[cartridge] unsupported cart type {other:#04X}, treating as MBC1"
                );
                Mapper::Mbc1
            }
        };

        // ROM comes straight from the file. Size RAM from the header, but always
        // allocate at least one bank so stray writes never panic.
        let ram = vec![0u8; header.ram_banks.max(1) * RAM_BANK_SIZE];

        Cartridge {
            header,
            mapper,
            rom,
            ram,
            ram_enabled: false,
            rom_bank_lo: 1,
            bank_hi: 0,
            advanced_banking: false,
        }
    }

    fn rom_bank_count(&self) -> usize {
        (self.rom.len() / ROM_BANK_SIZE).max(1)
    }

    fn ram_bank_count(&self) -> usize {
        (self.ram.len() / RAM_BANK_SIZE).max(1)
    }

    /// Effective bank mapped at 0x4000-0x7FFF.
    fn upper_rom_bank(&self) -> usize {
        match self.mapper {
            Mapper::None => 1,
            Mapper::Mbc1 => {
                let lo = if self.rom_bank_lo == 0 { 1 } else { self.rom_bank_lo } as usize;
                let bank = ((self.bank_hi as usize) << 5) | (lo & 0x1F);
                bank & (self.rom_bank_count() - 1)
            }
        }
    }

    /// Effective bank mapped at 0x0000-0x3FFF (only ever non-zero for large
    /// MBC1 carts in advanced-banking mode).
    fn lower_rom_bank(&self) -> usize {
        match self.mapper {
            Mapper::Mbc1 if self.advanced_banking => {
                ((self.bank_hi as usize) << 5) & (self.rom_bank_count() - 1)
            }
            _ => 0,
        }
    }

    fn ram_bank(&self) -> usize {
        match self.mapper {
            Mapper::Mbc1 if self.advanced_banking => {
                (self.bank_hi as usize) & (self.ram_bank_count() - 1)
            }
            _ => 0,
        }
    }

    /// Read from the ROM regions (0x0000-0x7FFF).
    pub fn read_rom(&self, addr: u16) -> u8 {
        let (bank, offset) = if addr < 0x4000 {
            (self.lower_rom_bank(), addr as usize)
        } else {
            (self.upper_rom_bank(), (addr as usize) - 0x4000)
        };
        let idx = bank * ROM_BANK_SIZE + offset;
        self.rom.get(idx).copied().unwrap_or(0xFF)
    }

    /// Writes to the ROM regions are mapper control registers.
    pub fn write_rom(&mut self, addr: u16, value: u8) {
        match self.mapper {
            Mapper::None => {}
            Mapper::Mbc1 => match addr {
                0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == 0x0A,
                0x2000..=0x3FFF => self.rom_bank_lo = value & 0x1F,
                0x4000..=0x5FFF => self.bank_hi = value & 0x03,
                0x6000..=0x7FFF => self.advanced_banking = value & 0x01 != 0,
                _ => {}
            },
        }
    }

    /// Read cartridge RAM (0xA000-0xBFFF).
    pub fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_enabled || self.ram.is_empty() {
            return 0xFF;
        }
        let idx = self.ram_bank() * RAM_BANK_SIZE + (addr as usize - 0xA000);
        self.ram.get(idx).copied().unwrap_or(0xFF)
    }

    /// Write cartridge RAM (0xA000-0xBFFF).
    pub fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled || self.ram.is_empty() {
            return;
        }
        let idx = self.ram_bank() * RAM_BANK_SIZE + (addr as usize - 0xA000);
        if let Some(slot) = self.ram.get_mut(idx) {
            *slot = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a ROM of `banks` * 16 KiB where every byte of bank N equals N,
    /// with a plausible header written into bank 0.
    fn synthetic_rom(cart_type: u8, banks: usize, ram_code: u8) -> Vec<u8> {
        let mut rom = vec![0u8; banks * ROM_BANK_SIZE];
        for (bank, chunk) in rom.chunks_mut(ROM_BANK_SIZE).enumerate() {
            chunk.fill(bank as u8);
        }
        for (i, b) in b"TESTROM".iter().enumerate() {
            rom[0x0134 + i] = *b;
        }
        rom[0x0147] = cart_type;
        rom[0x0148] = banks.trailing_zeros().saturating_sub(1) as u8; // 2<<n == banks
        rom[0x0149] = ram_code;
        rom
    }

    #[test]
    fn parses_header() {
        let cart = Cartridge::new(synthetic_rom(0x01, 4, 0x02));
        assert_eq!(cart.header.title, "TESTROM");
        assert_eq!(cart.header.cart_type, 0x01);
        assert_eq!(cart.header.rom_banks, 4);
        assert_eq!(cart.header.ram_banks, 1);
    }

    #[test]
    fn no_mbc_is_flat() {
        let cart = Cartridge::new(synthetic_rom(0x00, 2, 0x00));
        assert_eq!(cart.read_rom(0x0000), 0);
        assert_eq!(cart.read_rom(0x3FFF), 0);
        assert_eq!(cart.read_rom(0x4000), 1);
        assert_eq!(cart.read_rom(0x7FFF), 1);
    }

    #[test]
    fn mbc1_switches_upper_bank() {
        let mut cart = Cartridge::new(synthetic_rom(0x01, 8, 0x00));

        // Default: bank 1 at 0x4000-0x7FFF, bank 0 fixed at 0x0000-0x3FFF.
        assert_eq!(cart.read_rom(0x0000), 0);
        assert_eq!(cart.read_rom(0x4000), 1);

        cart.write_rom(0x2000, 5);
        assert_eq!(cart.read_rom(0x4000), 5);

        cart.write_rom(0x2000, 7);
        assert_eq!(cart.read_rom(0x6000), 7);
    }

    #[test]
    fn mbc1_bank0_maps_to_bank1() {
        let mut cart = Cartridge::new(synthetic_rom(0x01, 4, 0x00));
        cart.write_rom(0x2000, 0); // selecting 0 must behave as 1
        assert_eq!(cart.read_rom(0x4000), 1);
    }

    #[test]
    fn mbc1_ram_gated_by_enable() {
        let mut cart = Cartridge::new(synthetic_rom(0x02, 4, 0x02));

        // Disabled by default.
        cart.write_ram(0xA000, 0x42);
        assert_eq!(cart.read_ram(0xA000), 0xFF);

        cart.write_rom(0x0000, 0x0A); // enable
        cart.write_ram(0xA000, 0x42);
        assert_eq!(cart.read_ram(0xA000), 0x42);

        cart.write_rom(0x0000, 0x00); // disable again
        assert_eq!(cart.read_ram(0xA000), 0xFF);
    }
}
