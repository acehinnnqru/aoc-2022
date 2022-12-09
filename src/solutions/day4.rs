use crate::solution::Solution;

fn to_interval(s: &str) -> (i32, i32) {
    let interval: Vec<&str> = s.split('-').collect();
    let min: i32 = interval[0].parse().unwrap();
    let max: i32 = interval[1].parse().unwrap();

    (min, max)
}

fn split2intervals(line: &str) -> (i32, i32, i32, i32) {
    let splits: Vec<&str> = line.split(',').collect();
    let (left_min, left_max) = to_interval(splits[0]);
    let (right_min, right_max) = to_interval(splits[1]);

    (left_min, left_max, right_min, right_max)
}

fn be_fully_contained(line: &str) -> bool {
    let (lmin, lmax, rmin, rmax) = split2intervals(line);
    if (lmin <= rmin && lmax >= rmax) || (rmin <= lmin && rmax >= lmax) {
        return true;
    }

    false
}

fn be_overlap(line: &str) -> bool {
    let (lmin, lmax, rmin, rmax) = split2intervals(line);
    if (lmax < rmin) || (rmax < lmin) {
        return false;
    }

    true
}

trait Day4 {
    fn compare_func(&self) -> fn(&str) -> bool;
    fn process(&self, input: &str) -> String {
        input
            .lines()
            .map(|x| self.compare_func()(&String::from(x)))
            .filter(|x| *x)
            .count()
            .to_string()
    }
}

pub struct Day4Part1 {}

impl Day4 for Day4Part1 {
    fn compare_func(&self) -> fn(&str) -> bool {
        be_fully_contained
    }
}

impl Solution for Day4Part1 {
    fn run(&self, input: &str) -> String {
        self.process(input)
    }
}

pub struct Day4Part2 {}

impl Day4 for Day4Part2 {
    fn compare_func(&self) -> fn(&str) -> bool {
        be_overlap
    }
}

impl Solution for Day4Part2 {
    fn run(&self, input: &str) -> String {
        self.process(input)
    }
}
