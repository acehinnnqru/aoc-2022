use std::{cmp::Reverse, collections::BinaryHeap, str::Lines, any::Any};

use crate::solution::Solution;

pub struct Day1Part1 {}

struct Candidates {
    max_heap: BinaryHeap<Reverse<i32>>,
    limit: usize,
}

impl Candidates {
    fn new(n: usize) -> Candidates {
        Candidates {
            max_heap: BinaryHeap::new(),
            limit: n,
        }
    }

    fn update(&mut self, cur: i32) {
        if self.max_heap.len() < self.limit {
            self.max_heap.push(Reverse(cur));
            return;
        }

        if self.max_heap.peek().unwrap() < &Reverse(cur) {
            self.max_heap.pop();
            self.max_heap.push(Reverse(cur));
        }
    }

    fn sum(self) -> i32 {
        self.max_heap.into_iter().map(|Reverse(x)| x).sum()
    }
}

fn sum_of_top_n_candidates(lines: Lines, n: usize) -> i32 {
    let mut current_count = 0;
    let mut candidates = Candidates::new(n);
    for line in lines {
        if !line.is_empty() {
            current_count += line.parse::<i32>().unwrap();
            continue;
        }

        candidates.update(current_count);
        current_count = 0;
        continue;
    }

    candidates.sum()
}

impl Solution for Day1Part1 {
    fn run(&self, data: Box<dyn Any>) -> String {
        sum_of_top_n_candidates(input.lines(), 1).to_string()
    }
}

pub struct Day1Part2 {}

impl Solution for Day1Part2 {
    fn run(&self, input: &str) -> String {
        sum_of_top_n_candidates(input.lines(), 3).to_string()
    }
}
