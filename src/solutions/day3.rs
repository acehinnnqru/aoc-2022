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

pub struct Day3Part2 {}

impl Day3Part2 {
    fn process(&self, lines: Lines) -> i32 {
        let l: Vec<String> = lines.map(String::from).collect();
        let groups: Vec<&[String]> = l.chunks(3).collect();

        let mut points = 0;
        for group in groups {
            let mut charset = HashSet::<char>::new();
            for ch in group[0].chars() {
                charset.insert(ch);
            }
            for i in 1..group.len() {
                let mut new_charset = HashSet::<char>::new();
                for ch in group[i].chars() {
                    if charset.contains(&ch) {
                        new_charset.insert(ch);
                    }
                }
                charset = new_charset;
            }
            if charset.len() > 1 {
                unimplemented!()
            }
            let item = charset.iter().next().unwrap();
            points += priorities(*item)
        }

        points
    }
}

impl Solution for Day3Part2 {
    fn run(&self, input: &String) {
        let ret = self.process(input.lines());
        println!("result of day 3 part 2 is: {}", ret);
    }
}
