// TODO:
// - refactor code out of canvas to a better render thing
// - refactor code to run faster by changing how data is sent to wasm
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


    
    
        






