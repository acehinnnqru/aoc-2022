use std::collections::VecDeque;

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
}

impl Monkey {
    fn new(items: VecDeque<i32>, operation: Operation, tester: Tester) -> Monkey {
        Monkey {
            items,
            operation,
            tester,
        }
    }

    fn pop(&mut self) -> Option<(usize, i32)> {
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

    fn push(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn run(&self) {
        unimplemented!()
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
