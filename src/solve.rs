use::std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use ndarray::Array2;
use permutohedron::heap_recursive;
use std::{thread, time};

//use crate::make_grid::count_collisions;


fn count_collisions(board: &Array2<usize>) -> usize {
    let mut collisions = 0;

    // columns
    let mut all_nonzero: Vec<usize>;
    for col in board.gencolumns() {
        let all_nonzero: Vec<&usize> = col.iter().filter(|x| **x > 0).collect();
        let unique_nonzero: HashSet<&usize> = all_nonzero.iter().cloned().collect();
        collisions += all_nonzero.len() - unique_nonzero.len();
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

pub fn search(mut board: Array2<usize>) -> Array2<usize> {
    let original_board: Array2<usize> = board.clone();
    let mut rng = thread_rng();
    let mut n = 0;
    let full_set: HashSet<usize> = (1..10).collect();
    let mut n_resets = 0;
    let mut max_row = 0;

    while n < 9 {

        // build list of available locations and existing numbers
        let mut available= Vec::new();
        let mut present = Vec::new();
        for (i, x) in board.row(n).iter().enumerate() {
            if *x == 0 {
                available.push(i);
            } else {
                present.push(*x);
            }
        }

        // build list of missing numbers
        let present_set: HashSet<usize> = present.iter().cloned().collect();
        let missing = full_set.difference(&present_set);

        // cycle permutations until collisions == 0, or revert to original board
        let mut missing_vec: Vec<&usize> = missing.collect();
        let mut permutations: Vec<Vec<&usize>> = Vec::new();
        heap_recursive(&mut missing_vec, |permutation| {
            permutations.push(permutation.to_vec())
        });
        let mut best = 81;
        let mut row_solved = false;
        permutations.shuffle(&mut thread_rng());
        for permutation in &permutations {
            for (p, a) in permutation.iter().zip(&available) {
                board[[n, *a]] = **p;
            }
            let c = count_collisions(&board);
            if c < best {
                best = c;
            }
            if best == 0 {
                n += 1;
                row_solved = true;
                if n > max_row {
                    max_row = n;
                    println!("{}", max_row);
                }
                break
            }
        }

        if !row_solved {
            board = original_board.clone();
            n = 0;
            n_resets += 1;
        }
    }
    println!("found solution with {} resets", n_resets);
    board
}