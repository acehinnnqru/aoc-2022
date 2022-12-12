use std::collections::VecDeque;

enum OperationType {
    Plus,
    Sub,
    Multi,
    Divide,
}

struct Operation {
    ot: OperationType,
    constant: i32,
}

impl Operation {
    fn new(ot: OperationType, constant: i32) -> Operation {
        Operation { ot, constant }
    }

    fn execute(&self, old: i32) -> i32 {
        match self.ot {
            OperationType::Plus => old + self.constant,
            OperationType::Sub => old - self.constant,
            OperationType::Multi => old * self.constant,
            OperationType::Divide => old / self.constant,
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
    fn new(items: Vec<i32>, operation: Operation, tester: Tester) -> Monkey {
        Monkey {
            items: VecDeque::<i32>::from_iter(items.iter().cloned()),
            operation,
            tester,
        }
    }

    fn pop(&self) -> (usize, i32) {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
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
    unimplemented!()
}

fn build_game(input: &str) -> Game {
    let mut game = Game::new();
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let chunks = lines.chunks(5);
    for chunk in chunks {
        let monkey = build_monkey(chunk);
        game.push(monkey);
    }
    unimplemented!()
}
