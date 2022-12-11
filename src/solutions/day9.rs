use std::{str::Lines, collections::HashSet};

use crate::solution::Solution;

struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Knot {
        Knot { x: 0, y: 0 }
    }

    fn go_up(&mut self) {
        self.x += 1;
    }

    fn go_down(&mut self) {
        self.x -= 1;
    }

    fn go_left(&mut self) {
        self.y -= 1;
    }

    fn go_right(&mut self) {
        self.y += 1;
    }

    fn align_y(&mut self, knot: &Knot) {
        self.y = knot.y
    }

    fn align_x(&mut self, knot: &Knot) {
        self.x = knot.x
    }

    fn is_overlap(&self, knot: &Knot) -> bool {
        self.x == knot.x && self.y == knot.y
    }

    fn is_touching(&self, knot: &Knot) -> bool {
        ((self.x - knot.x).abs() == 1 && self.y == knot.y)
            || (self.x == knot.x && (self.y - knot.y).abs() == 1)
    }

    fn is_diagonal(&self, knot: &Knot) -> bool {
        ((self.x - knot.x) * (self.y - knot.y)).abs() == 1
    }

    fn is_far_away(&self, knot: &Knot) -> bool {
        (self.x != knot.x && (self.y - knot.y).abs() >= 2)
            || ((self.x - knot.x).abs() >= 2 && self.y != knot.y)
    }
}

impl ToString for Knot {
    fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

struct Rope {
    head: Knot,
    tail: Knot,
    tail_vec: Vec<String>,
}

enum Motion {
    U(usize),
    D(usize),
    L(usize),
    R(usize),
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Knot::new(),
            tail: Knot::new(),
            tail_vec: vec![],
        }
    }

    fn simulate(&mut self, motion: Motion) {
        match motion {
            Motion::U(steps) => self.move_up(steps),
            Motion::D(steps) => self.move_down(steps),
            Motion::L(steps) => self.move_left(steps),
            Motion::R(steps) => self.move_right(steps),
        }
    }

    fn to_be_overlap(&self) -> bool {
        self.tail.is_overlap(&self.head)
    }

    fn to_be_diagonal(&self) -> bool {
        self.tail.is_diagonal(&self.head)
    }

    fn to_be_touching(&self) -> bool {
        self.tail.is_touching(&self.head)
    }

    fn do_nothing(&self) -> bool {
        self.to_be_overlap() || self.to_be_diagonal() || self.to_be_touching()
    }

    fn align_tail_y(&mut self) {
        self.tail.align_y(&self.head);
    }

    fn align_tail_x(&mut self) {
        self.tail.align_x(&self.head);
    }

    fn trace_tail(&mut self) {
        let p = self.tail.to_string();
        // println!("{}", p);
        self.tail_vec.push(p);
    }

    fn to_be_far_away(&self) -> bool {
        self.tail.is_far_away(&self.head)
    }

    fn move_up(&mut self, steps: usize) {
        for _ in 0..steps {
            self.head.go_up();

            if self.do_nothing() {
                self.trace_tail();
                continue;
            }

            if self.to_be_far_away() {
                self.align_tail_y();
            }

            self.tail.go_up();
            self.trace_tail();
        }
    }

    fn move_down(&mut self, steps: usize) {
        for _ in 0..steps {
            self.head.go_down();
            if self.do_nothing() {
            self.trace_tail();
                continue;
            }

            if self.to_be_far_away() {
                self.align_tail_y();
            }

            self.tail.go_down();
            self.trace_tail();
        }
    }

    fn move_left(&mut self, steps: usize) {
        for _ in 0..steps {
            self.head.go_left();

            if self.do_nothing() {
            self.trace_tail();
                continue;
            }

            if self.to_be_far_away() {
                self.align_tail_x();
            }

            self.tail.go_left();
            self.trace_tail();
        }
    }
    fn move_right(&mut self, steps: usize) {
        for _ in 0..steps {
            self.head.go_right();

            if self.do_nothing() {
            self.trace_tail();
                continue;
            }

            if self.to_be_far_away() {
                self.align_tail_x();
            }

            self.tail.go_right();
            self.trace_tail();
        }
    }
}

fn simulate(lines: Lines) -> usize {
    let mut rope = Rope::new();
    for line in lines {
        let mut splits = line.split_whitespace();

        let direction = splits.next().unwrap();
        let steps = splits.next().unwrap().parse::<usize>().unwrap();

        rope.simulate(match direction {
            "R" => Motion::R(steps),
            "L" => Motion::L(steps),
            "U" => Motion::U(steps),
            "D" => Motion::D(steps),
            _ => unreachable!(),
        });
    }

    HashSet::<String>::from_iter(rope.tail_vec.iter().cloned()).len()
}
pub struct Day9Part1 {}

impl Solution for Day9Part1 {
    fn run(&self, input: &str) -> String {
        simulate(input.lines()).to_string()
    }
}
