import init, { Emulator } from "./pkg/rustboy_web.js";

const W = 160;
const H = 144;
const FRAME_MS = 1000 / 59.7275; // real DMG refresh rate
const DEMO_ROM = "./roms/cpu_instrs.gb";

const KEYMAP = {
  ArrowRight: "right",
  ArrowLeft: "left",
  ArrowUp: "up",
  ArrowDown: "down",
  KeyZ: "a",
  KeyX: "b",
  Enter: "start",
  Backspace: "select",
  ShiftRight: "select",
};

const canvas = document.getElementById("screen");
const ctx = canvas.getContext("2d");
const image = ctx.createImageData(W, H);
const statusEl = document.getElementById("status");
const fileInput = document.getElementById("rom-file");
const pauseBtn = document.getElementById("pause");
const dropHint = document.getElementById("drop-hint");

let emu = null;
let rom = null; // { bytes, name } — kept for Reset
let paused = false;
let crashed = null;
let last = 0;
let acc = 0;
let framesThisSecond = 0;
let fpsStamp = 0;
let fps = 0;

// A press and release can both land between two emulated frames (a fast tap),
// in which case the game would never see it. Releases of buttons pressed since
// the last frame are deferred until a frame has run.
const pressedSinceFrame = new Set();
const deferredRelease = new Set();

function setButton(button, down) {
  if (!emu) return;
  if (down) {
    deferredRelease.delete(button);
    pressedSinceFrame.add(button);
    emu.set_button(button, true);
  } else if (pressedSinceFrame.has(button)) {
    deferredRelease.add(button);
  } else {
    emu.set_button(button, false);
  }
  highlight(button, down);
}

function afterFrame() {
  for (const button of deferredRelease) emu.set_button(button, false);
  deferredRelease.clear();
  pressedSinceFrame.clear();
}

function boot(bytes, name) {
  try {
    emu?.free();
  } catch {
    // a previous instance that panicked can't be freed cleanly; drop it
  }
  rom = { bytes, name };
  emu = new Emulator(bytes);
  pressedSinceFrame.clear();
  deferredRelease.clear();
  crashed = null;
  paused = false;
  acc = 0;
  pauseBtn.textContent = "Pause";
  renderStatus();
}

function draw() {
  image.data.set(emu.frame_rgba());
  ctx.putImageData(image, 0, 0);
}

function renderStatus() {
  statusEl.classList.toggle("error", !!crashed);
  if (crashed) {
    statusEl.textContent = `Stopped: ${crashed}. Try another ROM.`;
    return;
  }
  if (!emu) {
    statusEl.textContent = "Load a ROM to start.";
    return;
  }
  const title = emu.title() || rom.name;
  const state = paused ? "paused" : `${fps.toFixed(1)} fps`;
  statusEl.innerHTML = "";
  const strong = document.createElement("strong");
  strong.textContent = title;
  statusEl.append(strong, ` — ${state}`);
}

function tick(now) {
  if (emu && !paused && !crashed) {
    acc += now - last;
    // Coming back from a background tab: don't try to catch up.
    if (acc > 250) acc = FRAME_MS;

    let ran = 0;
    try {
      while (acc >= FRAME_MS) {
        emu.run_frame();
        afterFrame();
        acc -= FRAME_MS;
        ran++;
      }
      if (ran) draw();
    } catch (err) {
      crashed = "the emulator hit an error (illegal opcode or unsupported cartridge?)";
      console.error(err);
      renderStatus();
    }
    framesThisSecond += ran;
  }
  last = now;

  if (now - fpsStamp >= 1000) {
    fps = (framesThisSecond * 1000) / (now - fpsStamp);
    framesThisSecond = 0;
    fpsStamp = now;
    renderStatus();
  }
  requestAnimationFrame(tick);
}

// ---- input: keyboard ----

window.addEventListener("keydown", (e) => {
  const button = KEYMAP[e.code];
  if (!button || !emu) return;
  e.preventDefault(); // keep arrows from scrolling, Enter from clicking buttons
  if (!e.repeat) setButton(button, true);
});

window.addEventListener("keyup", (e) => {
  const button = KEYMAP[e.code];
  if (!button || !emu) return;
  e.preventDefault();
  setButton(button, false);
});

window.addEventListener("blur", releaseAll);

function releaseAll() {
  if (!emu) return;
  for (const button of new Set(Object.values(KEYMAP))) {
    emu.set_button(button, false);
    highlight(button, false);
  }
  pressedSinceFrame.clear();
  deferredRelease.clear();
}

function highlight(button, on) {
  for (const el of document.querySelectorAll(`[data-btn="${button}"]`)) {
    el.classList.toggle("held", on);
  }
}

// ---- input: on-screen buttons (mouse + touch) ----

for (const el of document.querySelectorAll("[data-btn]")) {
  const button = el.dataset.btn;
  const press = (down) => (e) => {
    e.preventDefault();
    setButton(button, down);
  };
  el.addEventListener("pointerdown", (e) => {
    el.setPointerCapture(e.pointerId);
    press(true)(e);
  });
  el.addEventListener("pointerup", press(false));
  el.addEventListener("pointercancel", press(false));
  el.addEventListener("lostpointercapture", press(false));
  el.addEventListener("contextmenu", (e) => e.preventDefault());
}

// ---- loading ROMs ----

async function loadFile(file) {
  const bytes = new Uint8Array(await file.arrayBuffer());
  boot(bytes, file.name);
}

fileInput.addEventListener("change", async () => {
  const file = fileInput.files[0];
  if (file) await loadFile(file);
  fileInput.value = "";
  fileInput.closest("label").blur();
});

async function loadDemo() {
  statusEl.textContent = "Loading demo ROM…";
  const res = await fetch(DEMO_ROM);
  if (!res.ok) throw new Error(`demo ROM: HTTP ${res.status}`);
  boot(new Uint8Array(await res.arrayBuffer()), "cpu_instrs.gb");
}

document.getElementById("demo").addEventListener("click", (e) => {
  e.currentTarget.blur();
  loadDemo().catch(showError);
});

document.getElementById("reset").addEventListener("click", (e) => {
  e.currentTarget.blur();
  if (rom) boot(rom.bytes, rom.name);
});

pauseBtn.addEventListener("click", (e) => {
  e.currentTarget.blur();
  if (!emu || crashed) return;
  paused = !paused;
  pauseBtn.textContent = paused ? "Resume" : "Pause";
  releaseAll();
  renderStatus();
});

// Drag and drop anywhere on the page.
let dragDepth = 0;
window.addEventListener("dragenter", (e) => {
  e.preventDefault();
  dragDepth++;
  dropHint.hidden = false;
});
window.addEventListener("dragleave", () => {
  if (--dragDepth <= 0) {
    dragDepth = 0;
    dropHint.hidden = true;
  }
});
window.addEventListener("dragover", (e) => e.preventDefault());
window.addEventListener("drop", (e) => {
  e.preventDefault();
  dragDepth = 0;
  dropHint.hidden = true;
  const file = e.dataTransfer?.files?.[0];
  if (file) loadFile(file).catch(showError);
});

function showError(err) {
  console.error(err);
  statusEl.classList.add("error");
  statusEl.textContent = String(err.message || err);
}

// ---- start ----

await init();
loadDemo().catch(showError);
requestAnimationFrame((t) => {
  last = fpsStamp = t;
  requestAnimationFrame(tick);
});
