mod utils;
use wasm_bindgen::prelude::*;
use std::fmt;
use std::clone::Clone;
extern crate js_sys;
const UNIV_SIZE : u32  = 128;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
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
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[derive(Clone, Copy)]
pub struct Ant{
    position : (u32, u32), 
    status: AntState,
}
impl Ant{
    fn is_position_within_bounds(&self, x: u32, y: u32) -> bool {
        x < UNIV_SIZE && y < UNIV_SIZE
    }
    fn update_position(&mut self, new_x: u32, new_y: u32) {
        // Get current ant position

        // Calculate new position, ensuring it's within bounds
        let mut new_x = new_x as u32;
        let mut new_y = new_y as u32;

        // Ensure the new position is within the universe bounds
        if !self.is_position_within_bounds(new_x, new_y) {
   

            // Or wrap around (for a toroidal universe):
            new_x = (new_x + UNIV_SIZE) % UNIV_SIZE;
            new_y = (new_y + UNIV_SIZE) % UNIV_SIZE;
        }

        // Update the ant's position
        self.position = (new_x, new_y);    
    }
    fn get_index (&mut self) -> usize{
        return (self.position.0 * UNIV_SIZE + self.position.1) as usize;
    }

    fn return_home(&mut self, x: u32, y:u32) {
        // Change the ant's status to Searching if it is already at the origin
        if self.position.0 < 5 && self.position.1 < 5 {
            self.status = AntState::Searching;
            self.position.0 = {js_sys::Math::random().round() as u32} * UNIV_SIZE;
            self.position.1 = {js_sys::Math::random().round() as u32} * UNIV_SIZE;

        } 
        
    
        if self.status != AntState::Searching {

            if self.position.0 <= 1 && self.position.1 > 1 {
                self.update_position(self.position.0 -1, self.position.1);
                return;
            } else if self.position.1 <= 1 && self.position.0 > 1 {
                self.update_position(self.position.0 , self.position.1 -1);
                return;
            }
            
            let slope : f32 = x as f32 / y as f32;

            let mut delta_x = slope - {self.position.0 - 1} as f32 / y as f32;
            let mut delta_y = slope - self.position.0 as f32 / {self.position.1 - 1} as f32 ;

            delta_x = delta_x.abs();
            delta_y = delta_y.abs();

            if delta_x < delta_y {
                self.update_position(self.position.0 -1, self.position.1);
            } else if delta_x > delta_y {
                self.update_position(self.position.0, self.position.1-1);
            } else {
                self.update_position(self.position.0 -1, self.position.1-1);

            }

        }
    }
    
    // Helper function to calculate the greatest common divisor (Euclidean algorithm)
    
    
        
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AntState {
    Searching,
    Returning(u32, u32),
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell{
    Empty = 0,
    Trail = 1,
    Food = 2,
    Home = 3
}
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height : u32,
    cells: Vec<Cell>,
    ants: Vec<Ant>
}
#[wasm_bindgen]
impl Universe {

    fn get_index(&mut self, row : u32, col : u32) -> usize {
        return (row * self.width + col) as usize;
    }
    pub fn new_ant(&mut self, row : u32, col : u32) {
        let my_ant = Ant { position: (row, col), status: AntState::Searching};
        self.ants.push(my_ant);
    }
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.width {
            for col in 0..self.height {

                
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                for ant in &mut self.ants {
                    if ant.position.0 == row && ant.position.1 == col && cell == Cell::Food{
                        log!("Ant found food at ({},{})", ant.position.0, ant.position.1);  
                        ant.status = AntState::Returning(ant.position.0,  ant.position.1);
                        next[idx] = Cell::Empty;
                    }
                }
                // ??? TBD: What needs to happen every tick???
            }
        }
        for ant in &mut self.ants {
            // Implement logic based on ant state
            match (ant.status, js_sys::Math::random()) {
                (AntState::Searching, 0.0..=0.25) => {
                    ant.update_position(ant.position.0+1, ant.position.1);
                    // Example movement logic: move randomly
                    // Note: Implement actual logic for moving towards food or exploring
                },
                (AntState::Searching, 0.26..=0.5) => {
                    ant.update_position(ant.position.0-1, ant.position.1);

                },
                (AntState::Searching, 0.51..=0.75) => {
                    ant.update_position(ant.position.0, ant.position.1+1);

                    // Example movement logic: move randomly
                    // Note: Implement actual logic for moving towards food or exploring
                },
                (AntState::Searching, 0.76..=1.0) => {
                    ant.update_position(ant.position.0, ant.position.1-1);
                }
                
                (AntState::Searching, _) => {},
                (AntState::Returning(x, y), _) => {

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

        let width = UNIV_SIZE;
        let height = UNIV_SIZE;

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
        let ants = vec![
            Ant { position: (0, 0), status: AntState::Searching }; num_ants // Adjust as needed
            // Add more ants as needed
        ];
        Universe {
            width, 
            height, 
            cells,
            ants,
        }
    }
    
}

