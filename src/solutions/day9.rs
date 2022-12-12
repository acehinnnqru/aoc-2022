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

    fn follow(&mut self, knot: &Knot) {
        if self.do_nothing(knot) {
            return;
        }

        if self.is_in_row(knot) {
            self.align_x(knot);
            return;
        }

        if self.is_in_column(knot) {
            self.align_y(knot);
            return;
        }

        if self.is_far_away(knot) {
            self.align(knot);
            return;
        }
        unreachable!()
    }

    fn align(&mut self, knot: &Knot) {
        self.align_y(knot);
        self.align_x(knot);
    }

    fn align_y(&mut self, knot: &Knot) {
        if knot.y < self.y {
            self.step_down();
            return;
        } else if knot.y > self.y {
            self.step_up();
            return;
        }

        unreachable!()
    }

    fn align_x(&mut self, knot: &Knot) {
        if knot.x < self.x {
            self.step_left();
            return;
        } else if knot.x > self.x {
            self.step_right();
            return;
        }
        unreachable!()
    }

    fn is_in_row(&self, knot: &Knot) -> bool {
        self.y == knot.y
    }

    fn is_in_column(&self, knot: &Knot) -> bool {
        self.x == knot.x
    }

    fn do_nothing(&self, knot: &Knot) -> bool {
        self.is_overlap(knot) || self.is_touching(knot) || self.is_diagonal(knot)
    }

    fn is_overlap(&self, knot: &Knot) -> bool {
        self.x == knot.x && self.y == knot.y
    }

    fn is_touching(&self, knot: &Knot) -> bool {
        ((self.x - knot.x).abs() == 1 && self.is_in_row(knot))
            || (self.is_in_column(knot) && (self.y - knot.y).abs() == 1)
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
        }
    }

    fn simulate(&mut self, motion: Motion, steps: usize) {
        for _ in 0..steps {
            self.step(&motion);
            self.trace_tail();
        }
    }

    fn trace_tail(&mut self) {
        let k = self.knots.last().unwrap().clone();
        self.tail_visited.push(k);
    }

    fn follow(&mut self, i: usize) {
        let b = self.knots[i - 1].clone();
        self.knots[i].follow(&b);
    }

    fn step(&mut self, motion: &Motion) {
        let head = &mut self.knots[0];
        match motion {
            Motion::U => head.step_up(),
            Motion::D => head.step_down(),
            Motion::L => head.step_left(),
            Motion::R => head.step_right(),
        }

        for i in 1..self.knots.len() {
            self.follow(i)
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
        if knot.x > xmax {
            xmax = knot.x;
        }
        if knot.y < ymin {
            ymin = knot.y;
        }
        if knot.y > ymax {
            ymax = knot.y;
        }
    }
    xmin -= 3;
    ymin -= 3;
    xmax += 3;
    ymax += 3;
    println!("{}", ">".repeat((xmax - xmin + 1) as usize));
    println!("boundary: {},{},{},{}", xmin, xmax, ymin, ymax);
    println!("{:?}", knots_map);
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
