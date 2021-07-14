import { Universe, Gameboy } from "wasm-gameboy";
import { memory } from "wasm-gameboy/wasm_gameboy_bg";

const PIXEL_SIZE = 5;
const COLOR_BLACK = "#000000";
const COLOR_DGREY = "#555555";
const COLOR_LGREY = "#AAAAAA";
const COLOR_WHITE = "#FFFFFF";

const gameboy = Gameboy.new();
const width = gameboy.width();
const height = gameboy.height();

const pre = document.getElementById("gameboy-log");
const canvas = document.getElementById("gameboy-canvas");
canvas.height = PIXEL_SIZE * height;
canvas.width  = PIXEL_SIZE * width;

const fileUploader = document.getElementById("file-uploader");

fileUploader.addEventListener('change', (e) => {
  var reader = new FileReader();

  reader.onload = function() {
    const cartridge = gameboy.get_cartridge();
    var bytes = new Uint8Array(reader.result);

    const len = 0x8000;
    const m_cartridge = new Uint8Array(memory.buffer, cartridge, len);

    // set cartridge
    for (let idx = 0; idx < len; idx++) {
      m_cartridge[idx] = bytes[idx];
    }

    gameboy.set_cartridge();

    setInterval(() => {
      requestAnimationFrame(renderLoop);
    }, 17)
  }

  reader.readAsArrayBuffer(e.target.files[0]);
});

const ctx = canvas.getContext('2d');
var cnt = 0;

const renderLoop = () => {
  drawPixels();
  gameboy.tick();
  pre.textContent = gameboy.dump();
}

const getIndex = (row, column) => {
  return row * width + column;
};

const drawPixels = () => {
  //const ptr = gameboy.get_pixels();
  //const pixels = new Uint8Array(memory.buffer, ptr, width * height);

  const buffer = gameboy.get_buffer();
  const pixels = new Uint32Array(memory.buffer, buffer, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      if (pixels[idx] == 0x00000000) {
        ctx.fillStyle = COLOR_BLACK;
      } else if (pixels[idx] == 0x00555555) {
        ctx.fillStyle = COLOR_DGREY;
      } else if (pixels[idx] == 0x00AAAAAA) {
        ctx.fillStyle = COLOR_LGREY;
      } else {
        ctx.fillStyle = COLOR_WHITE;
      }

      ctx.fillRect(
        col * PIXEL_SIZE,
        row * PIXEL_SIZE,
        PIXEL_SIZE,
        PIXEL_SIZE
      );
    }
  }

  ctx.stroke();
};
