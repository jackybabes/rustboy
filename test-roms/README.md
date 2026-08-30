# Vendored test ROMs

These ROMs are committed to the repo (unlike `roms/`, which is git-ignored) so
that `cargo test` works on a fresh clone with no extra setup.

## blargg/

Shay Green's ("blargg") Game Boy hardware test ROMs, which he released freely
for emulator authors. Only the `cpu_instrs` set is vendored here — it's what the
test suite in `tests/` exercises.

- `cpu_instrs/individual/*.gb` — the 11 individual sub-tests (32 KiB each, no MBC)
- `cpu_instrs/cpu_instrs.gb` — the combined ROM (64 KiB, MBC1; exercises bank switching)
- `cpu_instrs/readme.txt` — blargg's original notes

Each ROM prints progress and a final `Passed` / `Failed` over the serial port;
`rustboy::run_test_rom` reads that to decide the outcome.

Upstream: https://github.com/retrio/gb-test-roms (mirror of blargg's originals).

Commercial game ROMs are **not** committed here — tests that need one (e.g. the
Dr. Mario PPU smoke test) look under `roms/` and skip themselves if it's absent.
