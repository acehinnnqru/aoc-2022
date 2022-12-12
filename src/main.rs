use std::fs;

use aoc_2022::{
    solution::Solution,
    solutions::{
        day1::{Day1Part1, Day1Part2},
        day10::{Day10Part1, Day10Part2},
        day2::{Day2Part1, Day2Part2},
        day3::{Day3Part1, Day3Part2},
        day4::{Day4Part1, Day4Part2},
        day5::{Day5Part1, Day5Part2},
        day6::{Day6Part1, Day6Part2},
        day7::{Day7Part1, Day7Part2},
        day8::{Day8Part1, Day8Part2},
        day9::{Day9Part1, Day9Part2}, day11::{part1::Day11Part1, part2::Day11Part2},
    },
};

fn main() {
    let days: Vec<Vec<&dyn Solution>> = vec![
        vec![&Day1Part1 {}, &Day1Part2 {}],
        vec![&Day2Part1 {}, &Day2Part2 {}],
        vec![&Day3Part1 {}, &Day3Part2 {}],
        vec![&Day4Part1 {}, &Day4Part2 {}],
        vec![&Day5Part1 {}, &Day5Part2 {}],
        vec![&Day6Part1 {}, &Day6Part2 {}],
        vec![&Day7Part1 {}, &Day7Part2 {}],
        vec![&Day8Part1 {}, &Day8Part2 {}],
        vec![&Day9Part1 {}, &Day9Part2 {}],
        vec![&Day10Part1 {}, &Day10Part2 {}],
        vec![&Day11Part1 {}, &Day11Part2 {}],
    ];

    days.iter()
        .enumerate()
        .for_each(|(x, y)| day_n((x + 1) as i32, y.to_vec()));
}

fn day_n(n: i32, parts: Vec<&dyn Solution>) {
    let input = fs::read_to_string(format!("data/day{}.txt", n)).expect("read failed");

    parts.iter().enumerate().for_each(|(i, part)| {
        print!("Result of Day {} Part {} is:", n, i + 1);
        println!("{}", part.run(&input));
    });
}
