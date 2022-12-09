use std::str::Lines;

use crate::solution::Solution;

pub struct Day2Part1 {}

trait Day2 {
    fn process(&self, lines: Lines, f: fn(opponent: &u8, me: &u8) -> u8) -> i32 {
        let mut score = 0;
        for line in lines {
            let bs: Vec<u8> = line.bytes().collect();
            let opponent = bs[0];
            let me = f(&opponent, &bs[2]);

            score += self.point(&opponent, &me);
        }

        score
    }

    fn point(&self, opponent: &u8, me: &u8) -> i32 {
        let choice_point = me - b'A' + 1;
        if opponent == me {
            return (choice_point + 3) as i32;
        }
        if *me == towin(opponent) {
            return (choice_point + 6) as i32;
        }

        choice_point as i32
    }
}

fn towin(opponent: &u8) -> u8 {
    match opponent {
        b'A' => b'B',
        b'B' => b'C',
        b'C' => b'A',
        _ => unimplemented!(),
    }
}

fn tolose(opponent: &u8) -> u8 {
    match opponent {
        b'A' => b'C',
        b'B' => b'A',
        b'C' => b'B',
        _ => unimplemented!(),
    }
}

fn todraw(opponent: &u8) -> u8 {
    return *opponent;
}

impl Day2 for Day2Part1 {}

impl Solution for Day2Part1 {
    fn run(&self, input: &String) -> i32 {
        let f = |_: &u8, y: &u8| -> u8 {
            match y {
                b'X' => b'A',
                b'Y' => b'B',
                b'Z' => b'C',
                _ => unimplemented!(),
            }
        };

        self.process(input.lines(), f)
    }
}

pub struct Day2Part2 {}

impl Day2Part2 {}

impl Day2 for Day2Part2 {}

impl Solution for Day2Part2 {
    fn run(& self, input: &String) -> i32 {
        let f = |x: &u8, y: &u8| -> u8 {
            match y {
                b'X' => tolose(x),
                b'Y' => todraw(x),
                b'Z' => towin(x),
                _ => unimplemented!(),
            }
        };

        self.process(input.lines(), f)
    }
}
