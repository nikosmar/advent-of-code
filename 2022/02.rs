use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const WIN: usize = 2;
const DRAW: usize = 1;
const LOSE: usize = 0;
const ASCII_A: usize = 65;
const ASCII_X: usize = 88;

pub fn main() {
    let results = [
        [DRAW, LOSE, WIN], // X vs A B C
        [WIN, DRAW, LOSE], // Y vs A B C
        [LOSE, WIN, DRAW], // Z vs A B C
    ];

    let file = File::open("input_files/02_rps.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score_a: usize = 0;
    let mut score_b: usize = 0;

    for line in reader.lines() {
        let tmp = line.unwrap();

        let opp_move = tmp.chars().nth(0).unwrap() as usize - ASCII_A;
        let my_move = tmp.chars().nth(2).unwrap() as usize - ASCII_X;
        score_a += 1 + my_move + results[my_move][opp_move] * 3;

        score_b += 1 + my_move * 3;
        for i in 0..3 {
            if results[i][opp_move] == my_move {
                score_b += i;
            }
        }
    }

    println!("Part A: {}", score_a);
    println!("Part B: {}", score_b);
}
