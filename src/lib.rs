mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use ru_gameboy::vm::{WIDTH, HEIGHT, Vm};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-gameboy!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
    Black = 0,
    LGrey = 1,
    DGrey = 2,
    White = 3
}

#[wasm_bindgen]
pub struct Gameboy {
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Gameboy {
    pub fn new() -> Self {
        let pixels = (0..WIDTH*HEIGHT)
            .map(|i| {
                match i % 4 {
                    0 => Pixel::Black,
                    1 => Pixel::DGrey,
                    2 => Pixel::LGrey,
                    3 => Pixel::White,
                    _ => unreachable!(),
                }
            })
            .collect();

        Self {
            pixels,
        }
    }
    pub fn width(&self) -> u32 {
        WIDTH as u32
    }

    pub fn height(&self) -> u32 {
        HEIGHT as u32
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Gameboy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(WIDTH as usize) {
            for &pixel in line {
                let symbol = match pixel {
                    Pixel::Black => '◼',
                    Pixel::DGrey => '◼',
                    Pixel::LGrey => '◻',
                    Pixel::White => '◻',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
