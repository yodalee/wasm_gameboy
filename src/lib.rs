mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::ptr;
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
    DGrey = 1,
    LGrey = 2,
    White = 3
}

#[wasm_bindgen]
pub struct Gameboy {
    pixels: Vec<Pixel>,
    vm: Option<Vm>,
    cartridge: Vec<u8>,
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

        let cartridge = vec![0;0x8000];

        Self {
            pixels,
            vm: None,
            cartridge,
        }
    }
    pub fn width(&self) -> u32 {
        WIDTH as u32
    }

    pub fn height(&self) -> u32 {
        HEIGHT as u32
    }

    pub fn get_buffer(&self) -> *const u32 {
        match self.vm {
            Some(ref vm) => vm.buffer.as_ptr(),
            None => ptr::null(),
        }
    }

    pub fn get_cartridge(&self) -> *const u8 {
        self.cartridge.as_ptr()
    }

    pub fn tick(&mut self) {
        if let Some(ref mut vm) = self.vm {
            vm.run();
        }
    }

    pub fn dump(&self) -> String {
        match self.vm {
            Some(ref vm) => vm.cpu.dump(),
            None => "".to_string(),
        }
    }

    pub fn set_cartridge(&mut self) {
        self.vm = Some(Vm::new(self.cartridge.clone()));
    }
}
