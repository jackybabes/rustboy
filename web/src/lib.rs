//! WebAssembly bindings: a thin wrapper around `rustboy::GameBoy` for the
//! browser frontend in `web/index.html`.

use rustboy::GameBoy;
use rustboy::gameboy_doctor::gb_doc_set_inital_registers;
use rustboy::joypad::Button;
use rustboy::ppu::{SCREEN_H, SCREEN_W};
use wasm_bindgen::prelude::*;

/// DMG shade (0..3) -> RGB, same soft-green LCD look as the desktop build.
const SHADES: [[u8; 3]; 4] = [
    [0xE0, 0xF8, 0xD0],
    [0x88, 0xC0, 0x70],
    [0x34, 0x68, 0x56],
    [0x08, 0x18, 0x20],
];

/// Route Rust panics (e.g. an illegal opcode from a bad ROM) to the browser
/// console instead of an opaque `unreachable` trap.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Emulator {
    gb: GameBoy,
    rgba: Vec<u8>,
}

#[wasm_bindgen]
impl Emulator {
    /// Boot a cartridge from raw ROM bytes.
    #[wasm_bindgen(constructor)]
    pub fn new(rom: &[u8]) -> Emulator {
        let mut gb = GameBoy::new();
        gb.load_rom(rom.to_vec());
        gb_doc_set_inital_registers(&mut gb.cpu);
        Emulator { gb, rgba: vec![0xFF; SCREEN_W * SCREEN_H * 4] }
    }

    pub fn width() -> usize {
        SCREEN_W
    }

    pub fn height() -> usize {
        SCREEN_H
    }

    /// Cartridge title from the ROM header.
    pub fn title(&self) -> String {
        self.gb
            .memory
            .cartridge()
            .map(|c| c.header.title.clone())
            .unwrap_or_default()
    }

    /// Emulate one full frame (70224 dots).
    pub fn run_frame(&mut self) {
        self.gb.run_frame();
    }

    /// The current frame as RGBA bytes (160 * 144 * 4), ready for `ImageData`.
    pub fn frame_rgba(&mut self) -> Vec<u8> {
        for (px, &shade) in self.rgba.as_chunks_mut::<4>().0.iter_mut().zip(self.gb.framebuffer()) {
            px[..3].copy_from_slice(&SHADES[shade as usize]);
        }
        self.rgba.clone()
    }

    /// Press / release a button by name: right, left, up, down, a, b, select, start.
    pub fn set_button(&mut self, name: &str, down: bool) {
        let button = match name {
            "right" => Button::Right,
            "left" => Button::Left,
            "up" => Button::Up,
            "down" => Button::Down,
            "a" => Button::A,
            "b" => Button::B,
            "select" => Button::Select,
            "start" => Button::Start,
            _ => return,
        };
        self.gb.memory.joypad.set(button, down);
    }
}
