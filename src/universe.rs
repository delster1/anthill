use crate::ant::{Ant, AntState};
use std::fmt;
use wasm_bindgen::prelude::*;
use crate::log;
use crate::utils;
use crate::cell::{Cell, CellType};
use noise::{NoiseFn, Perlin};
#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: u32,
    height : u32,
    cells: Vec<Cell>,
    simple_cells: Vec<u8>,
    ants: Vec<Ant>
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol: char = match cell.cell_type {
                    CellType::Empty => '◻',
                    CellType::Trail => '|',
                    CellType::Searched => '0',
                    CellType::Food => '🥞',
                    CellType::Home => '⌂',
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
        let dest_x ={ js_sys::Math::random() * 80.0} as u32;
        let dest_y = {js_sys::Math::random() * 80.0} as u32;

        let my_ant = Ant { pos: (self.width/2, self.height/2), status: AntState::Searching(dest_x, dest_y), home : (self.width/2, self.height/2), food_ct: 0};
        self.ants.push(my_ant);
        log!("Created new ant going to {dest_x}, {dest_y}");
    }
    fn is_cell_trail(&mut self, row: u32, col : u32)-> bool {
        let idx = self.get_index(row, col);

        self.cells[idx].cell_type == CellType::Trail
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.width {
            for col in 0..self.height {
                
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let current_cell_type = cell.cell_type;
                for ant in &mut self.ants {
                    // FOR EACH ANT, update universe and antstate depending on cell

                    if ant.pos.0 == row && ant.pos.1 == col && current_cell_type == CellType::Food{

                        next[idx] = ant.process_cell(cell, &row, &col);
                    }
                    if ant.pos.0 + 1 == row && ant.pos.1 == col && current_cell_type == CellType::Food{

                        next[idx] = ant.process_cell(cell, &row, &col);
                    }
                    if ant.pos.0 - 1 == row && ant.pos.1 == col && current_cell_type == CellType::Food{

                        next[idx] = ant.process_cell(cell, &row, &col);
                    }
                    if ant.pos.0 == row && ant.pos.1 + 1 == col && current_cell_type == CellType::Food{

                        next[idx] = ant.process_cell(cell, &row, &col);
                    }
                    if ant.pos.0 == row && ant.pos.1 - 1 == col && current_cell_type == CellType::Food{

                        next[idx] = ant.process_cell(cell, &row, &col);
                    } 

                }
                next[idx].pheromone_level -= 0.01;
                if next[idx].pheromone_level < 0.5 && next[idx].cell_type == CellType::Trail {
                    next[idx].cell_type = CellType::Empty;
                    next[idx].pheromone_level = 0.0;

                }
            }

            
        }
        let mut cloned_self = self.clone();

        for ant in &mut self.ants {
            let cells_copy = self.cells.clone();
            let possible_moves: [(i32, i32); 8] = [(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),((1,-1))];
            let perimeter_cells : Vec<(u32, u32, Cell)> = possible_moves.iter().map(|(x,y)| {
                let x_pos = (ant.pos.0 as i32 + x) as u32 ;
                let y_pos = (ant.pos.1 as i32 + y) as u32 ;

                let x_pos = x_pos as u32;
                let y_pos = y_pos as u32;
                let idx = cloned_self.get_index((x_pos as i32 + x) as u32, (y_pos as i32 + y) as u32);
                (x_pos, y_pos as u32, cells_copy[idx])
            }).collect();
            // Implement logic based on ant state
            match (ant.status, ant.food_ct) {
                (AntState::Searching(x , y), _) => {
                    
                    ant.wander(&x, &y, &perimeter_cells);
                    // ant goes to it's implicitly defined path
                },

                (AntState::Returning(x, y), 0) => {

                    let idx = ant.get_index();
                    next[idx] = Cell::build_searched_cell();
                    log!("RETURNING WITHOUT FOOD");
                    ant.return_home(x, y);
                    
                    // Logic for returning home
                },
                (AntState::Returning(x, y), 1..) => {

                    let idx = ant.get_index();
                    next[idx] = Cell::build_pheremone_cell(ant.food_ct as f64 * 1.0);
                    ant.return_home(x, y);
                    
                    // Logic for returning home
                },
                (AntState::Wandering(x, y),_) => {
                    let idx = ant.get_index();
                    next[idx] = Cell::build_pheremone_cell(1.0);
                    ant.wander(&x,&y, &perimeter_cells);
                },
                (_,_) => {}

                // Handle other states as needed
            }
        }
        self.cells = next;
        self.simple_cells = self.cells.iter().map(|cell| cell.cell_type as u8).collect();

        
    }
    

    
    // This method returns a pointer to the flat array of ant positions
    pub fn ants_positions_flat(&self) -> *const u32 {
        let mut flat_positions: Vec<u32> = Vec::with_capacity(self.ants.len() * 2);
        for ant in &self.ants {
            flat_positions.push(ant.pos.0);
            flat_positions.push(ant.pos.1);
        }

        // IMPORTANT: Ensure this memory is not freed while being used in JS
        // You might need to manage this memory's lifecycle carefully
        flat_positions.as_ptr()
    }


    // fn make_food_clusters(&mut self, cells: Vec<Cell>) -> Vec<Cell> {
        
        
    //     let new_cells = cells.iter().map(|i| {
    //         js_sys::Math::random();
    //     }).collect();
    //     new_cells
    // }
    // You might also need a method to get the number of ants to correctly iterate over the array in JS
    pub fn ants_count(&mut self) -> u32 {
        self.ants.len() as u32
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.simple_cells.as_ptr()
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn new() -> Universe {
        utils::set_panic_hook();
    
        let perlin = Perlin::new((js_sys::Math::random() * 100.0) as u32); // Corrected constructor call (remove the '1')
        let scale = 0.09;  // Adjust scale to zoom in or out of the noise pattern
        let threshold = 0.5;  // Adjust threshold to increase/decrease food density
    
        let width = 160;
        let height = 160;
    
        let cells: Vec<Cell> = (0..width * height).map(|i| {
            let x = (i % width) as f64 * scale;
            let y = (i / width) as f64 * scale;
            let noise_value = perlin.get([x, y, 0.0]);  // Use 3D noise for future flexibility
            
            let cell_type = if noise_value > threshold {
                CellType::Food
            } else if i % 12800 == 0 {  // Keep your home cells as originally designed
                CellType::Home
            } else {
                CellType::Empty
            };
            Cell {
                cell_type,
                pheromone_level: 0.0,
            }
        }).collect();
    
        let num_ants = 1;
        let home_loc = (width / 2, height / 2);
        let mut ants = Vec::new();
        for _ in 0..num_ants {
            ants.push(Ant {
                pos: home_loc,
                status: AntState::Searching(js_sys::Math::random() as u32 * width, js_sys::Math::random() as u32 * height),
                home: home_loc,
                food_ct:  0,
            });
        }
        let simple_cells = cells.iter().map(|cell| cell.cell_type as u8).collect();

        Universe {
            width, 
            height, 
            cells,
            simple_cells,
            ants
        }
    }
    
}
