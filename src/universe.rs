use crate::ant::{Ant, AntState};
use std::fmt;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;
use crate::cell::Cell;
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height : u32,
    cells: Vec<Cell>,
    ants: Vec<Ant>
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol: char = match cell {
                    Cell::Empty => '◻',
                    Cell::Trail => '|',
                    Cell::Food => '🥞',
                    Cell::Home => '⌂',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {

    fn get_index(&mut self, row : u32, col : u32) -> usize {
        return (row * self.width + col) as usize;
    }
    pub fn new_ant(&mut self) {
        let dest_x ={ js_sys::Math::random() * (159 as f64)} as u32;
        let dest_y = {js_sys::Math::random() * (159 as f64)} as u32;

        let my_ant = Ant { position: (0, 0), status: AntState::Searching(dest_x, dest_y)};
        self.ants.push(my_ant);
        log!("Created new ant");
    }
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.width {
            for col in 0..self.height {

                
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                for ant in &mut self.ants {
                    if ant.position.0 == row && ant.position.1 == col && cell == Cell::Food{
                        log!("Ant found food at ({},{})", ant.position.1, ant.position.0);  
                        ant.status = AntState::Returning(row, col);
                        next[idx] = Cell::Empty;
                    }
                }

            }
        }
        for ant in &mut self.ants {
            // Implement logic based on ant state
            match ant.status {
                AntState::Searching(x , y) => {
                    ant.goto(x, y);
                    // ant goes to it's implicitly defined path
                },

                AntState::Returning(x, y) => {

                    let idx = ant.get_index();
                    next[idx] = Cell::Trail;
                    ant.return_home(x, y);
                    
                    // Logic for returning home
                }
                // Handle other states as needed
            }
        }
        self.cells = next;
    }
    

    
    // This method returns a pointer to the flat array of ant positions
    pub fn ants_positions_flat(&self) -> *const u32 {
        let mut flat_positions: Vec<u32> = Vec::with_capacity(self.ants.len() * 2);
        for ant in &self.ants {
            flat_positions.push(ant.position.0);
            flat_positions.push(ant.position.1);
        }

        // IMPORTANT: Ensure this memory is not freed while being used in JS
        // You might need to manage this memory's lifecycle carefully
        flat_positions.as_ptr()
    }

    // You might also need a method to get the number of ants to correctly iterate over the array in JS
    pub fn ants_count(&self) -> u32 {
        self.ants.len() as u32
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 160;
        let height = 160;

        let cells = (0..width * height)
            .map(|i| {
                if i % 4096 == 0 {
                    Cell::Home
                } else if js_sys::Math::random() < 0.01{
                   Cell::Food
                }
                else {

                    Cell::Empty
                }
            }).collect();
        let num_ants = 3;
        let mut ants = Vec::new();

        for i in 0..num_ants {
            let loc_x = (js_sys::Math::random() * 159.0) as u32;
            let loc_y = (js_sys::Math::random() * 159.0) as u32;
            ants.push(Ant {
                position: (0, 0), // Set initial position if needed, or use loc_x, loc_y
                status: AntState::Searching(loc_x, loc_y),
            });
        }

        Universe {
            width, 
            height, 
            cells,
            ants,
        }
    }
    
}