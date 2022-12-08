use std::{collections::HashSet, str::Lines};

use crate::solution::Solution;

pub struct Day3Part1 {}

fn find_same_char(chars: &Vec<char>) -> char {
    let p = chars.len() / 2;

    let mut char_set = HashSet::<char>::new();
    for i in 0..p {
        char_set.insert(chars[i]);
    }

    for i in p..chars.len() {
        if char_set.contains(&chars[i]) {
            return chars[i];
        }
    }
    unimplemented!()
}

fn priorities(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as u8 - 96) as i32;
    }

    (c as u8 - 64) as i32 + 26
}

impl Day3Part1 {
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
    fn run(&self, input: &String) {
        let ret = self.process(input.lines());
        println!("result of day 3 part 1 is: {}", ret);
    }
}
