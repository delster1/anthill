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
    pub fn is_within_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
    pub fn get_cell(&mut self, row: u32, col: u32) -> Cell {
        let index = self.get_index(row, col);
        self.cells[index]
    }
    pub fn new_ant(&mut self) {
        

        let my_ant = Ant { pos: (self.width/2, self.height/2), status: AntState::Searching(0, 0), home : (self.width/2, self.height/2), food_ct: 0, energy: 1000, starting_energy: 1000, wander_chance: {js_sys::Math::random() * 100.0} as u32};
        self.ants.push(my_ant);
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.width {
            for col in 0..self.height {
                
                let idx = self.get_index(row, col);
                if next[idx].pheromone_level > 0.0 {

                    next[idx].pheromone_level -= 0.05;
                }
                if next[idx].pheromone_level < 0.5 && (next[idx].cell_type == CellType::Trail)  {
                    next[idx].cell_type = CellType::Empty;
                    next[idx].pheromone_level = 0.0;

                }
            }

            
        }
        let mut cloned_self = self.clone();
        let mut next_ants : Vec<Ant> = vec![];
        for ant in &mut self.ants {
            
            let possible_moves: [(i32, i32); 8] = [(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),((1,-1))];
            let perimeter_cells: Vec<(u32, u32, Cell)> = possible_moves.iter()
                .filter_map(|(dx, dy)| {
                    let new_x = (ant.pos.0 as i32 + dx) as u32;
                    let new_y = (ant.pos.1 as i32 + dy) as u32;
                    if cloned_self.is_within_bounds(new_x, new_y) {
                        let idx = cloned_self.get_index(new_x, new_y);
                        Some((new_x, new_y, cloned_self.cells[idx].clone()))
                    } else {
                        None
                    }
                })
            .collect();
            // Implement logic based on ant state
            // process cells around a given ant
            perimeter_cells.clone().into_iter().for_each(|(row, col, cell)| {
                let index = cloned_self.get_index(row, col);
                next[index] = ant.process_cell(cell, row, col);
            });
            match (ant.status, ant.food_ct) {
                (AntState::Searching(x , y), _) => {
                    // random explorations
                    ant.wander(&0, &0, &perimeter_cells);
                },

                (AntState::Returning(_x, _y), 0) => {}, // this is impossible, ants cannot return w/o food
                (AntState::Returning(x, y), 1..) => {
                     
                    let idx = ant.get_index();
                    next[idx] = Cell::build_pheremone_cell(ant.food_ct as f32 * 1.0);
                    perimeter_cells.clone().into_iter().for_each(|(row, col, cell)| {
                        let index = cloned_self.get_index(row, col);
                        next[index] = Cell::build_pheremone_cell(ant.food_ct as f32 * 1.0);
                    });
                    ant.return_home(x as i32, y as i32);
                    
                },
                (AntState::Wandering(_), 1..) => {
                    ant.random_wander();
                },
                (AntState::Wandering(_), 0) => {
                    panic!("This shouldn't happen");
                }
            }
            if ant.is_near_home() {
                // let mutation = ({js_sys::Math::random() as u32} * 10) - 20;
                let energy = ant.starting_energy ;
                // Ant has returned home, create a new ant
                (1..ant.food_ct).for_each(|_| {
                    
                    let new_ant = Ant {
                        pos: ant.home,
                        status: AntState::Searching(0,0),
                        home: ant.home,
                        food_ct: 0,
                        energy: energy,
                        starting_energy: energy,
                        wander_chance: {js_sys::Math::random() * 100.0} as u32,
                    };
                    next_ants.push(new_ant);
                });
                log!("made {:?} ants", ant.food_ct );
                
            }
            if ant.energy > 1 {
                next_ants.push(*ant);
            }
            else {
                perimeter_cells.clone().into_iter().for_each(|(row, col, cell)| {

                    let index = cloned_self.get_index(row, col);
                    next[index] = ant.die();
                });
                let index = cloned_self.get_index(ant.pos.0, ant.pos.1);
                next[index] = ant.die();
            }
            
        }
        self.ants = next_ants;
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
        let scale = 0.12;  // Adjust scale to zoom in or out of the noise pattern
        let threshold = 0.4;  // Adjust threshold to increase/decrease food density
    
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
    
        let num_ants = 100;
        let home_loc = (width / 2, height / 2);
        let mut ants = Vec::new();
        let energy = {js_sys::Math::random() * 300.0 }as u32;
        for _ in 0..num_ants {
            ants.push(Ant {
                pos: home_loc,
                status: AntState::Searching(0,0),
                home: home_loc,
                food_ct:  0,
                energy: energy,
                starting_energy : energy,
                wander_chance: {js_sys::Math::random() * 100.0  } as u32,
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
