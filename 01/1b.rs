use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("depth.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    const WINDOW_SIZE: usize = 3;

    let mut last_three: [u32; WINDOW_SIZE] = [0; WINDOW_SIZE];
    let mut array_index = 0;
    let mut depth_increases = 0;
    let mut previous_depth = 0;

    for i in 0..WINDOW_SIZE {
        let line = lines.next();
        last_three[i] = line.unwrap().unwrap().parse::<u32>().unwrap();
        previous_depth += last_three[i];
    }

    for line in lines {
        let depth = line.unwrap().parse::<u32>().unwrap();
        let mut window_depth = 0;

        last_three[array_index] = depth;
        array_index = (array_index + 1) % WINDOW_SIZE;

        for x in last_three {
            window_depth += x;
        }

        if window_depth > previous_depth {
            depth_increases += 1;
        }

        previous_depth = window_depth;
    }

    println!("{}", depth_increases);
}