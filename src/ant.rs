// TODO: FIX CALCULATE SLOPE FUNCTION - FIGURE IT OUT!

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
    fn calculate_slope(&mut self, position: (u32, u32)) -> Option<f32> {
        let (x, y) = position;
    
        if x == 0 {
            // If x is 0, slope is undefined (vertical line), return None to indicate error
            None
        } else {
            match self.status {
                AntState::Returning(_,_ ) => {
                    Some((self.home.0 as f32 - y as f32)  / (self.home.1 as f32 - x as f32))
                },
                AntState::Searching(x, ) => {
                    Some()
                }
            }
            // Otherwise, calculate the slope and wrap it in Some
        }
    }
    
    pub fn goto(&mut self, x: u32, y: u32) {
        let dest_x = self.position.0 ;
        let dest_y = self.position.1 ;
        // gets ant out of the house
        if dest_x <= 1 || dest_y <= 1 {
            self.update_position(dest_x + 1, dest_y + 1);
            return;
        }
        // gets slope and handles error
        let slope = match self.calculate_slope((x,y)) {
            Some(slope) => {
                // Slope calculation was successful, proceed with logic using the slope
                log!("Slope is: {}", slope);
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
            (dest_x + 1, dest_y), // Right
            (dest_x, dest_y + 1), // Up
            (dest_x - 1, dest_y), // Left
            (dest_x, dest_y - 1), // Down
        ];

        let deltas : Vec<f32> = potential_moves.iter().map(|(x, y)|  {
            log!("CALCULATING DELTAS, X:{}, Y:{} : SLOPE{:?}", x,y, self.calculate_slope((*x,*y)));
            let new_slope = match self.calculate_slope((*x, *y)) {
                Some(new_slope) => new_slope,
                None => {
                    log!("SLOPE CALCULATION FAILED");
                    self.position = self.home;
                    self.start_search();
                    f32::MAX
                }
            };
            (slope - new_slope).abs()
        }).collect();

        log!("{:?}", deltas);

        
        let min_index = deltas.iter().enumerate().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(index, _)| index);
        match min_index {
            Some(0) => self.update_position(dest_x + 1, dest_y),
            Some(1) => self.update_position(dest_x, dest_y + 1),
            Some(2) => self.update_position(dest_x - 1, dest_y),
            Some(3) => self.update_position(dest_x, dest_y - 1),
            _ => println!("Unexpected error"),
        };


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
        // Change the ant's status to Searching if it is already at the origin
        if self.position.0 < 3 && self.position.1 < 3 && self::AntState::Returning(x,y) == AntState::Returning(x,y) {
            self.start_search();
            return;
        } 
        if self.position.0 < 1 && self.position.1 > 1 && self.status == AntState::Returning(x,y){
            self.update_position(self.position.0, self.position.1 - 1);
            return;
        } 
        if self.position.1 < 1 && self.position.0 > 1 && self.status == AntState::Returning(x,y){
            self.update_position(self.position.0 - 1, self.position.1 );
            return;
        } 
        if self.status != AntState::Searching(x,y) {

            if self.position.0 <= 1 && self.position.1 > 1 {
                self.update_position(self.position.0 -1, self.position.1);
                return;
            } else if self.position.1 <= 1 && self.position.0 > 1 {
                self.update_position(self.position.0 , self.position.1 -1);
                return;
            }
            
            let slope : f32 = y as f32 - self.home.0 as f32 / x as f32 - self.home.1 as f32;

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

        
            

}
