extern crate rand;
#[macro_use(s)]
extern crate ndarray;
extern crate permutohedron;

mod make_grid;
mod solve;

//use ndarray::Array2;
use crate::make_grid::{make_sudoku, display_board};
use crate::solve::search;



fn main() {
    let mut board = make_sudoku(30);
    display_board(&board);
    board = search(board);
    display_board(&board);
}
