//! Game Pak: ROM image + optional cartridge RAM + memory bank controller.
//!
//! Supported mappers: no-MBC (32 KiB flat), MBC1, MBC3 (+ a frozen RTC) and
//! MBC5 — enough for the blargg suite and the bulk of the DMG/GBC library.

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
    Mbc3,
    Mbc5,
}

impl Mapper {
    fn from_cart_type(cart_type: u8) -> Mapper {
        match cart_type {
            0x00 | 0x08 | 0x09 => Mapper::None,
            0x01..=0x03 => Mapper::Mbc1,
            0x0F..=0x13 => Mapper::Mbc3,
            0x19..=0x1E => Mapper::Mbc5,
            other => {
                eprintln!("[cartridge] unsupported cart type {other:#04X}, assuming MBC1");
                Mapper::Mbc1
            }
        }
    }
}

const ROM_BANK_SIZE: usize = 0x4000; // 16 KiB
const RAM_BANK_SIZE: usize = 0x2000; // 8 KiB

pub struct Cartridge {
    pub header: Header,
    mapper: Mapper,
    rom: Vec<u8>,
    ram: Vec<u8>,

    ram_enabled: bool,
    /// ROM bank register. MBC1: low 5 bits. MBC3: 7 bits. MBC5: 9 bits.
    rom_bank: u16,
    /// Secondary register: MBC1 upper ROM bits / RAM bank; MBC3 RAM bank or RTC
    /// register select (0x08-0x0C); MBC5 RAM bank.
    ram_bank_sel: u8,
    /// MBC1 only: 0 = simple ROM banking, 1 = RAM / advanced banking.
    advanced_banking: bool,

    /// MBC3 real-time clock: [seconds, minutes, hours, days-low, days-high].
    /// Latched but never ticks — games boot and play; the in-game clock is
    /// simply frozen.
    rtc: [u8; 5],
    rtc_latched: [u8; 5],
    rtc_latch_last: u8,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Cartridge {
        let header = Header::parse(&rom);
        let mapper = Mapper::from_cart_type(header.cart_type);

        // ROM comes straight from the file. Size RAM from the header, but always
        // allocate at least one bank so stray writes never panic.
        let ram = vec![0u8; header.ram_banks.max(1) * RAM_BANK_SIZE];

        Cartridge {
            header,
            mapper,
            rom,
            ram,
            ram_enabled: false,
            rom_bank: 1,
            ram_bank_sel: 0,
            advanced_banking: false,
            rtc: [0; 5],
            rtc_latched: [0; 5],
            rtc_latch_last: 0xFF,
        }
    }

    fn rom_bank_count(&self) -> usize {
        (self.rom.len() / ROM_BANK_SIZE).max(1)
    }

    fn ram_bank_count(&self) -> usize {
        (self.ram.len() / RAM_BANK_SIZE).max(1)
    }

    /// Bank mapped at 0x4000-0x7FFF.
    fn upper_rom_bank(&self) -> usize {
        let bank = match self.mapper {
            Mapper::None => 1,
            Mapper::Mbc1 => {
                let lo = if self.rom_bank & 0x1F == 0 { 1 } else { self.rom_bank & 0x1F };
                ((self.ram_bank_sel as u16) << 5) as usize | lo as usize
            }
            Mapper::Mbc3 => (self.rom_bank as usize & 0x7F).max(1),
            Mapper::Mbc5 => self.rom_bank as usize,
        };
        bank % self.rom_bank_count()
    }

    /// Bank mapped at 0x0000-0x3FFF (non-zero only for large MBC1 carts in
    /// advanced-banking mode).
    fn lower_rom_bank(&self) -> usize {
        match self.mapper {
            Mapper::Mbc1 if self.advanced_banking => {
                ((self.ram_bank_sel as usize) << 5) % self.rom_bank_count()
            }
            _ => 0,
        }
    }

    fn ram_bank(&self) -> usize {
        match self.mapper {
            Mapper::Mbc1 => {
                if self.advanced_banking {
                    self.ram_bank_sel as usize % self.ram_bank_count()
                } else {
                    0
                }
            }
            Mapper::Mbc3 | Mapper::Mbc5 => {
                (self.ram_bank_sel as usize & 0x0F) % self.ram_bank_count()
            }
            Mapper::None => 0,
        }
    }

    /// Read from the ROM regions (0x0000-0x7FFF).
    pub fn read_rom(&self, addr: u16) -> u8 {
        let (bank, offset) = if addr < 0x4000 {
            (self.lower_rom_bank(), addr as usize)
        } else {
            (self.upper_rom_bank(), (addr as usize) - 0x4000)
        };
        self.rom.get(bank * ROM_BANK_SIZE + offset).copied().unwrap_or(0xFF)
    }

    /// Writes to the ROM regions are mapper control registers.
    pub fn write_rom(&mut self, addr: u16, value: u8) {
        match self.mapper {
            Mapper::None => {}

            Mapper::Mbc1 => match addr {
                0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == 0x0A,
                0x2000..=0x3FFF => self.rom_bank = (value & 0x1F) as u16,
                0x4000..=0x5FFF => self.ram_bank_sel = value & 0x03,
                0x6000..=0x7FFF => self.advanced_banking = value & 0x01 != 0,
                _ => {}
            },

            Mapper::Mbc3 => match addr {
                0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == 0x0A,
                0x2000..=0x3FFF => self.rom_bank = (value & 0x7F) as u16,
                0x4000..=0x5FFF => self.ram_bank_sel = value, // 0x00-0x03 RAM, 0x08-0x0C RTC
                0x6000..=0x7FFF => {
                    if self.rtc_latch_last == 0 && value == 1 {
                        self.rtc_latched = self.rtc;
                    }
                    self.rtc_latch_last = value;
                }
                _ => {}
            },

            Mapper::Mbc5 => match addr {
                0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == 0x0A,
                0x2000..=0x2FFF => self.rom_bank = (self.rom_bank & 0x100) | value as u16,
                0x3000..=0x3FFF => {
                    self.rom_bank = (self.rom_bank & 0xFF) | ((value as u16 & 1) << 8)
                }
                0x4000..=0x5FFF => self.ram_bank_sel = value & 0x0F,
                _ => {}
            },
        }
    }

    /// Read cartridge RAM / RTC (0xA000-0xBFFF).
    pub fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        if self.mapper == Mapper::Mbc3 && (0x08..=0x0C).contains(&self.ram_bank_sel) {
            return self.rtc_latched[(self.ram_bank_sel - 0x08) as usize];
        }
        let idx = self.ram_bank() * RAM_BANK_SIZE + (addr as usize - 0xA000);
        self.ram.get(idx).copied().unwrap_or(0xFF)
    }

    /// Write cartridge RAM / RTC (0xA000-0xBFFF).
    pub fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }
        if self.mapper == Mapper::Mbc3 && (0x08..=0x0C).contains(&self.ram_bank_sel) {
            self.rtc[(self.ram_bank_sel - 0x08) as usize] = value;
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

    /// Build a ROM of `banks` * 16 KiB where every byte of bank N equals N
    /// (mod 256), with a plausible header written into bank 0.
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
        cart.write_rom(0x2000, 0);
        assert_eq!(cart.read_rom(0x4000), 1);
    }

    #[test]
    fn mbc1_ram_gated_by_enable() {
        let mut cart = Cartridge::new(synthetic_rom(0x02, 4, 0x02));

        cart.write_ram(0xA000, 0x42);
        assert_eq!(cart.read_ram(0xA000), 0xFF);

        cart.write_rom(0x0000, 0x0A);
        cart.write_ram(0xA000, 0x42);
        assert_eq!(cart.read_ram(0xA000), 0x42);

        cart.write_rom(0x0000, 0x00);
        assert_eq!(cart.read_ram(0xA000), 0xFF);
    }

    #[test]
    fn mbc3_selects_high_banks() {
        // 64 banks (1 MiB) — needs the full 7-bit bank number.
        let mut cart = Cartridge::new(synthetic_rom(0x13, 64, 0x03));
        cart.write_rom(0x2000, 0x2A);
        assert_eq!(cart.read_rom(0x4000), 0x2A);
        cart.write_rom(0x2000, 0x3F);
        assert_eq!(cart.read_rom(0x7FFF), 0x3F);
    }

    #[test]
    fn mbc3_ram_and_rtc() {
        let mut cart = Cartridge::new(synthetic_rom(0x10, 8, 0x03));
        cart.write_rom(0x0000, 0x0A); // enable RAM + RTC

        // RAM bank 2
        cart.write_rom(0x4000, 0x02);
        cart.write_ram(0xA000, 0x99);
        assert_eq!(cart.read_ram(0xA000), 0x99);

        // RTC register 0x08 (seconds): write, then latch, then read back.
        cart.write_rom(0x4000, 0x08);
        cart.write_ram(0xA000, 0x1F);
        cart.write_rom(0x6000, 0x00);
        cart.write_rom(0x6000, 0x01); // latch
        assert_eq!(cart.read_ram(0xA000), 0x1F);
    }

    #[test]
    fn mbc5_nine_bit_rom_bank() {
        let mut cart = Cartridge::new(synthetic_rom(0x1A, 512, 0x03));
        cart.write_rom(0x2000, 0x00);
        cart.write_rom(0x3000, 0x01); // bit 8 -> bank 0x100
        assert_eq!(cart.read_rom(0x4000), 0x00); // bank 256 -> byte value 256 % 256 = 0
        cart.write_rom(0x2000, 0x05);
        cart.write_rom(0x3000, 0x00);
        assert_eq!(cart.read_rom(0x4000), 0x05);
    }
}
