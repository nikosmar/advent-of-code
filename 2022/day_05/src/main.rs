use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Puzzle {
    stacks: Vec<Vec<char>>,
}

impl Puzzle {
    fn new(amount_of_stacks: usize) -> Self {
        Puzzle {
            stacks: vec![Vec::<char>::new(); amount_of_stacks],
        }
    }

    fn add_crate(&mut self, stack: usize, crate_name: char) {
        self.stacks[stack].insert(0, crate_name);
    }

    fn move_crates_one_by_one(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            if let Some(crt) = self.stacks[from].pop() {
                self.stacks[to].push(crt);
            }
        }
    }

    fn move_crates_batch(&mut self, amount: usize, from: usize, to: usize) {
        let ind = self.stacks[to].len();
        for _ in 0..amount {
            if let Some(crt) = self.stacks[from].pop() {
                self.stacks[to].insert(ind, crt);
            }
        }
    }

    fn print_skyline(&self) {
        for stack in &self.stacks {
            if let Some(top) = stack.last() {
                print!("{}", top);
            }
        }
        println!();
    }
}

fn main() {
    let re_numbers = Regex::new(r"[0-9]+").unwrap();
    let re_crates = Regex::new(r"[A-Z]").unwrap();

    let file = File::open("../input_files/05_crates.txt").unwrap();
    let reader = BufReader::new(file);

    let mut reading_graph = true;
    let mut part_a = Puzzle::new(9);
    let mut part_b = Puzzle::new(9);

    for line in reader.lines() {
        let tmp_line = line.unwrap();
        if tmp_line.is_empty() {
            reading_graph = false;
            continue;
        }

        if reading_graph {
            for mat in re_crates.find_iter(&tmp_line) {
                let stack_index: usize = (mat.start() - 1) / 4;
                let crate_name = mat.as_str().chars().next().unwrap();

                part_a.add_crate(stack_index, crate_name);
                part_b.add_crate(stack_index, crate_name);
            }
        } else {
            let mv = re_numbers
                .find_iter(tmp_line.as_str())
                .map(|mat| mat.as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            part_a.move_crates_one_by_one(mv[0], mv[1] - 1, mv[2] - 1);
            part_b.move_crates_batch(mv[0], mv[1] - 1, mv[2] - 1);
        }
    }

    part_a.print_skyline();
    part_b.print_skyline();
}
