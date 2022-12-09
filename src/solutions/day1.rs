use std::{cmp::Reverse, collections::BinaryHeap, str::Lines};

use crate::solution::Solution;

pub struct Day1Part1 {}

struct Candidates {
    h: BinaryHeap<Reverse<i32>>,
    limit: usize,
}

impl Candidates {
    fn new(n: usize) -> Candidates {
        Candidates {
            h: BinaryHeap::new(),
            limit: n,
        }
    }

    fn update(&mut self, cur: i32) {
        if self.h.len() < self.limit {
            self.h.push(Reverse(cur));
            return;
        }

        if self.h.peek().unwrap() < &Reverse(cur) {
            self.h.pop();
            self.h.push(Reverse(cur));
        }
    }

    fn sum(self) -> i32 {
        self.h.into_iter().map(|Reverse(x)| x).sum()
    }
}

fn cal_max_n(lines: Lines, n: usize) -> i32 {
    let mut cur = 0;
    let mut candidates = Candidates::new(n);
    for line in lines {
        if !line.is_empty() {
            cur += line.parse::<i32>().unwrap();
            continue;
        }

        candidates.update(cur);
        cur = 0;
        continue;
    }

    candidates.sum()
}

impl Solution for Day1Part1 {
    fn run(&self, input: &String) -> String {
        cal_max_n(input.lines(), 1).to_string()
    }
}

pub struct Day1Part2 {}

impl Solution for Day1Part2 {
    fn run(&self, input: &String) -> String {
        cal_max_n(input.lines(), 3).to_string()
    }
}
