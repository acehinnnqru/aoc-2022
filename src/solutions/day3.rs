use std::{collections::HashSet, str::Lines};

use crate::solution::Solution;

fn build_charset(chars: Vec<char>) -> HashSet<char> {
    let mut charset = HashSet::<char>::new();
    chars.iter().for_each(|x| {charset.insert(*x);});

    charset
}

fn find_same_char(chars: &Vec<char>) -> char {
    let p = chars.len() / 2;

    let charset = build_charset(chars.to_vec());

    for item in chars.iter().skip(p) {
        if charset.contains(item) {
            return *item;
        }
    }

    unreachable!()
}

fn priorities(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as u8 - 96) as i32;
    }

    (c as u8 - 64) as i32 + 26
}

trait Day3 {
    fn process(&self, lines: Lines) -> i32;
}

pub struct Day3Part1 {}

impl Day3 for Day3Part1 {
    fn process(&self, lines: Lines) -> i32 {
        let mut total = 0;
        for line in lines {
            let cs: Vec<char> = line.chars().collect();
            let c = find_same_char(&cs);
            total += priorities(c);
        }

        total
    }
}

impl Solution for Day3Part1 {
    fn run(&self, input: &str) -> String {
        self.process(input.lines()).to_string()
    }
}

pub struct Day3Part2 {}

fn update_charset(group: &[String], charset: &mut HashSet<char>) {
    for item in group {
        let temp_charset = build_charset(item.chars().collect());
        charset.retain(|x| temp_charset.contains(x));
    }
}

impl Day3 for Day3Part2 {
    fn process(&self, lines: Lines) -> i32 {
        let l: Vec<String> = lines.map(String::from).collect();
        let groups: Vec<&[String]> = l.chunks(3).collect();

        let mut points = 0;
        for group in groups {
            let mut charset = build_charset(group[0].chars().collect());
            update_charset(&group[1..], &mut charset);

            if charset.len() > 1 {
                unreachable!()
            }

            let item = charset.iter().next().unwrap();
            points += priorities(*item)
        }

        points
    }
}

impl Solution for Day3Part2 {
    fn run(&self, input: &str) -> String {
        self.process(input.lines()).to_string()
    }
}
