use std::str::Lines;

use crate::solution::Solution;

pub struct Day1Part1 {}

fn update_candidates(candidates: &mut Vec<i32>, cur: i32) {
    for i in 0..candidates.len() {
        if cur > candidates[i] {
            candidates[i] = cur;
            return;
        }
    }
}

fn cal_max_n(lines: Lines, n: usize) -> Vec<i32> {
    let mut cur = 0;
    let mut candidates = vec![0; n];
    for line in lines {
        if !line.is_empty() {
            cur += line.parse::<i32>().unwrap();
            continue;
        }
        
        update_candidates(&mut candidates, cur);
        cur = 0;
        continue;
    }

    return candidates;
}

impl Solution for Day1Part1 {
    fn run(&self, input: &String) {
        let ret = cal_max_n(input.lines(), 1);
        println!("result of day 1 part 1 is: {}", ret.iter().sum::<i32>());
    }
}

pub struct Day1Part2 {}

impl Solution for Day1Part2 {
    fn run(&self, input: &String) {
        let ret = cal_max_n(input.lines(), 3);
        println!("result of day 1 part 2 is: {}", ret.iter().sum::<i32>());
    }
}
