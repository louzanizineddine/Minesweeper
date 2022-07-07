use std::vec;


type  Position = (i8 , i8);
pub struct Minesweeper {
    height: u8,
    width: u8,
    open_position: Vec<Position>,
    mines_positions: Vec<Position>,
    flags_postions: Vec<Position>,
}

impl Minesweeper {

    fn new(height:u8 , width:u8) -> Minesweeper {
        Minesweeper{
            height ,
            width,
            open_position: Vec::new(),
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

    fn generate_miesses(&mut self) {

    }
}

fn main () {

}