extern crate rand;
#[macro_use(s)]
extern crate ndarray;
extern crate permutohedron;

mod make_grid;

//use ndarray::Array2;
use crate::make_grid::{make_board, make_sudoku, display_board};




fn main() {
    let mut board = make_board();
    //board = shuffle_row_nd(board, 1);
    //count_collisions(&board, 1);
    board = make_sudoku(board);
    display_board(&board);
}
