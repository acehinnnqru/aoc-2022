pub mod part1;
pub mod part2;

#[derive(Clone)]
enum OperationType {
    Plus,
    Multi,
}

#[derive(Clone)]
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
}
