// TODO: 
// - FIX SLope calculation errror when ant is at y=0
   

    use crate::cell::{Cell, CellType};

    const UNIV_SIZE : u32  = 160;

    use crate::log;
    #[derive(Clone, Copy)]
    pub struct Ant{
        pub pos : (u32, u32), 
        pub status: AntState,
        pub home: (u32, u32),
        pub food_ct: u32,
    }
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum AntState {
        Searching(u32, u32),
        Returning(u32, u32),
    }

    impl Ant{
        fn is_position_within_bounds(&self, x: u32, y: u32) -> bool {
            x < UNIV_SIZE && y < UNIV_SIZE
        }

        fn start_search(&mut self) {
            self.food_ct = 0;
            self.status = AntState::Searching(0, 0);
            let searched_cells: Vec<(u32, u32, Cell)> = vec![];

            self.pos = self.home;
            self.wander(&0,&0, &searched_cells);
        }
        pub fn process_cell(&mut self, cell_to_process : Cell, row:  u32, col : u32) -> Cell {

            let current_cell_type = cell_to_process.cell_type;
            match current_cell_type {
                CellType::Empty => {
                    Cell {
                        pheromone_level : 0.0,
                        cell_type : CellType::Empty,
                    }
                },
                CellType::Food => {
                    self.food_ct += 1;
                    self.found_food(&row, &col);
                    Cell {
                        pheromone_level : 3.0,
                        cell_type : CellType::Trail,
                    }
                },
                CellType::Trail => {
                    Cell {
                        pheromone_level : cell_to_process.pheromone_level + 1.0,
                        cell_type : CellType::Trail,
                    }
                },
                CellType::Searched => {
                    Cell {
                        pheromone_level : 0.0,
                        cell_type : CellType::Searched,
                    }
                },
                CellType::Home => {
                    Cell {
                        pheromone_level : 0.0,
                        cell_type : CellType::Home,
                    }
                }

            }
        }
        fn calculate_slope(&mut self) -> Option<f32> {
        // determines a static-ish slope depending on antstate that will be compared to determine ant's next move every tick
        // gives slope to a random food when searching, and home when home
            match self.status {
                AntState::Returning(x,y) => {
                    let slope = (self.home.1 as f32 - y as f32)  / (self.home.0 as f32 - x as f32);
                    if slope.abs() == std::f32::INFINITY  {
                        return Some((self.home.0 - self.pos.1) as f32 * 0.01)
                    }
                    Some(slope)
                },
                AntState::Searching(x, y) => {
                    let slope = (y as f32 - self.home.1 as f32) / (x as f32 - self.home.0 as f32);
                    Some(slope)
                },
                
                // Otherwise, calculate the slope and wrap it in Some 
            }

        }
    pub fn found_food(&mut self, row: &u32, col: &u32) {
        self.food_ct += 1;

        match self.status {
            AntState::Searching(_, _) => {
                // If the ant finds food while searching, it should start wandering.
                log!("Ant found food, returning home!");
                self.status = AntState::Returning(*row, *col);
                self.return_home(*row,* col);
            },
            AntState::Returning(_, _) => {
                // If the ant is already returning, ignore additional food.
                self.food_ct += 1;
            },
        }
    }

    pub fn wander(&mut self, row : &u32, col: &u32, perimeter_cells: &Vec<(u32, u32, Cell)>) {
        let homex = self.home.0;
        let homey = self.home.1;
        let mut searched_cells: Vec<(u32, u32)> = vec![];
        let new_move : (u32, u32);
        let current_distance = self.get_current_distance_to(&homex, &homey) as f32;
        for &(new_x, new_y, ref cell) in perimeter_cells {
            let test_distance = self.get_distance_from_to(&new_x, &new_y, &homex, &homey) as f32;

            match cell.cell_type {
                CellType::Trail  if test_distance > current_distance && cell.pheromone_level > 0.1 => {
                    self.update_position(new_x, new_y);
                    return;
                },
                CellType::Food => {
                    // self.status = AntState::Returning(*row, *col);
                    // self.food_ct += 1;
                    // self.found_food(&row, &col);
                }, 
                CellType::Searched => {
                    searched_cells.push((*row, *col));
                }, // Do nothing if the cell is already searched
                _ => {} // Ignore other cell types
            }
        }
        match js_sys::Math::random() {
            (0.0..=0.25) => {
                // Randomly move right
                new_move = (self.pos.0 + 1, self.pos.1);
            },
            (0.25..=0.5) => {
                // Randomly move left
                new_move = (self.pos.0 - 1, self.pos.1);
            },
            (0.5..=0.75) => {
                // Randomly move down
                new_move = (self.pos.0, self.pos.1 + 1);
            },
            (0.75..=1.0) => {
                // Randomly move up
                new_move = (self.pos.0, self.pos.1 - 1);
            },
            _ => {
                // Default case should theoretically never be hit since Math::random() returns [0, 1)
                new_move = (self.home.0, self.home.1);
            }
        }

        self.update_position(new_move.0,new_move.1);
    }
    pub fn calculate_current_slope(&mut self, test_position: (u32, u32)) -> Option<f32> {
        // determines a slope given a test posititon and implicit destination
            // this slope is used to compare with the static-ish slope to calculate the best move given 
        let curr_x = test_position.0;
        let curr_y = test_position.1;
        if test_position.0 == 0 {
            // If x is 0, slope is undefined (vertical line), return None to indicate error
            None
        } else {
            match self.status {
                AntState::Returning(_,_) => {
                    let slope = (self.home.0 as f32 - curr_y as f32)  / (self.home.1 as f32 - curr_x as f32);
                    Some(slope)
                },
                AntState::Searching(x, y) => {
                    let slope = (y as f32 - curr_y as f32) / (x as f32 - curr_x as f32);
                    Some(slope)
                },
            }
            // Otherwise, calculate the slope and wrap it in Some
        }
    }   

    fn get_distance_from_to(&mut self, y2 : &u32, x2 : &u32, y1 : &u32, x1 : &u32) -> u32{
        let current_distance_squared : u32 = (y2  - y1 ).pow(2) + (x2  - x1 ).pow(2);
        current_distance_squared
    }

    fn get_current_distance_to(&mut self, x: &u32, y: &u32) -> u32 {
        
        let current_distance_squared : u32 = (x  - self.pos.0 ).pow(2) + (y  - self.pos.1 ).pow(2);
        current_distance_squared
    }


    fn update_position(&mut self, new_x: u32, new_y: u32) {

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
        self.pos = (new_x, new_y);    
    }

    pub fn get_index (&mut self) -> usize{
        return (self.pos.0 * UNIV_SIZE + self.pos.1) as usize;
    }

    pub fn return_home(&mut self, x: u32, y:u32) {
        let pos_x = self.pos.0;
        let pos_y = self.pos.1;
        // Change the ant's status to Searching if it is already at the origin
        if {self.home.0 - 1} <= pos_x && pos_x <= {self.home.0 + 1} && {self.home.1 - 1} <= pos_y && pos_y <= {self.home.1 + 1} {
            self.start_search();
            return;
        }
        if self.status != AntState::Searching(x,y) {
            // actual code for returning home below:
            // if on an edge of the screen, keep walking on edge

            // still broken below, need to update calculate current slope for going home
            //  Should just need to call calculate current slope for 4 potential moves and return best move - build a function for this  
            let slope = match self.calculate_slope() {
                Some(slope) => {
                    // Slope calculation was successful, proceed with logic using the slope
                    slope
                },
                None => {
                    // Slope calculation failed (e.g., division by zero), ant teleport home for now..
                    log!("SLOPE CALCULATION FAILED");
                    self.pos = self.home;
                    self.update_position(self.home.0,self.home.1);
                    self.start_search();
                    return;
                }
            };
            // algorithm for finding best move to get home
            let potential_moves = [
                (pos_x + 1, pos_y), // Right
                (pos_x, pos_y + 1), // Up
                (pos_x - 1, pos_y), // Left
                (pos_x, pos_y - 1), // Down
            ];
            let home = self.home;
            let current_distance = self.get_current_distance_to(&home.0,&home.1) as f32;
            let deltas : Vec<f32> = potential_moves.iter().map(|(new_x, new_y)|  {
                let mut new_slope = self.calculate_current_slope((*new_x, *new_y)).unwrap_or(f32::MAX);
                let distance = self.get_distance_from_to(&home.0, &home.1, new_x, new_y) as f32;
                if distance >= current_distance{
                    new_slope = f32::MAX;
                }
                (slope - new_slope).abs()
            }).collect();

            let min_index = (deltas
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| 
                    a.partial_cmp(b)
                    .unwrap_or(std::cmp::Ordering::Equal))
                .map(|(index, _)| index))
                .unwrap_or(usize::MAX);
            self.update_position(potential_moves[min_index].0, potential_moves[min_index].1);

        }

    }

}
