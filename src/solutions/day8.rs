use std::{collections::HashMap, str::Lines};

use crate::solution::Solution;

type VisibleMap = HashMap<usize, HashMap<usize, Visible>>;

pub struct Grid {
    data: Vec<Vec<usize>>,
    visible_map: VisibleMap,
    view_trees_map: ViewTreesMap,
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
}

struct ViewTrees {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl ViewTrees {
    fn new() -> Self {
        ViewTrees {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
        }
    }

    fn score(&self) -> usize {
        self.left * self.right * self.up * self.down
    }
}

type ViewTreesMap = HashMap<usize, HashMap<usize, ViewTrees>>;

fn count_until_taller(l: Vec<usize>, target: usize) -> usize {
    let mut count = 0;
    for i in (0..l.len()).rev() {
        count += 1;
        if l[i] >= target {
            break;
        }
    }
    count
}

impl Grid {
    fn new(data: Lines) -> Grid {
        let mut grid = Grid {
            data: Vec::<Vec<usize>>::new(),
            visible_map: HashMap::<_, _>::new(),
            view_trees_map: HashMap::<_, _>::new(),
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

    fn init_map(&mut self) {
        self.visible_map.clear();
        self.view_trees_map.clear();
        for (i, row) in self.data.iter().enumerate() {
            let row_vm = self.visible_map.entry(i).or_default();
            let row_vtm = self.view_trees_map.entry(i).or_default();
            row.iter().enumerate().for_each(|(i, _)| {
                row_vm.insert(i, Visible::new());
                row_vtm.insert(i, ViewTrees::new());
            });
        }
    }

    // View visible rule:
    //   1. edge tree is must visible
    //   2. if tree is shorter than premax, it is invisible
    //
    // Viewable pre trees count rule:
    //   1. edge tree can't see any tree from a direction, edge = 0
    //   2. when current tree is taller than previous, current = previous + 1
    //   3. if current tree is at most tall as previous, current = 1
    fn build_visible_map(&mut self) {
        self.init_map();

        // left to right
        for i in 0..self.rows {
            let first = self.data[i][0];
            let mut premax = first;
            let mut prevec = vec![first];

            for j in 0..self.columns {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                let vt = self
                    .view_trees_map
                    .get_mut(&i)
                    .unwrap()
                    .get_mut(&j)
                    .unwrap();
                if j == 0 {
                    v.left = true;
                    vt.left = 0;
                    continue;
                }
                let curheight = self.data[i][j];
                if curheight > premax {
                    v.left = true;
                    premax = self.data[i][j];
                }

                vt.left = count_until_taller(prevec.clone(), curheight);
                prevec.push(curheight);
            }
        }

        // right to left
        for i in 0..self.rows {
            let first = self.data[i][self.columns - 1];
            let mut premax = first;
            let mut prevec = vec![first];
            for j in (0..self.columns).rev() {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                let vt = self
                    .view_trees_map
                    .get_mut(&i)
                    .unwrap()
                    .get_mut(&j)
                    .unwrap();
                if j == self.columns - 1 {
                    v.right = true;
                    vt.right = 0;
                    continue;
                }
                let curheight = self.data[i][j];
                if curheight > premax {
                    v.right = true;
                    premax = curheight;
                }

                vt.right = count_until_taller(prevec.clone(), curheight);
                prevec.push(curheight);
            }
        }

        // up to down
        for j in 0..self.columns {
            let first = self.data[0][j];
            let mut premax = first;
            let mut prevec = vec![first];
            for i in 0..self.rows {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                let vt = self
                    .view_trees_map
                    .get_mut(&i)
                    .unwrap()
                    .get_mut(&j)
                    .unwrap();
                if i == 0 {
                    v.up = true;
                    vt.up = 0;
                    continue;
                }
                let curheight = self.data[i][j];
                if curheight > premax {
                    v.up = true;
                    premax = curheight;
                }

                vt.up = count_until_taller(prevec.clone(), curheight);
                prevec.push(curheight);
            }
        }

        // down to up
        for j in 0..self.columns {
            let first = self.data[self.rows - 1][j];
            let mut premax = first;
            let mut prevec = vec![first];
            for i in (0..self.rows).rev() {
                let v = self.visible_map.get_mut(&i).unwrap().get_mut(&j).unwrap();
                let vt = self
                    .view_trees_map
                    .get_mut(&i)
                    .unwrap()
                    .get_mut(&j)
                    .unwrap();
                if i == self.rows - 1 {
                    v.down = true;
                    vt.down = 0;
                    continue;
                }
                let curheight = self.data[i][j];
                if curheight > premax {
                    v.down = true;
                    premax = curheight;
                }

                vt.down = count_until_taller(prevec.clone(), curheight);
                prevec.push(curheight);
            }
        }
    }

    fn visible_trees_count(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.rows {
            for j in 0..self.columns {
                if self.visible_map.get(&i).unwrap().get(&j).unwrap().visible() {
                    count += 1;
                }
            }
        }

        count
    }

    fn highest_scenic_score(&self) -> usize {
        let mut highest = 0;
        for i in 0..self.rows {
            for j in 0..self.columns {
                let score = self
                    .view_trees_map
                    .get(&i)
                    .unwrap()
                    .get(&j)
                    .unwrap()
                    .score();
                if score > highest {
                    highest = score;
                }
            }
        }

        highest
    }
}

pub struct Day8Part1 {}

impl Solution for Day8Part1 {
    fn run(&self, input: &str) -> String {
        let grid = Grid::new(input.lines());
        grid.visible_trees_count().to_string()
    }
}

pub struct Day8Part2 {}

impl Solution for Day8Part2 {
    fn run(&self, input: &str) -> String {
        let grid = Grid::new(input.lines());
        grid.highest_scenic_score().to_string()
    }
}
