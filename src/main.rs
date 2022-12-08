use std::fs;

use aoc_2022::{
    solution::Solution,
    solutions::{
        day1::{Day1Part1, Day1Part2},
        day2::{Day2Part1, Day2Part2},
        day3::{Day3Part1, Day3Part2}, day4::{Day4Part1, Day4Part2},
    },
};

fn main() {
    day_n(1, &Day1Part1 {}, &Day1Part2 {});
    day_n(2, &Day2Part1 {}, &Day2Part2 {});
    day_n(3, &Day3Part1 {}, &Day3Part2 {});
    day_n(4, &Day4Part1 {}, &Day4Part2 {});
}

fn day_n(n: i32, part1: &dyn Solution, part2: &dyn Solution) {
    let input = fs::read_to_string(format!("data/day{}.txt", n)).expect("read failed");
    part1.run(&input);
    part2.run(&input);
}

