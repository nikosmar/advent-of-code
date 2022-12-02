use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("navigation.txt").unwrap();
    let reader = BufReader::new(file);

    let opening = HashMap::from([
        ('(', (3, 1)),
        ('[', (57, 2)),
        ('{', (1197, 3)),
        ('<', (25137, 4)),
    ]);
    let closing = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut part_one = 0;
    let mut part_two = vec![];

    for line in reader.lines() {
        let tmp = line.unwrap();
        let mut chunk: Vec<char> = vec![];
        let mut corrupted = false;

        for pattern in tmp.chars() {
            if opening.contains_key(&pattern) {
                chunk.push(pattern);
            } else if chunk.len() == 0 || opening[&chunk.pop().unwrap()].0 != closing[&pattern] {
                part_one += closing[&pattern];
                corrupted = true;
                break;
            }
        }

        if !corrupted {
            let mut completion_score: u128 = 0;
            while !chunk.is_empty() {
                completion_score = completion_score * 5 + opening[&chunk.pop().unwrap()].1;
            }
            part_two.push(completion_score);
        }
    }

    part_two.sort_unstable();
    println!(
        "Part One: {}\nPart Two: {}",
        part_one,
        part_two[part_two.len() / 2]
    );
}
