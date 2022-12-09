use crate::solution::Solution;

pub struct Day4Part1 {}

fn to_interval(s: &str) -> (i32, i32) {
    let interval: Vec<&str> = s.split('-').collect();
    let min: i32 = interval[0].parse().unwrap();
    let max: i32 = interval[1].parse().unwrap();

    (min, max)
}

fn split2intervals(line: &String) -> (i32, i32, i32, i32) {
    let splits: Vec<&str> = line.split(',').collect();
    let (left_min, left_max) = to_interval(splits[0]);
    let (right_min, right_max) = to_interval(splits[1]);

    (left_min, left_max, right_min, right_max)
}

fn be_fully_contained(line: &String) -> bool {
    let (lmin, lmax, rmin, rmax) = split2intervals(line);
    if (lmin <= rmin && lmax >= rmax) || (rmin <= lmin && rmax >= lmax) {
        return true;
    }

    false
}

fn be_overlap(line: &String) -> bool {
    let (lmin, lmax, rmin, rmax) = split2intervals(line);
    if (lmax < rmin) || (rmax < lmin) {
        return false;
    }
    true
}

impl Solution for Day4Part1 {
    fn run(&self, input: &String) -> i32 {
        input
            .lines()
            .map(|x| be_fully_contained(&String::from(x)))
            .filter(|x| *x)
            .count() as i32
    }
}

pub struct Day4Part2 {}

impl Solution for Day4Part2 {
    fn run(&self, input: &String) -> i32 {
        input
            .lines()
            .map(|x| be_overlap(&String::from(x)))
            .filter(|x| *x)
            .count() as i32
    }
}
