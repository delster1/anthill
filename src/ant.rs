const UNIV_SIZE : u32  = 160;
use crate::log;
use std::fmt;
#[derive(Clone, Copy)]
pub struct Ant{
    pub position : (u32, u32), // position when ant started going home or random path generated
    pub status: AntState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AntState {
    Searching(u32, u32),
    Returning(u32, u32),
}

impl Ant{
    fn is_position_within_bounds(&self, x: u32, y: u32) -> bool {
        x < UNIV_SIZE && y < UNIV_SIZE
    }

    fn set_random_destination(&mut self) {
        let dest_x ={ js_sys::Math::random().round() as u32} * UNIV_SIZE;
        let dest_y = {js_sys::Math::random().round() as u32} * UNIV_SIZE;

        self.status = AntState::Searching(dest_x, dest_y);
    }

    pub fn goto(&mut self, x: u32, y: u32) {
        let dest_x = self.position.0 ;
        let dest_y = self.position.1 ;

        if dest_x <= 1 || dest_y <= 1 {
            self.update_position(dest_x + 1, dest_y + 1);
            return;
        }

        log!("Going to: ({},{})", x, y);
        let slope : u32 = x / y;

        let mut delta_x = slope - (dest_x - 1) / dest_y;
        let mut delta_y = slope - dest_x  / (dest_y - 1);

        delta_x = delta_x.max(0);
        delta_y = delta_y.max(0);

        if delta_x < delta_y {
            self.update_position(dest_x + 1, dest_y);
        } else if delta_x > delta_y {
            self.update_position(dest_x, dest_y + 1);
        } else {
            self.update_position(dest_x + 1, dest_y + 1);
        }

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
            self.set_random_destination();
            return;
        } 
        if self.position.0 < 1 && self.position.1 > 1 && self::AntState::Returning(x,y) == AntState::Returning(x,y){
            self.update_position(self.position.0 -1, self.position.1);
            return;
        } 
        if self.position.1 < 1 && self.position.0 > 1 && self::AntState::Returning(x,y) == AntState::Returning(x,y){
            self.update_position(self.position.0 , self.position.1 -1);
            return;
        } 
        self.goto(x, y);
    }

        
            

}