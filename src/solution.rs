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
    fn run(&self) {
        self.solution.run(self.solution.process(&self.datafile));
    }

    fn debug(&self) {
        self.solution.run(self.solution.process(&self.debugfile));
    }
}

pub struct Config {
    _custom: HashMap<String, bool>,
}

impl Config {
    fn push(&mut self, k: String, v: bool) {
        self._custom.insert(k, v);
    }

    fn get(&self, k: &str) -> &bool {
        self._custom.get(k).unwrap()
    }
}

pub trait Solution {
    fn run(&self, data: Box<dyn Any>) -> String;

    fn process(&self, path: &str) -> Box<dyn Any>;
}

