use::std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;
use ndarray::Array2;
use permutohedron::heap_recursive;
use crate::make_grid::count_collisions;


pub fn search(mut board: Array2<usize>) -> Array2<usize> {
    let original_board: Array2<usize> = board.clone();
    let mut n = 0;
    let full_set: HashSet<usize> = (1..10).collect();

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
                break
            }
        }

        if !row_solved {
            board = original_board.clone();
            n = 0;
        }
    }
    board
}