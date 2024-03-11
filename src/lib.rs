// TODO:
// - refactor code out of canvas to a better render thing
// - refactor code to run faster by changing how data is sent to wasm
// - Add food spawning in clusters
// - change home to middle 
    // - change ant movement functions
    // - change slope calculations to match
// - figure out ant pheremones
    // - do paths need to be a list of pts?
    // - how will ants travel along them to and from?
// - add obstacles and pathfinding around
// - add ants searching perimeter for food
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
    
    
        






