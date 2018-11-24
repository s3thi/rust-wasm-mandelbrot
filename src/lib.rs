extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

const MAGNIFICATION: u32 = 200;
const PAN_X: f64 = 2.0;
const PAN_Y: f64 = 1.5;

#[wasm_bindgen]
pub struct Mandelbrot {
    width: u32,
    height: u32,
    pixels: Vec<u8>
}

#[wasm_bindgen]
impl Mandelbrot {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels_size = width * height * 4;

        Mandelbrot {
            width,
            height,
            pixels: vec![0; pixels_size as usize]
        }
    }
    
    pub fn draw(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let is_outside = is_outside(
                    (x as f64/MAGNIFICATION as f64) - PAN_X,
                    (y as f64/MAGNIFICATION as f64) - PAN_Y
                );

                let mut pixel_index = self.get_pixel_index(x, y);

                self.pixels[pixel_index as usize] = is_outside.unwrap_or(255);

                self.pixels[(pixel_index + 1) as usize] = 61;
                self.pixels[(pixel_index + 2) as usize] = 127;

                let alpha = 255 - is_outside.unwrap_or(0) * 2;
                self.pixels[(pixel_index + 3) as usize] = alpha;
            }
        }
    }

    pub fn size(&self) -> u32 {
        self.width * self.height * 4
    }

    pub fn get_pixel_index(&self, x: u32, y: u32) -> u32 {
        (y * self.width * 4) + (x * 4)
    }

    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}

fn is_outside(x: f64, y: f64) -> Option<u8> {
    let mut real = x;
    let mut imaginary = y;

    for i in 0..100 {
        let temp_real = real * real - imaginary * imaginary + x;

        let temp_imaginary = 2.0 * real * imaginary + y;

        real = temp_real;
        imaginary = temp_imaginary;

        if real * imaginary > 5.0 {
            return Some(i);
        }
    }

    None
}
