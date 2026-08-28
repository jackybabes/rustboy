//! Runs blargg's test ROMs through the emulator.
//!
//! Each ROM is its own `#[test]`, so `cargo test --test blargg` runs them one
//! after another and reports pass/fail per ROM. ROMs are vendored under
//! `roms/gb-test-roms/` (git submodule).
//!
//! Run just these:      cargo test --test blargg
//! See serial output:   cargo test --test blargg -- --nocapture
//! One at a time:       cargo test --test blargg -- --test-threads=1

use std::path::PathBuf;

use rustboy::{run_test_rom, TestOutcome};

fn roms_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("roms/gb-test-roms")
}

fn run(rel_path: &str) {
    let path = roms_root().join(rel_path);
    assert!(
        path.exists(),
        "test ROM not found: {}\n(is the gb-test-roms submodule checked out?)",
        path.display()
    );

    let run = run_test_rom(&path, |_| {}).expect("failed to read/run ROM");

    match run.outcome {
        TestOutcome::Passed => {}
        TestOutcome::Failed => panic!("{rel_path} reported failure:\n{}", run.serial.trim()),
        TestOutcome::Stuck { pc } => panic!(
            "{rel_path} wedged at PC {pc:#06X} without a verdict\nserial so far:\n{}",
            run.serial.trim()
        ),
    }
}

macro_rules! rom_tests {
    ($($name:ident => $file:literal,)*) => {
        $(
            #[test]
            fn $name() {
                run($file);
            }
        )*
    };
}

// blargg cpu_instrs — individual sub-tests (32 KiB, no MBC).
rom_tests! {
    cpu_01_special            => "cpu_instrs/individual/01-special.gb",
    cpu_02_interrupts         => "cpu_instrs/individual/02-interrupts.gb",
    cpu_03_op_sp_hl           => "cpu_instrs/individual/03-op sp,hl.gb",
    cpu_04_op_r_imm           => "cpu_instrs/individual/04-op r,imm.gb",
    cpu_05_op_rp              => "cpu_instrs/individual/05-op rp.gb",
    cpu_06_ld_r_r             => "cpu_instrs/individual/06-ld r,r.gb",
    cpu_07_jr_jp_call_ret_rst => "cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
    cpu_08_misc_instrs        => "cpu_instrs/individual/08-misc instrs.gb",
    cpu_09_op_r_r             => "cpu_instrs/individual/09-op r,r.gb",
    cpu_10_bit_ops            => "cpu_instrs/individual/10-bit ops.gb",
    cpu_11_op_a_hl            => "cpu_instrs/individual/11-op a,(hl).gb",
}

// blargg cpu_instrs — combined ROM (64 KiB, MBC1: exercises bank switching).
rom_tests! {
    cpu_instrs_combined => "cpu_instrs/cpu_instrs.gb",
}
