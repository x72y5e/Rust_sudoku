extern crate rand;
#[macro_use(s)]
extern crate ndarray;
extern crate permutohedron;

mod make_grid;
mod solve;

use crate::make_grid::{make_sudoku, display_board};
use crate::solve::search;
use std::thread;
use std::sync::mpsc;



fn main() {

    let (tx, rx) = mpsc::channel();

    for _ in 0..7 {
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let b = make_sudoku(30);
            let _ = tx1.send(b);
        });
    }

    let board = rx.recv().unwrap();
    display_board(&board);

    drop(tx);
    drop(rx);

    let (tx, rx) = mpsc::channel();

    for _ in 0..7 {
        let tx1 = mpsc::Sender::clone(&tx);
        let board_clone = board.clone();
        thread::spawn(move || {
            let b= search(board_clone);
            tx1.send(b).unwrap();
        });
    }

    let solved_board = rx.recv().unwrap();
    display_board(&solved_board);
}
