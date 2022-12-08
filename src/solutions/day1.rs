use std::str::Lines;

use crate::solution::Solution;

pub struct Day1Part1 {}

fn cal_max_n(lines: Lines, n: usize) -> Vec<i32> {
    let mut candidates = vec![0; n];
    let iter = lines.into_iter();

    let mut cur = 0;
    for item in iter {
        if !item.is_empty() {
            cur += item.parse::<i32>().unwrap();
            continue;
        } else {
            for i in 0..candidates.len() {
                if cur > candidates[i] {
                    candidates[i] = cur;
                }
            }
            cur = 0;
            continue;
        }
    }

    return candidates;
}

impl Solution for Day1Part1 {
    fn run(&self, input: String) {
        let ret = cal_max_n(input.lines(), 1);
        println!("result of day 1 part 1 is: {}", ret.iter().sum::<i32>());
    }
}
