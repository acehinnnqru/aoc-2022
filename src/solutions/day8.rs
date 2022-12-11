use std::str::Lines;

use crate::solution::Solution;

pub struct Grid {
    data: Vec<Vec<usize>>,
}

impl Grid {
    fn new(data: Lines) -> Grid {
        let grid = Grid { data: Vec::<Vec<usize>>::new() };

        grid
    }

    fn build_grid(&self) {
        
    }
    fn edge_tree_count(&self) -> i32 {
        0
    }

    fn find(&self) {
        
    }
}

pub struct Day8Part1 {}

impl Solution for Day8Part1 {
    fn run(&self, input: &str) -> String {
        unimplemented!()
    }
}
