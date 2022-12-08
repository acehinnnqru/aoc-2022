use std::fs;

use aoc_2022::{
    solution::Solution,
    solutions::{
        day1::{Day1Part1, Day1Part2},
        day2::{Day2Part1, Day2Part2}, day3::Day3Part1,
    },
};

fn main() {
    day1();
    day2();
    day3();
}

fn day1() {
    let input = fs::read_to_string("data/day1.txt").expect("read failed");
    let s = Day1Part1 {};
    s.run(&input);
    let s = Day1Part2 {};
    s.run(&input);
}

fn day2() {
    let input = fs::read_to_string("data/day2.txt").expect("read failed");
    let s = Day2Part1 {};
    s.run(&input);
    let s = Day2Part2 {};
    s.run(&input);
}


fn day3() {
    let input = fs::read_to_string("data/day3.txt").expect("read failed");
    let s = Day3Part1 {};
    s.run(&input);
    // let s = Day2Part2 {};
    // s.run(&input);
}
