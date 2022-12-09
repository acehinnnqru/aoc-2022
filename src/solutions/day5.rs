use std::str::Lines;

use crate::solution::Solution;

struct Operation {
    from: usize,
    to: usize,
    count: usize,
}

fn extract_indexes(source: &mut Vec<&str>) -> Vec<Vec<char>> {
    let last: &str = source.pop().unwrap();

    let mut indexes: Vec<usize> = vec![];
    let mut viewing: bool = false;
    for (i, ch) in last.chars().enumerate() {
        if ch != ' ' && !viewing {
            viewing = true;
            indexes.push(i);
        }

        if ch == ' ' {
            viewing = false;
        }

        if viewing {
            continue;
        }
    }

    let mut origin: Vec<Vec<char>> = vec![vec![]; indexes.len()];
    source.reverse();
    for line in source {
        let line_bytes = line.as_bytes();
        for i in 0..indexes.len() {
            let ch = line_bytes[indexes[i]] as char;
            if ch == ' ' {
                continue;
            }
            origin[i].push(ch);
        }
    }

    origin
}

fn extract_operations(source: &Vec<&str>) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];

    fn f(line: &str) -> Operation {
        let splits = line.split_whitespace();
        let mut t = vec![];
        for item in splits {
            match item.parse::<usize>() {
                Ok(x) => t.push(x),
                _ => {}
            };
        }

        Operation {
            from: t[1] - 1,
            to: t[2] - 1,
            count: t[0],
        }
    }

    for line in source {
        operations.push(f(line))
    }

    operations
}

fn process_input(lines: Lines) -> (Vec<Vec<char>>, Vec<Operation>) {
    let mut source_origin: Vec<&str> = vec![];
    let mut source_operations: Vec<&str> = vec![];
    let mut viewing_head = true;
    for line in lines {
        if line.is_empty() {
            viewing_head = false;
            continue;
        }

        if viewing_head {
            source_origin.push(line);
        } else {
            source_operations.push(line);
        }
    }

    (
        extract_indexes(&mut source_origin),
        extract_operations(&source_operations),
    )
}

fn operate(origin: &mut Vec<Vec<char>>, operation: Operation) {
    for _ in 0..operation.count {
        let val = origin[operation.from].pop().unwrap();
        origin[operation.to].push(val);
    }
}

fn multi_operate(origin: &mut Vec<Vec<char>>, operation: Operation) {
    let mut temp = vec![];
    for _ in 0..operation.count {
        temp.push(origin[operation.from].pop().unwrap());
    }
    for _ in 0..operation.count {
        origin[operation.to].push(temp.pop().unwrap());
    }
}

fn peek_tops(origin: &Vec<Vec<char>>) -> String {
    let mut s = String::new();
    for item in origin {
        s.push(*item.last().unwrap());
    }
    s
}

type OperateFn = fn(origin: &mut Vec<Vec<char>>, operation: Operation);

trait Day5 {
    fn operate_func(&self) -> OperateFn;

    fn execute_operations(
        &self,
        origin: &mut Vec<Vec<char>>,
        operations: Vec<Operation>,
        operate_func: OperateFn,
    ) {
        for operation in operations {
            operate_func(origin, operation);
        }
    }

    fn execute(&self, input: &str) -> String {
        let (mut o, os) = process_input(input.lines());
        self.execute_operations(&mut o, os, self.operate_func());
        peek_tops(&o)
    }
}

pub struct Day5Part1 {}

impl Day5 for Day5Part1 {
    fn operate_func(&self) -> OperateFn {
        operate
    }
}

impl Solution for Day5Part1 {
    fn run(&self, input: &str) -> String {
        self.execute(input)
    }
}

pub struct Day5Part2 {}

impl Day5 for Day5Part2 {
    fn operate_func(&self) -> OperateFn {
        multi_operate
    }
}

impl Solution for Day5Part2 {
    fn run(&self, input: &str) -> String {
        self.execute(input)
    }
}
