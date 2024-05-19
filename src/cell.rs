use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone,Copy, Debug, PartialEq)]
pub struct Cell {
    pub cell_type: CellType,
    pub pheromone_level: f32,  // This field is only used when cell_type is Trail
}
#[wasm_bindgen]

#[repr(u8)]
#[derive(Clone,Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Empty = 0,
    Trail = 1,
    Searched = 2,
    Food = 3,
    Home = 4,
}
#[wasm_bindgen]

impl Cell {
    pub fn to_u8(&self) -> u8 {
        self.cell_type as u8
    }
    pub fn get_pheromone_level(&self) -> f32 {
        self.pheromone_level
    }
    pub fn build_pheremone_cell(new_pheremone_level : f32) -> Cell{
        
        Cell {
            cell_type : CellType::Trail,
            pheromone_level : new_pheremone_level,
        }
    }
    pub fn build_empty_cell() -> Cell {
        Cell {
            cell_type : CellType::Empty,
            pheromone_level : 0.0,
        }
    }
    pub fn build_searched_cell() -> Cell {
        Cell { 
            cell_type : CellType::Searched,
            pheromone_level : -1.0,
        }
    }
    pub fn build_food_cell() -> Cell {
        Cell {
            cell_type : CellType::Food,
            pheromone_level : 0.0,
        }
    }
}
