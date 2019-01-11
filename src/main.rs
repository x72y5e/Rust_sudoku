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
use std::env;
use ndarray::Array2;


fn new_board(clues: usize) -> Array2<usize> {
    println!("generating puzzle...");
    let (tx, rx) = mpsc::channel();

    for _ in 0..7 {
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let b = make_sudoku(clues);
            let _ = tx1.send(b);
        });
    }

    let board = rx.recv().unwrap();
    display_board(&board);
    board
}

fn get_solution(board: Array2<usize>) {
    let (tx, rx) = mpsc::channel();

    for _ in 0..7 {
        let tx1 = mpsc::Sender::clone(&tx);
        let board_clone = board.clone();
        thread::spawn(move || {
            let b = search(board_clone);
            tx1.send(b).unwrap();
        });
    }

    let solved_board = rx.recv().unwrap();
    display_board(&solved_board);
}

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string())
        .collect();

    match args.as_slice() {

        // no arguments
        [_] => {
            println!("making new puzzle with 20 clues");
            let board = new_board(20);
            println!("solving...");
            get_solution(board);
        },

        // 1 command and 1 argument
        [_, ref cmd, ref num] => {
            match cmd.as_str() {
                "--clues" => {
                    let clues: usize = match num.parse() {
                        Ok(n) => {
                            println!("making board with {} clues", n);
                            n
                        },
                        _ => {
                            println!("should be e.g. --clues 25 (defaulting to 20)");
                            20
                        },
                    };
                    let board = new_board(clues);
                    println!("solving...");
                    get_solution(board);
                },
                "--board" => {
                    let board_vec: Vec<usize> = num
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    match board_vec.len() {
                        81 => {
                            let mut board_ndarray = Array2::zeros((9, 9));
                            for (i, digit) in board_vec.iter().enumerate() {
                                let row = i / 9 as usize;
                                board_ndarray[[row, i - row * 9]] = *digit;
                            }
                            display_board(&board_ndarray);
                            println!("solving...");
                            get_solution(board_ndarray);

                        },
                        _ => println!("error - wrong number of digits."),
                    }
                },
                _ => println!("error!"),
            }
        },

        // anything else
        _ => println!("wrong format."),
    }
}
