// TODO: need to optimize
use std::collections::{HashMap, VecDeque};

use crate::solution::Solution;

struct Recorder {
    m: HashMap<char, i32>,
    q: VecDeque<char>,
    limit: usize,
}

impl Recorder {
    fn new(n: usize) -> Recorder {
        Recorder {
            m: HashMap::<char, i32>::new(),
            q: VecDeque::with_capacity(n),
            limit: n,
        }
    }

    fn pop_front(&mut self) {
        let ch = self.q.pop_front().unwrap();
        
        self.reduce(&ch);
    }

    fn reduce(&mut self, ch: &char) {
        if let Some(x) = self.m.get_mut(ch) {
            *x -= 1;
            if *x == 0 {
                self.m.remove(ch);
            }
        }
    }

    fn insert(&mut self, ch: char) {
        match self.m.get_mut(&ch) {
            Some(x) => {
                *x += 1;
            },
            None => {
                self.m.insert(ch, 1);
            },
        }
        self.q.push_back(ch);
    }

    fn update(&mut self, ch: char) {
        self.pop_front();
        self.insert(ch);
    }

    fn try_insert(&mut self, ch: char) {
        if !self.is_full() {
            self.insert(ch);
        } else {
            self.update(ch);
        }
    }

    fn is_full(&self) -> bool {
        self.q.len() == self.limit
    }

    fn unique(&self) -> bool {
        self.is_full() && self.m.len() == self.q.len()
    }
}

fn first_packet(buffer: &str, n: usize) -> usize {
    let mut recorder = Recorder::new(n);

    for (i, ch) in buffer.chars().enumerate() {
        recorder.try_insert(ch);
        if recorder.unique() {
            return i+1;
        }
    }

    unreachable!()
}

pub struct Day6Part1 {}

impl Solution for Day6Part1 {
    fn run(&self, input: &str) -> String {
        first_packet(input, 4).to_string()
    }
}

pub struct Day6Part2 {}

impl Solution for Day6Part2 {
    fn run(&self, input: &str) -> String {
        first_packet(input, 14).to_string()
    }
}
