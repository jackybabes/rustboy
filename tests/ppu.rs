//! Smoke tests for the PPU: boot a ROM for a while and check the framebuffer
//! actually contains rendered graphics (not a blank or uniform screen).
//!
//! The blargg-based tests use ROMs committed under `test-roms/`. The
//! commercial-game test looks under the git-ignored `roms/` dir and skips
//! itself if the ROM isn't there, so a fresh clone still passes.

use std::path::{Path, PathBuf};

use rustboy::gameboy_doctor::gb_doc_set_inital_registers;
use rustboy::ppu::{SCREEN_H, SCREEN_W};
use rustboy::GameBoy;

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn boot(path: &Path, frames: usize) -> GameBoy {
    let mut gb = GameBoy::new();
    gb.load_rom(std::fs::read(path).unwrap());
    gb_doc_set_inital_registers(&mut gb.cpu);
    for _ in 0..frames {
        gb.run_frame();
    }
    gb
}

fn shade_histogram(fb: &[u8]) -> [usize; 4] {
    let mut h = [0usize; 4];
    for &s in fb {
        h[s as usize] += 1;
    }
    h
}

#[test]
fn framebuffer_has_expected_shape() {
    let rom = crate_dir().join("test-roms/blargg/cpu_instrs/cpu_instrs.gb");
    let gb = boot(&rom, 60);
    assert_eq!(gb.framebuffer().len(), SCREEN_W * SCREEN_H);
}

#[test]
fn blargg_output_is_rendered_as_tiles() {
    // The blargg ROM prints "cpu_instrs" etc. to the background tilemap.
    let rom = crate_dir().join("test-roms/blargg/cpu_instrs/cpu_instrs.gb");
    let gb = boot(&rom, 200);
    let hist = shade_histogram(gb.framebuffer());
    let ink = hist[1] + hist[2] + hist[3];
    assert!(ink > 100, "expected rendered text, got histogram {hist:?}");
}

#[test]
fn commercial_rom_title_screen_renders() {
    // Needs a real cartridge dump; not committed. Skip if it's not present.
    let rom = crate_dir().join("roms/Dr. Mario (World).gb");
    if !rom.exists() {
        eprintln!("skipping: {} not found", rom.display());
        return;
    }

    let gb = boot(&rom, 600);
    let hist = shade_histogram(gb.framebuffer());

    let distinct = hist.iter().filter(|&&c| c > 50).count();
    assert!(
        distinct >= 3,
        "title screen should use multiple shades, histogram {hist:?}"
    );

    let total = SCREEN_W * SCREEN_H;
    assert!(
        hist.iter().all(|&c| c < total - 50),
        "framebuffer is nearly uniform, histogram {hist:?}"
    );
}
