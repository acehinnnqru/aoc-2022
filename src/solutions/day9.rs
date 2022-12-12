use std::{
    collections::{HashMap, HashSet},
    str::Lines,
};

use crate::solution::Solution;

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Knot {
        Knot { x: 0, y: 0 }
    }

    fn step_up(&mut self) {
        self.y += 1;
    }

    fn step_down(&mut self) {
        self.y -= 1;
    }

    fn step_left(&mut self) {
        self.x -= 1;
    }

    fn step_right(&mut self) {
        self.x += 1;
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
    knots: Vec<Knot>,
    tail_visited: Vec<Knot>,
    // boundary: (xmax, xmin, ymax, ymin)
    boundary: (i32, i32, i32, i32),
}

enum Motion {
    U,
    D,
    L,
    R,
}

impl Rope {
    fn new(count: usize) -> Rope {
        Rope {
            knots: vec![Knot::new(); count],
            tail_visited: vec![],
            boundary: (0, 0, 0, 0),
        }
    }

    fn simulate(&mut self, motion: Motion, steps: usize) {
        for _ in 0..steps {
            match motion {
                Motion::U => self.step_up(),
                Motion::D => self.step_down(),
                Motion::L => self.step_left(),
                Motion::R => self.step_right(),
            }
            self.trace_tail();
            self.print_knots();
        }
    }

    fn trace_tail(&mut self) {
        let k = self.knots.last().unwrap().clone();
        {
            if k.x > self.boundary.0 {
                self.boundary.0 = k.x + 1;
            }
            if k.x < self.boundary.1 {
                self.boundary.1 = k.x - 1;
            }
            if k.y > self.boundary.2 {
                self.boundary.2 = k.y + 1;
            }
            if k.y < self.boundary.3 {
                self.boundary.3 = k.y - 1;
            }
        }
        self.tail_visited.push(k);
    }

    fn to_be_overlap(&self, i: usize) -> bool {
        self.knots[i].is_overlap(&self.knots[i - 1])
    }

    fn to_be_diagonal(&self, i: usize) -> bool {
        self.knots[i].is_diagonal(&self.knots[i - 1])
    }

    fn to_be_touching(&self, i: usize) -> bool {
        self.knots[i].is_touching(&self.knots[i - 1])
    }

    fn do_nothing(&self, i: usize) -> bool {
        self.to_be_overlap(i) || self.to_be_diagonal(i) || self.to_be_touching(i)
    }

    fn to_be_far_away(&self, i: usize) -> bool {
        self.knots[i].is_far_away(&self.knots[i - 1])
    }

    fn align_tail_y(&mut self, i: usize) {
        let b = self.knots[i - 1].clone();
        self.knots[i].align_y(&b);
    }

    fn align_tail_x(&mut self, i: usize) {
        let b = self.knots[i - 1].clone();
        self.knots[i].align_x(&b);
    }

    fn step_up(&mut self) {
        self.knots[0].step_up();
        for i in 1..self.knots.len() {
            if self.do_nothing(i) {
                continue;
            }

            if self.to_be_far_away(i) {
                self.align_tail_x(i);
            }
            self.knots[i].step_up();
        }
    }

    fn step_down(&mut self) {
        self.knots[0].step_down();
        for i in 1..self.knots.len() {
            if self.do_nothing(i) {
                continue;
            }

            if self.to_be_far_away(i) {
                self.align_tail_y(i);
            }
            self.knots[i].step_down();
        }
    }

    fn step_left(&mut self) {
        self.knots[0].step_left();
        for i in 1..self.knots.len() {
            if self.do_nothing(i) {
                continue;
            }

            if self.to_be_far_away(i) {
                self.align_tail_y(i);
            }
            self.knots[i].step_left();
        }
    }

    fn step_right(&mut self) {
        self.knots[0].step_right();
        for i in 1..self.knots.len() {
            if self.do_nothing(i) {
                continue;
            }

            if self.to_be_far_away(i) {
                self.align_tail_y(i);
            }
            self.knots[i].step_right();
        }
    }

    fn tail_visited_set(&self) -> HashSet<String> {
        HashSet::from_iter(
            build_map(&self.tail_visited)
                .keys()
                .into_iter()
                .map(|x| x.to_string()),
        )
    }

    fn print_tail_visited_path(&self) {
        print_knots(&self.tail_visited, false)
    }
    fn print_knots(&self) {
        print_knots(&self.knots, true)
    }
}

fn build_map(knots: &Vec<Knot>) -> HashMap<String, usize> {
    HashMap::<String, usize>::from_iter(
        knots
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, k)| (k.to_string(), i)),
    )
}

fn print_knots(knots: &Vec<Knot>, index: bool) {
    let knots_map = build_map(knots);
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (0, 0, 0, 0);
    for knot in knots {
        if knot.x < xmin {
            xmin = knot.x;
        }
        if knot.x > xmin {
            xmax = knot.x;
        }
        if knot.y < ymin {
            ymin = knot.y;
        }
        if knot.y > ymin {
            ymax = knot.y;
        }
    }
    xmin -= 3;
    ymin -= 3;
    xmax += 3;
    ymax += 3;
    println!("boundary: {},{},{},{}", xmin, xmax, ymin, ymax);
    println!("{}", ">".repeat((xmax - xmin + 1) as usize));
    for j in (ymin..ymax + 1).rev() {
        for i in xmin..xmax + 1 {
            if i == 0 && j == 0 {
                print!("s");
                continue;
            }
            if let Some(ind) = knots_map.get(format!("{},{}", i, j).as_str()) {
                if index {
                    if *ind == 0 {
                        print!("H");
                    } else {
                        print!("{}", ind);
                    }
                } else {
                    print!("#");
                }
                continue;
            }
            print!(".");
        }
        println!("");
    }
    println!("{}", "<".repeat((ymax - ymin + 1) as usize));
}

fn simulate(knots_count: usize, lines: Lines) -> usize {
    let mut rope = Rope::new(knots_count);
    for line in lines {
        let mut splits = line.split_whitespace();

        let direction = splits.next().unwrap();
        let steps = splits.next().unwrap().parse::<usize>().unwrap();

        rope.simulate(
            match direction {
                "R" => Motion::R,
                "L" => Motion::L,
                "U" => Motion::U,
                "D" => Motion::D,
                _ => unreachable!(),
            },
            steps,
        );
    }

    rope.tail_visited_set().len()
}

pub struct Day9Part1 {}

impl Solution for Day9Part1 {
    fn run(&self, input: &str) -> String {
        simulate(2, input.lines()).to_string()
    }
}

pub struct Day9Part2 {}

impl Solution for Day9Part2 {
    fn run(&self, input: &str) -> String {
        simulate(10, input.lines()).to_string()
    }
}
