pub mod part1;
pub mod part2;

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
