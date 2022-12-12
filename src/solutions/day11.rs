use std::collections::VecDeque;

use crate::solution::Solution;

enum OperationType {
    Plus,
    Multi,
}

struct Operation {
    ot: OperationType,
    constant: i32,
    old_itself: bool,
}

impl Operation {
    fn new(ot: OperationType, constant: i32, old_itself: bool) -> Operation {
        Operation {
            ot,
            constant,
            old_itself,
        }
    }

    fn execute(&self, old: i32) -> i32 {
        let mut arg = self.constant;
        if self.old_itself {
            arg = old;
        }
        match self.ot {
            OperationType::Plus => old + arg,
            OperationType::Multi => old * arg,
        }
    }
}

struct Tester {
    constant: i32,
    ttarget: usize,
    ftarget: usize,
}

impl Tester {
    fn new(constant: i32, ttarget: usize, ftarget: usize) -> Tester {
        Tester {
            constant,
            ttarget,
            ftarget,
        }
    }

    fn test(&self, worry_level: i32) -> usize {
        if worry_level % self.constant == 0 {
            return self.ttarget;
        }

        self.ftarget
    }
}

struct Monkey {
    items: VecDeque<i32>,
    operation: Operation,
    tester: Tester,
    count: usize,
}

impl Monkey {
    fn new(items: VecDeque<i32>, operation: Operation, tester: Tester) -> Monkey {
        Monkey {
            items,
            operation,
            tester,
            count: 0,
        }
    }

    fn inspect(&mut self) {
        self.count += 1;
    }

    fn inspect_times(&self) -> usize {
        self.count
    }

    fn pop(&mut self) -> Option<(usize, i32)> {
        self.inspect();
        if self.is_empty() {
            return None;
        }

        let mut item = self.items.pop_front().unwrap();

        item = self.operation.execute(item);
        item /= 3;
        let target = self.tester.test(item);

        Some((target, item))
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn push(&mut self, item: i32) {
        self.items.push_back(item);
    }
}

struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    fn new() -> Game {
        Game { monkeys: vec![] }
    }

    fn get_monkeys(&self) -> &Vec<Monkey> {
        &self.monkeys
    }

    fn push(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn run(&mut self, rounds: usize) {
        (0..rounds).for_each(|_| self.run_single_round());
    }

    fn run_single_round(&mut self) {
        for i in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[i];
            if monkey.is_empty() {
                continue;
            }

            let mut waiting_list = vec![];
            loop {
                if let Some(x) = monkey.pop() {
                    waiting_list.push(x);
                } else {
                    break;
                }
            }

            for (target, item) in waiting_list {
                self.monkeys[target].push(item)
            }
        }
    }
}

fn build_monkey(chunks: &[String]) -> Monkey {
    let mut iter = chunks.iter();

    // let index: usize = iter
    //     .next()
    //     .unwrap()
    //     .trim_end_matches(':')
    //     .trim_start_matches("Monkey")
    //     .parse()
    //     .unwrap();

    let items: VecDeque<i32> = iter
        .next()
        .unwrap()
        .trim_start_matches("Starting items:")
        .trim()
        .split_terminator(&[',', ' '])
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut op_splits = iter.next().unwrap().split_whitespace().rev();
    let (old_itself, constant) = match op_splits.next().unwrap() {
        "old" => (true, 0),
        d => (false, d.parse().unwrap()),
    };
    let ot = match op_splits.next().unwrap() {
        "*" => OperationType::Multi,
        "+" => OperationType::Plus,
        _ => unreachable!(),
    };
    let operation = Operation::new(ot, constant, old_itself);

    let mut last_number = || {
        iter.next()
            .unwrap()
            .split_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap()
    };
    let test_arg: i32 = last_number();
    let ttarget: usize = last_number() as usize;
    let ftarget: usize = last_number() as usize;
    let tester = Tester::new(test_arg, ttarget, ftarget);

    Monkey::new(items, operation, tester)
}

fn build_game(input: &str) -> Game {
    let mut game = Game::new();
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let chunks = lines.chunks(7);

    chunks.for_each(|chunk| {
        game.push(build_monkey(chunk));
    });

    game
}

fn simulate_part1(input: &str, rounds: usize) -> usize {
    let mut g = build_game(input);
    g.run(rounds);

    g.get_monkeys().iter().map(|x| x.inspect_times()).sum()
}

pub struct Day11Part1 {}

impl Solution for Day11Part1 {
    fn run(&self, input: &str) -> String {
        simulate_part1(input, 20).to_string()
    }
}
