// TODO: optimize plz
use std::collections::VecDeque;

use crate::solution::Solution;

use super::{Operation, OperationType, Tester};

struct Item {
    val: i32,
    ops: Vec<Operation>,
}

impl Item {
    fn new(val: i32) -> Item {
        Item {
            val,
            ops: vec![],
        }
    }

    fn test(&self, arg: i32) -> bool {
        let mut val = self.val;
        for op in self.ops.iter() {
            val = op.execute(val) % arg;
        }
        val % arg == 0
    }

    fn push(&mut self, op: Operation) {
        self.ops.push(op)
    }
}

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    tester: Tester,
    count: usize,
}

impl Monkey {
    fn new(items: VecDeque<Item>, operation: Operation, tester: Tester) -> Monkey {
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

    fn pop(&mut self) -> Option<(usize, Item)> {
        if self.is_empty() {
            return None;
        }

        self.inspect();
        let mut item = self.items.pop_front().unwrap();

        item.push(self.operation.clone());
        if item.test(self.tester.constant) {
            return Some((self.tester.ttarget, item));
        }

        Some((self.tester.ftarget, item))
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn push(&mut self, item: Item) {
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

        // println!(">>>>>>>");
        // println!(">>>>>>>");
        // for (i, m) in self.monkeys.iter().enumerate() {
        //     println!("Monkey {}: {}", i, m.inspect_times());
        // }
    }
}

fn build_monkey(chunks: &[String]) -> Monkey {
    let mut iter = chunks.iter();

    let _index: usize = iter
        .next()
        .unwrap()
        .trim_end_matches(':')
        .trim_start_matches("Monkey")
        .trim()
        .parse()
        .unwrap();

    let items = iter
        .next()
        .unwrap()
        .trim()
        .split_terminator(':')
        .rev()
        .next()
        .unwrap()
        .split_terminator(&[',', ' '])
        .filter(|x| !x.is_empty())
        .map(|x| Item::new(x.parse::<i32>().unwrap()))
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

    let mut max1 = 0;
    let mut max2 = 0;
    g.get_monkeys()
        .iter()
        .map(|x| x.inspect_times())
        .for_each(|t| {
            if t > max1 || t > max2 {
                max1 = max1.max(max2);
                max2 = t;
            }
        });

    max1 * max2
}

pub struct Day11Part2 {}

impl Solution for Day11Part2 {
    fn run(&self, input: &str) -> String {
        simulate_part1(input, 10000).to_string()
    }
}
