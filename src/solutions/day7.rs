use std::{collections::HashMap, fmt::Display, str::Lines};

use crate::solution::Solution;

struct TrieNode {
    val: i32,
    childs: HashMap<String, Box<TrieNode>>,
    is_dir: bool,
}

impl TrieNode {
    fn new(val: i32, is_dir: bool) -> TrieNode {
        TrieNode {
            val,
            childs: HashMap::<String, Box<TrieNode>>::new(),
            is_dir,
        }
    }

    fn push(&mut self, paths: &[String], val: i32) -> i32 {
        if paths.len() == 1 {
            let k = paths[0].to_string();
            match self.childs.get(&k) {
                Some(_) => {
                    return 0;
                }
                None => {
                    self.childs.insert(k, Box::new(TrieNode::new(val, false)));
                    self.val += val;
                    return val;
                }
            }
        }

        let k = paths[0].to_string();
        let child = self
            .childs
            .entry(k)
            .or_insert(Box::new(TrieNode::new(0, true)));
        let increment = child.push(&paths[1..], val);
        self.val += increment;

        increment
    }

    fn fmt_r(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        write!(f, ":{}\n", self.val).unwrap();
        // if level > 1 {
        //     return Ok(());
        // }
        for (k, v) in self.childs.iter() {
            write!(f, "{}{}", "-".repeat(level), k).unwrap();
            v.fmt_r(f, level + 1).unwrap();
        }
        Ok(())
    }

    fn sum_less_than_size(&self, limit: i32) -> i32 {
        let mut s = 0;
        for child in self.childs.values().filter(|x| x.is_dir) {
            s += child.sum_less_than_size(limit);
        }

        if self.val <= limit {
            return s + self.val;
        }

        s
    }
}

impl Display for TrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_r(f, 0)
    }
}

struct FileBrowser {
    files: HashMap<String, i32>,
    dir_stack: Vec<String>,
    tree: Box<TrieNode>,
}

impl FileBrowser {
    fn new() -> FileBrowser {
        FileBrowser {
            files: HashMap::<String, i32>::new(),
            dir_stack: vec![],
            tree: Box::new(TrieNode::new(0, true)),
        }
    }

    fn get_cwd(&self) -> String {
        let cwd = self.dir_stack.join("/");
        let cwd = format!("/{}", cwd);
        cwd.to_string()
    }

    fn cd(&mut self, path: &str) {
        if path == "/" {
            self.dir_stack.clear();
            return;
        }

        if path == ".." {
            self.dir_stack.pop();
            return;
        }

        self.dir_stack.push(path.to_string());
    }

    fn list(&mut self, lines: &mut Vec<String>) {
        for line in lines.iter() {
            if line.starts_with("dir") {
                continue;
            }

            let mut splits = line.split_whitespace().into_iter();
            let filesize = splits.next().unwrap().parse::<i32>().unwrap();
            let filename = splits.next().unwrap();

            self.files
                .insert(self.get_cwd() + "/" + &filename.to_string(), filesize);
            let mut x = self.dir_stack.clone();
            x.push(filename.to_string());

            self.tree.push(&x, filesize);
        }

        lines.clear();
    }

    fn sum_less_than_size(&self, limit: i32) -> i32 {
        self.tree.sum_less_than_size(limit)
    }
}

enum Command {
    CD(String),
    LS,
    OUTPUT(String),
}

fn command(line: &str) -> Command {
    let real = line.trim_start_matches("$").trim();
    if real.starts_with("cd") {
        let splits = real.split_whitespace();
        return Command::CD(splits.last().unwrap().to_string());
    }

    if real.starts_with("ls") {
        return Command::LS;
    }

    Command::OUTPUT(line.to_string())
}

fn decode(lines: Lines) -> i32 {
    let mut fb = FileBrowser::new();

    let mut temp: Vec<String> = vec![];
    for line in lines {
        match command(line) {
            Command::CD(path) => {
                fb.list(&mut temp);
                fb.cd(&path);
            }
            Command::LS => {
                fb.list(&mut temp);
            }
            Command::OUTPUT(x) => {
                temp.push(x);
            }
        }
    }
    fb.list(&mut temp);

    fb.sum_less_than_size(100000)
}

pub struct Day7Part1 {}

impl Solution for Day7Part1 {
    fn run(&self, input: &str) -> String {
        decode(input.lines()).to_string()
    }
}
