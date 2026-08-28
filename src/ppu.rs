//! Pixel Processing Unit: LCD timing state machine + scanline renderer.
//!
//! The MMU stays the single source of truth for VRAM, OAM and the LCD
//! registers (0xFF40-0xFF4B); the PPU is clocked from `GameBoy::tick` exactly
//! like the timer, reads what it needs through `Memory`, writes `LY`/`STAT`
//! back, raises the VBlank and STAT interrupts, and renders into its own
//! framebuffer.

use crate::data::HardwareRegister as R;
use crate::memory::Memory;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

const DOTS_PER_LINE: u32 = 456;
const LINES_PER_FRAME: u8 = 154;
const OAM_SCAN_DOTS: u32 = 80;
const DRAW_DOTS: u32 = 172;
const VBLANK_START_LINE: u8 = 144;

const INT_VBLANK: u8 = 1 << 0;
const INT_STAT: u8 = 1 << 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    /// Final shade (0 = lightest .. 3 = darkest) per pixel, row-major.
    framebuffer: Vec<u8>,
    /// Set when a full frame has just finished; cleared by `take_frame`.
    pub frame_ready: bool,

    mode: Mode,
    /// Dots elapsed in the current scanline (0..456).
    line_dots: u32,
    ly: u8,
    /// Window's own line counter (only advances on lines the window drew).
    window_line: u8,

    lcd_on: bool,
    /// Rising-edge tracker for the (level-triggered) STAT interrupt line.
    stat_line: bool,

    /// BG/window colour index (pre-palette, 0..3) for the scanline being drawn,
    /// used for sprite-priority decisions.
    bg_index: [u8; SCREEN_W],
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            framebuffer: vec![0; SCREEN_W * SCREEN_H],
            frame_ready: false,
            mode: Mode::OamScan,
            line_dots: 0,
            ly: 0,
            window_line: 0,
            lcd_on: true,
            stat_line: false,
            bg_index: [0; SCREEN_W],
        }
    }

    /// The most recent complete frame as shade indices (0..3), length 160*144.
    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    /// Take the finished-frame flag (returns true once per completed frame).
    pub fn take_frame(&mut self) -> bool {
        std::mem::take(&mut self.frame_ready)
    }

    pub fn step(&mut self, cycles: u16, mem: &mut Memory) {
        let lcdc = mem.read_byte(R::LCDC as u16);
        let lcd_on = lcdc & 0x80 != 0;

        // Handle the LCD being turned on/off.
        if lcd_on != self.lcd_on {
            self.lcd_on = lcd_on;
            if !lcd_on {
                self.mode = Mode::HBlank;
                self.line_dots = 0;
                self.ly = 0;
                self.window_line = 0;
                self.stat_line = false;
                mem.write_byte(R::LY as u16, 0);
                self.write_stat(mem);
                return;
            } else {
                self.mode = Mode::OamScan;
            }
        }
        if !lcd_on {
            return;
        }

        self.line_dots += cycles as u32;

        // Advance whole scanlines.
        while self.line_dots >= DOTS_PER_LINE {
            self.line_dots -= DOTS_PER_LINE;
            self.ly = (self.ly + 1) % LINES_PER_FRAME;

            if self.ly == 0 {
                self.window_line = 0;
            }
            if self.ly == VBLANK_START_LINE {
                mem.request_interrupt(INT_VBLANK);
                self.frame_ready = true;
            }
        }

        // Derive the mode for the current position in the line.
        let new_mode = if self.ly >= VBLANK_START_LINE {
            Mode::VBlank
        } else if self.line_dots < OAM_SCAN_DOTS {
            Mode::OamScan
        } else if self.line_dots < OAM_SCAN_DOTS + DRAW_DOTS {
            Mode::Drawing
        } else {
            Mode::HBlank
        };

        // Render the scanline once, when drawing ends for a visible line.
        if new_mode == Mode::HBlank && self.mode != Mode::HBlank {
            self.render_scanline(mem);
        }
        self.mode = new_mode;

        mem.write_byte(R::LY as u16, self.ly);
        self.write_stat(mem);
        self.update_stat_interrupt(mem);
    }

    /// Recompose STAT: keep the CPU-writable enable bits, refresh mode +
    /// LYC-coincidence, force bit 7.
    fn write_stat(&self, mem: &mut Memory) {
        let lyc = mem.read_byte(R::LYC as u16);
        let coincidence = self.ly == lyc;
        let prev = mem.read_byte(R::STAT as u16);
        let stat =
            0x80 | (prev & 0x78) | ((coincidence as u8) << 2) | (self.mode as u8);
        mem.write_byte(R::STAT as u16, stat);
    }

    fn update_stat_interrupt(&mut self, mem: &mut Memory) {
        let stat = mem.read_byte(R::STAT as u16);
        let line = (stat & 0x08 != 0 && self.mode == Mode::HBlank)
            || (stat & 0x10 != 0 && self.mode == Mode::VBlank)
            || (stat & 0x20 != 0 && self.mode == Mode::OamScan)
            || (stat & 0x40 != 0 && stat & 0x04 != 0);

        if line && !self.stat_line {
            mem.request_interrupt(INT_STAT);
        }
        self.stat_line = line;
    }

    fn render_scanline(&mut self, mem: &Memory) {
        let ly = self.ly;
        let lcdc = mem.read_byte(R::LCDC as u16);
        let vram = mem.vram();

        let bg_enable = lcdc & 0x01 != 0;
        let win_enable = lcdc & 0x20 != 0;
        let signed_tiles = lcdc & 0x10 == 0;
        let bg_map: usize = if lcdc & 0x08 != 0 { 0x1C00 } else { 0x1800 };
        let win_map: usize = if lcdc & 0x40 != 0 { 0x1C00 } else { 0x1800 };

        let scy = mem.read_byte(R::SCY as u16);
        let scx = mem.read_byte(R::SCX as u16);
        let wy = mem.read_byte(R::WY as u16);
        let wx = mem.read_byte(R::WX as u16);
        let bgp = mem.read_byte(R::BGP as u16);

        let fetch_tile_row = |tile: u8, row: u8| -> (u8, u8) {
            let base = if signed_tiles {
                (0x1000i32 + (tile as i8 as i32) * 16) as usize
            } else {
                (tile as usize) * 16
            };
            let addr = base + (row as usize) * 2;
            (vram[addr], vram[addr + 1])
        };

        let window_here = win_enable && wy <= ly && wx < 167;
        let mut drew_window = false;

        for x in 0..SCREEN_W {
            let in_window = window_here && (x as i32) >= (wx as i32 - 7);

            let (map, tx, ty) = if in_window {
                drew_window = true;
                let wx0 = (wx as i32 - 7).max(0) as usize;
                (win_map, (x - wx0) as u8, self.window_line)
            } else if bg_enable {
                (bg_map, x.wrapping_add(scx as usize) as u8, ly.wrapping_add(scy))
            } else {
                self.bg_index[x] = 0;
                self.framebuffer[ly as usize * SCREEN_W + x] = 0;
                continue;
            };

            let tile_idx = vram[map + (ty as usize / 8) * 32 + (tx as usize / 8)];
            let (lo, hi) = fetch_tile_row(tile_idx, ty & 7);
            let bit = 7 - (tx & 7);
            let color = (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1);

            self.bg_index[x] = color;
            self.framebuffer[ly as usize * SCREEN_W + x] = shade(bgp, color);
        }

        if drew_window {
            self.window_line = self.window_line.wrapping_add(1);
        }

        if lcdc & 0x02 != 0 {
            self.render_sprites(mem, ly, lcdc);
        }
    }

    fn render_sprites(&mut self, mem: &Memory, ly: u8, lcdc: u8) {
        let oam = mem.oam();
        let vram = mem.vram();
        let height: i16 = if lcdc & 0x04 != 0 { 16 } else { 8 };
        let obp0 = mem.read_byte(R::OBP0 as u16);
        let obp1 = mem.read_byte(R::OBP1 as u16);

        // Select up to 10 sprites for this line, in OAM order.
        let mut chosen: Vec<usize> = Vec::with_capacity(10);
        for i in 0..40 {
            let oy = oam[i * 4] as i16 - 16;
            if (ly as i16) >= oy && (ly as i16) < oy + height {
                chosen.push(i);
                if chosen.len() == 10 {
                    break;
                }
            }
        }

        // DMG priority: smaller X wins; ties broken by lower OAM index. Draw
        // lowest priority first so the winner lands on top.
        chosen.sort_by_key(|&i| (oam[i * 4 + 1] as i16, i as i16));
        chosen.reverse();

        for i in chosen {
            let oy = oam[i * 4] as i16 - 16;
            let ox = oam[i * 4 + 1] as i16 - 8;
            let mut tile = oam[i * 4 + 2];
            let flags = oam[i * 4 + 3];

            let flip_x = flags & 0x20 != 0;
            let flip_y = flags & 0x40 != 0;
            let behind_bg = flags & 0x80 != 0;
            let palette = if flags & 0x10 != 0 { obp1 } else { obp0 };

            let mut row = (ly as i16 - oy) as u8;
            if flip_y {
                row = (height as u8 - 1) - row;
            }
            if height == 16 {
                tile &= 0xFE;
            }
            let addr = (tile as usize) * 16 + (row as usize) * 2;
            let lo = vram[addr];
            let hi = vram[addr + 1];

            for px in 0..8i16 {
                let sx = ox + px;
                if sx < 0 || sx >= SCREEN_W as i16 {
                    continue;
                }
                let bit = if flip_x { px as u8 } else { 7 - px as u8 };
                let color = (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1);
                if color == 0 {
                    continue; // transparent
                }
                if behind_bg && self.bg_index[sx as usize] != 0 {
                    continue;
                }
                self.framebuffer[ly as usize * SCREEN_W + sx as usize] =
                    shade(palette, color);
            }
        }
    }
}

/// Map a 2-bit colour index through a DMG palette register to a final shade.
fn shade(palette: u8, color: u8) -> u8 {
    (palette >> (color * 2)) & 0x3
}
