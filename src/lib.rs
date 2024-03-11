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

extern crate js_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


    
    
        






