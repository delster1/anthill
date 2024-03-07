use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell{
    Empty = 0,
    Trail = 1,
    Food = 2,
    Home = 3
}