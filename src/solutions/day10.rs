use std::str::Lines;

use crate::solution::Solution;

struct Device {
    ram: i32,
    cycles: Vec<i32>,
    drawing_pos: i32,
}

impl Device {
    fn new() -> Device {
        Device {
            ram: 1,
            cycles: vec![],
            drawing_pos: 0,
        }
    }

    fn noop(&mut self) {
        self.record();
        self.draw();
    }

    fn record(&mut self) {
        self.cycles.push(self.ram);
    }

    fn draw(&mut self) {
        if (self.drawing_pos - self.ram).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if self.drawing_pos == 39 {
            println!();
            self.drawing_pos = 0;
        } else {
            self.drawing_pos += 1;
        }
    }

    fn addx(&mut self, x: i32) {
        self.record();
        self.draw();
        self.draw();
        self.ram += x;
        self.record();
    }

    fn get_cycles(&self) -> &Vec<i32> {
        &self.cycles
    }
}

fn simulate(lines: Lines) -> Device {
    let mut device = Device::new();
    for line in lines {
        let mut splits = line.split_whitespace();
        let command = splits.next().unwrap();
        match command {
            "noop" => {
                device.noop();
            }
            "addx" => {
                device.addx(splits.next().unwrap().parse::<i32>().unwrap());
            }
            _ => unreachable!(),
        }
    }

    device
}
pub struct Day10Part1 {}

impl Solution for Day10Part1 {
    fn run(&self, input: &str) -> String {
        let device = simulate(input.lines());
        let c = device.get_cycles();
        let mut s = 0;
        for i in (18..220).step_by(40) {
            s += (i+2) as i32 * c[i]
        }

        s.to_string()
    }
}
pub struct Day10Part2 {}

impl Solution for Day10Part2 {
    fn run(&self, input: &str) -> String {
        simulate(input.lines());

        "".to_string()
    }
}
