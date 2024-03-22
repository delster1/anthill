
const UNIV_SIZE : u32  = 160;
use crate::log;
#[derive(Clone, Copy)]
pub struct Ant{
    pub position : (u32, u32), // position when ant started going home or random path generated
    pub status: AntState,
    pub home: (u32, u32),
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
    
        let dest_x = {js_sys::Math::random() * 160.0 } as u32;
        let dest_y = {js_sys::Math::random() * 160.0} as u32;
        self.status = AntState::Searching(dest_x, dest_y);
        log!("NEW RANDOM DEST:{dest_x}, {dest_y}");
        self.position = (0,0);
        self.goto(dest_x,dest_y);
    }
    fn calculate_slope(&mut self) -> Option<f32> {
    // determines a static-ish slope depending on antstate that will be compared to determine ant's next move every tick
    // gives slope to a random food when searching, and home when home
        match self.status {
            AntState::Returning(x,y ) => {
                let slope = (self.home.0 as f32 - y as f32)  / (self.home.1 as f32 - x as f32);
                Some(slope)
            },
            AntState::Searching(x, y) => {
                let slope = (y as f32 - self.home.0 as f32) / (x as f32 - self.home.1 as f32);
                Some(slope)
            }
        }
            // Otherwise, calculate the slope and wrap it in Some
        
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
                AntState::Returning(_,_ ) => {
                    let slope = (self.home.0 as f32 - curr_y as f32)  / (self.home.1 as f32 - curr_x as f32);
                    Some(slope)
                },
                AntState::Searching(x, y) => {
                    let slope = (y as f32 - curr_y as f32) / (x as f32 - curr_x as f32);
                    Some(slope)
                }
            }
            // Otherwise, calculate the slope and wrap it in Some
        }
    }   

    fn get_distance_from_to(&mut self, y2 : &u32, x2 : &u32, y1 : &u32, x1 : &u32) -> u32{
        let current_distance_squared : u32 = (y2  - y1 ).pow(2) + (x2  - x1 ).pow(2);
        current_distance_squared
    }

    fn get_current_distance_to(&mut self, x: &u32, y: &u32) -> u32 {
        
        let current_distance_squared : u32 = (x  - self.position.0 ).pow(2) + (y  - self.position.1 ).pow(2);
        current_distance_squared
    }

    pub fn goto(&mut self, x: u32, y: u32) {
        let pos_x = self.position.0 ;
        let pos_y = self.position.1 ;
        // gets ant out of the house
        if pos_x <= 1 || pos_y <= 1 {
            self.update_position(pos_x + 1, pos_y + 1);
            return;
        }
        // gets slope and handles error
        let slope = match self.calculate_slope() {
            Some(slope) => {
                // Slope calculation was successful, proceed with logic using the slope
                slope
            },
            None => {
                // Slope calculation failed (e.g., division by zero), ant teleport home for now..
                log!("SLOPE CALCULATION FAILED");
                self.position = self.home;
                self.start_search();
                return;
            }
        };
        // find move with least change to slope from place where food was found to home
        let potential_moves = [
            (pos_x + 1, pos_y), // Right
            (pos_x, pos_y + 1), // Up
            (pos_x - 1, pos_y), // Left
            (pos_x, pos_y - 1), // Down
        ];

        let current_distance = self.get_current_distance_to(&x, &y) as f32;
        let deltas : Vec<f32> = potential_moves.iter().map(|(new_x, new_y)|  {
            let mut new_slope = self.calculate_current_slope((*new_x, *new_y)).unwrap_or(f32::MAX);
            let distance = self.get_distance_from_to(new_x, new_y, &x, &y) as f32;
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
                .unwrap())
            .map(|(index, _)| index))
            .unwrap_or(usize::MAX);
        self.update_position(potential_moves[min_index].0, potential_moves[min_index].1);
        // log!("{:?} : {:?}",deltas, deltas[min_index]);
        
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
    pub fn get_index (&mut self) -> usize{
        return (self.position.0 * UNIV_SIZE + self.position.1) as usize;
    }

    pub fn return_home(&mut self, x: u32, y:u32) {
        let pos_x = self.position.0;
        let pos_y = self.position.1;
        // Change the ant's status to Searching if it is already at the origin
        if pos_x < 3 && pos_y < 3 && self::AntState::Returning(x,y) == AntState::Returning(x,y) {
            self.start_search();
            return;
        } 
        if pos_x < 1 && pos_y > 1 && self.status == AntState::Returning(x,y){
            self.update_position(pos_x, pos_y - 1);
            return;
        } 
        if pos_y < 1 && pos_x > 1 && self.status == AntState::Returning(x,y){
            self.update_position(pos_x - 1, pos_y );
            return;
        } 
        if self.status != AntState::Searching(x,y) {
            // actual code for returning home below:
            // if on an edge of the screen, keep walking on edge
            if pos_x <= 1 && pos_y > 1 {
                self.update_position(pos_x -1, pos_y);
                return;
            } else if pos_y <= 1 && pos_x > 1 {
                self.update_position(pos_x , pos_y -1);
                return;
            }


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
                    self.position = self.home;
                    self.start_search();
                    return;
                }
            };

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
                    .unwrap())
                .map(|(index, _)| index))
                .unwrap_or(usize::MAX);
            self.update_position(potential_moves[min_index].0, potential_moves[min_index].1);

        }

    }

}
