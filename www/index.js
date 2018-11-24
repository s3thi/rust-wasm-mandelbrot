import { Mandelbrot } from "rust-wasm-mandelbrot";
import { memory } from "rust-wasm-mandelbrot/rust_wasm_mandelbrot_bg";

const mandelbrot = Mandelbrot.new(1280, 1024);
mandelbrot.draw();
const pixelsPtr = mandelbrot.pixels();

const canvasEl = document.querySelector("canvas");
const ctx = canvasEl.getContext("2d");
const imageData = new ImageData(1280, 1024);

const pixelsArray = new Uint8Array(
    memory.buffer,
    pixelsPtr,
    mandelbrot.size()
);
imageData.data.set(pixelsArray);
ctx.putImageData(imageData, 0, 0);
