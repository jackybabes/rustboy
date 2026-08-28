//! Runs the blargg `cpu_instrs` individual test ROMs through the emulator.
//!
//! Each ROM is its own `#[test]`, so `cargo test --test blargg` runs them one
//! after another and reports pass/fail per ROM. ROMs live under
//! `roms/gb-test-roms/cpu_instrs/individual/` (git submodule / vendored).
//!
//! Run just these:      cargo test --test blargg
//! See serial output:   cargo test --test blargg -- --nocapture

use std::path::PathBuf;

use rustboy::{run_test_rom, TestOutcome};

fn rom_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("roms/gb-test-roms/cpu_instrs/individual")
}

fn run(rom_file: &str) {
    let path = rom_dir().join(rom_file);
    assert!(
        path.exists(),
        "test ROM not found: {}\n(are the gb-test-roms vendored / submodule checked out?)",
        path.display()
    );

    let run = run_test_rom(&path, |_| {}).expect("failed to read/run ROM");

    match run.outcome {
        TestOutcome::Passed => {}
        TestOutcome::Failed => panic!("{rom_file} reported failure:\n{}", run.serial.trim()),
        TestOutcome::Stuck { pc } => panic!(
            "{rom_file} wedged at PC {pc:#06X} without a verdict\nserial so far:\n{}",
            run.serial.trim()
        ),
    }
}

macro_rules! blargg_rom_tests {
    ($($name:ident => $file:literal,)*) => {
        $(
            #[test]
            fn $name() {
                run($file);
            }
        )*
    };
}

blargg_rom_tests! {
    cpu_01_special           => "01-special.gb",
    cpu_02_interrupts        => "02-interrupts.gb",
    cpu_03_op_sp_hl          => "03-op sp,hl.gb",
    cpu_04_op_r_imm          => "04-op r,imm.gb",
    cpu_05_op_rp             => "05-op rp.gb",
    cpu_06_ld_r_r            => "06-ld r,r.gb",
    cpu_07_jr_jp_call_ret_rst => "07-jr,jp,call,ret,rst.gb",
    cpu_08_misc_instrs       => "08-misc instrs.gb",
    cpu_09_op_r_r            => "09-op r,r.gb",
    cpu_10_bit_ops           => "10-bit ops.gb",
    cpu_11_op_a_hl           => "11-op a,(hl).gb",
}
