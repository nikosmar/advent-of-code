use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(input: &Vec<Vec<i32>>) -> Result<(), std::io::Error> {
    let mut part_a: u32 = 0;
    let mut part_b: usize = input.len();

    for pair in input {
        if (pair[0] - pair[2]) * (pair[1] - pair[3]) <= 0 {
            part_a += 1;
        }

        if pair[3] < pair[0] || pair[1] < pair[2] {
            part_b -= 1;
        }
    }

    println!("Part A: {}", part_a);
    println!("Part B: {}", part_b);

    Ok(())
}

fn main() {
    let mut input = Vec::new();
    let re = Regex::new(r"[0-9]+").unwrap();

    let file = File::open("../input_files/04_cleanup.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let x = re
            .find_iter(line.unwrap().as_str())
            .map(|mat| mat.as_str().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push(x);
    }

    solve(&input);
}
