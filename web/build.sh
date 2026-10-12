#!/usr/bin/env bash
# Build the browser version into web/dist/ (what GitHub Pages serves).
#
#   ./web/build.sh
#   python3 -m http.server -d web/dist 8080   # then open http://localhost:8080
#
# Needs: rustup target add wasm32-unknown-unknown && cargo install wasm-pack
set -euo pipefail
cd "$(dirname "$0")"

wasm-pack build --target web --release --out-dir pkg --no-pack

rm -rf dist
mkdir -p dist/pkg dist/roms
cp index.html main.js style.css dist/
cp pkg/rustboy_web.js pkg/rustboy_web_bg.wasm dist/pkg/
cp ../test-roms/blargg/cpu_instrs/cpu_instrs.gb dist/roms/

echo "built web/dist/"
