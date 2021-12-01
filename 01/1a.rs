use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("depth.txt").unwrap();
    let reader = BufReader::new(file);

    let mut depth_increases = -1;
    let mut previous_depth = 0;

    for line in reader.lines() {
        let depth = line.unwrap().parse::<u32>().unwrap();
        
        if depth > previous_depth {
            depth_increases += 1;
        }

        previous_depth = depth;
    }

    println!("{}", depth_increases);
}