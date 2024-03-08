// TODO:
// - BUG FIXING - Food
// - FIGURE OUT STUPID BUG
// - Add food spawning in clusters
// - change home to middle 
// - figure out ant pheremones
// - add obstacles and pathfinding around
mod utils;
mod universe;
mod cell;
mod ant;
use wasm_bindgen::prelude::*;
pub use universe::Universe;
pub use cell::Cell;

use std::fmt;
extern crate js_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.


fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


    // Helper function to calculate the greatest common divisor (Euclidean algorithm)
    
    
        






