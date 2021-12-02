use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("commands.txt").unwrap();
    let reader = BufReader::new(file);

    let mut depth: i32 = 0;
    let mut hor: i32 = 0;
    let mut aim: i32 = 0;

    for line in reader.lines() {
        let x = line.unwrap();
        let mut command = x.split_whitespace();

        let opcode = command.next().unwrap();
        let steps = command.next().unwrap().parse::<i32>().unwrap();

        match opcode {
            "forward" => {
                hor += steps;
                depth += aim * steps;
            }
            "up" => aim -= steps,
            "down" => aim += steps,
            _ => (),
        }
    }

    println!("{}", hor * depth);
}
