use::std::collections::HashSet;
//use std::{thread, time};
use rand::seq::SliceRandom;
use rand::thread_rng;
use ndarray::Array2;
use permutohedron::heap_recursive;


pub fn make_board() -> Array2<usize> {
    let mut board = Array2::zeros((9, 9));

    for i in 0..9 {
        board[[0, i]] = i + 1;
    }

    board
        .row_mut(0)
        .into_slice()
        .expect("contiguous row")
        .shuffle(&mut thread_rng());

    board
}

pub fn shuffle_row(mut board: Array2<usize>, n: usize) -> Array2<usize> {
    for i in 0..9 {
        board.row_mut(n)[i] = i + 1;
    }

    board
        .row_mut(n)
        .into_slice()
        .expect("contiguous row")
        .shuffle(&mut thread_rng());
    board
}

pub fn count_collisions(board: &Array2<usize>, n: usize) -> usize {
    // n is the row we are up to
    let mut collisions = 0usize;


    // columns
    for col in board.gencolumns() {
        let present: HashSet<usize> = col.iter().filter(|x| **x != 0).cloned().collect();
        collisions += (n + 1) - present.len();
    }

    // 3x3 squares
    for row_n in (0..9).step_by(3) {
        for col_n in (0..9).step_by(3) {
            let square = board.slice(s![row_n..row_n + 3,
                                                                     col_n..col_n + 3]);
            let present: Vec<&usize> = square.iter()
                                             .filter(|x| **x != 0)
                                             .collect();
            let unique: HashSet<&usize> = square.iter()
                                                .filter(|x| **x != 0)
                                                .collect();
            collisions += present.len() - unique.len();
        }
    }

    collisions
}

pub fn display_board(board: &Array2<usize>) {
    println!();
    for row in board.genrows() {
        println!("{:?}", row);
    }
    println!();
}

fn try_permutations(mut board: Array2<usize>, mut n: usize) -> Array2<usize> {
    let mut best = 81;
    let mut nums = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut nums, |permutation| {
        permutations.push(permutation.to_vec())
    });

    while n < 9 {
        for permutation in &permutations {
            let this_permutation = permutation;
            for (i, p) in this_permutation.iter().enumerate() {
                board[[n, i]] = *p;
            }

            let collisions = count_collisions(&board, n);
            if collisions < best {
                best = collisions;
                println!("row {} best {}", n, best);

                if best == 0 {
                    n += 1;
                    best = 81;
                    break;
                }
            }
        }
    }
    board
}

pub fn make_sudoku(mut board: Array2<usize>) -> Array2<usize> {
    let mut best = 81;
    let mut n = 1;

    while n < 7 {
        board = shuffle_row(board, n);
        let collisions = count_collisions(&board, n);
            if collisions < best {
                best = collisions;
                println!("row {} best {}", n, best);

                if best == 0 {
                    n += 1;
                    best = 81;
                }
            }
    }

    try_permutations(board, n)
}
