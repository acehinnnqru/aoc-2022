use std::{collections::HashMap, str::Lines};

use crate::solution::Solution;

type VisibleMap = HashMap<usize, HashMap<usize, Visible>>;

pub struct Grid {
    data: Vec<Vec<usize>>,
    visible_map: VisibleMap,
    rows: usize,
    columns: usize,
}

struct Visible {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Visible {
    fn new() -> Self {
        Visible {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    fn visible(&self) -> bool {
        self.up || self.down || self.left || self.right
    }

    fn invisible(&self) -> bool {
        !self.visible()
    }
}

fn view_through(line: &mut Vec<(usize, &mut bool)>) {
    let mut premax = line.first().unwrap().0;
    for i in 1..line.len() {
        if line[i].0 > premax {
            *line[i].1 = true;
            premax = line[i].0;
        }
    }
}

fn get_mut_visible_obj(m: &mut VisibleMap, i: usize, j: usize) -> &mut Visible {
    m.get_mut(&i).unwrap().get_mut(&j).unwrap()
}

fn get_visible_obj(m: &VisibleMap, i: usize, j: usize) -> &Visible {
    m.get(&i).unwrap().get(&j).unwrap()
}

impl Grid {
    fn new(data: Lines) -> Grid {
        let mut grid = Grid {
            data: Vec::<Vec<usize>>::new(),
            visible_map: HashMap::<_, _>::new(),
            rows: 0,
            columns: 0,
        };

        grid.build_grid(data);
        grid.build_visible_map();

        grid
    }

    fn build_grid(&mut self, lines: Lines) {
        self.data.clear();
        for line in lines {
            let mut digits = vec![];
            line.chars()
                .for_each(|ch| digits.push(ch.to_digit(10).unwrap() as usize));
            self.data.push(digits);
        }
        self.rows = self.data.len();
        self.columns = self.data.first().unwrap().len();
    }

    fn init_visible_map(&mut self) {
        self.visible_map.clear();
        for (i, row) in self.data.iter().enumerate() {
            let rowm = self.visible_map.entry(i).or_insert(HashMap::<_, _>::new());
            row.iter().enumerate().for_each(|(i, _)| {
                rowm.insert(i, Visible::new());
            });
        }
    }

    fn build_visible_map(&mut self) {
        self.init_visible_map();

        for i in 0..self.rows {
            let mut premax = self.data[i][0];
            for j in 0..self.columns {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                if j == 0 {
                    v.left = true;
                    continue;
                }
                if self.data[i][j] > premax {
                    v.left = true;
                    premax = self.data[i][j];
                }
            }
        }
        for i in 0..self.rows {
            let mut premax = self.data[i][self.columns - 1];
            for j in (0..self.columns).rev() {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                if j == self.columns - 1 {
                    v.right = true;
                    continue;
                }
                if self.data[i][j] > premax {
                    v.right = true;
                    premax = self.data[i][j];
                }
            }
        }

        for j in 0..self.columns {
            let mut premax = self.data[0][j];
            for i in 0..self.rows {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                if i == 0 {
                    v.up = true;
                    continue;
                }
                if self.data[i][j] > premax {
                    v.right = true;
                    premax = self.data[i][j];
                }
            }
        }
        for j in 0..self.columns {
            let mut premax = self.data[self.columns - 1][j];
            for i in (0..self.rows).rev() {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                if i == self.rows - 1 {
                    v.down = true;
                    continue;
                }
                if self.data[i][j] > premax {
                    v.down = true;
                    premax = self.data[i][j];
                }
            }
        }
    }

    fn visible_trees_count(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.rows {
            for j in 0..self.columns {
                if get_visible_obj(&self.visible_map, i, j).visible() {
                    count += 1;
                }
            }
        }

        count
    }
}

pub struct Day8Part1 {}

impl Solution for Day8Part1 {
    fn run(&self, input: &str) -> String {
        let grid = Grid::new(input.lines());
        grid.visible_trees_count().to_string()
    }
}
