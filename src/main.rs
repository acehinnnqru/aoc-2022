use std::fs;

use aoc_2022::{solutions::day1::Day1Part1, solution::Solution};

fn main() {
    day1();
}

fn day1() {
    let input = fs::read_to_string("tests/day1.txt").expect("read failed");
    let s = Day1Part1 {};
    s.run(input);
}
