use std::vec;
use rand::Rng;

type  Position = (i8 , i8);
pub struct Minesweeper {
    height: u8,
    width: u8,
    open_positions: Vec<Position>,
    mines_positions: Vec<Position>,
    flags_postions: Vec<Position>,
}

impl Minesweeper {

    fn new(height:u8 , width:u8) -> Minesweeper {
        Minesweeper{
            height ,
            width,
            open_positions: Vec::new(),
            mines_positions: Vec::new(),
            flags_postions: Vec::new(),
        }
    }

    fn click(&mut self , pos:Position) {

    }

    fn is_pos_mines(pos:Position)-> bool {
        return true
    }


    fn mark_as_flag(&mut self , pos:Position) {
        
    }

    fn generate_miesses(&mut self, number_minesses:i8) {
        let min_row:u8 = 0;
        let max_row:u8 = self.width;
        let min_col:u8 = 0;
        let max_col:u8 = self.height;
        let mut i:i8 = 0;
        while i <= number_minesses {
            self.mines_positions.insert(i, (self.rand_number(0 , self.width) , self.rand_number(0 , self.height)));
            i++;
        }
        
    }

    fn rand_number(min:i8 , max:i8) -> i8 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(min..max);
    }
}

fn main () {

}