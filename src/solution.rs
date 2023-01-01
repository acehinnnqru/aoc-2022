use std::{any::Any, collections::HashMap};

fn debug(q: Question) {
    q.debug()
}

fn run(q: Question) {
    q.run()
}

pub struct Question {
    debugfile: String,
    datafile: String,

    solution: Box<dyn Solution>,
}

impl Question {
    fn run_part1(&self) {
        self.solution.run_part1(self.solution.process(&self.datafile));
    }
    fn run(&self) {
        self.solution.run(self.solution.process(&self.datafile));
    }

    fn debug(&self) {
        self.solution.run(self.solution.process(&self.debugfile));
    }
}

type SolutionFn = fn(data: Box<dyn Any>) -> String;
type ProcessFn = fn(path: &str) -> Box<dyn Any>;

pub trait Solution {
    fn part1(&self) -> SolutionFn;

    fn part2(&self) -> SolutionFn;

    fn process(&self) -> ProcessFn;
}

