use std::fs::File;
use std::io::{BufRead, BufReader};

fn card_results(card: &[[u32; 5]; 5], drawn_numbers: &Vec<u32>, mut card_score: u32) -> (u32, u32) {
    let mut current_round: u32 = 1;
    let mut marks = [[false; 5]; 5];

    for drawn_number in drawn_numbers {
        for i in 0..card.len() {
            for j in 0..card[i].len() {
                if card[i][j] == *drawn_number {
                    marks[i][j] = true;
                    card_score -= card[i][j];
                }
            }
        }

        let mut bingo_on_row = false;
        let mut bingo_on_col = false;
        if current_round > 4 {
            let mut i = 0;
            while !(bingo_on_row || bingo_on_col) && i < card.len() {
                bingo_on_row = true;
                bingo_on_col = true;
                for j in 0..card[i].len() {
                    bingo_on_row &= marks[i][j];
                    bingo_on_col &= marks[j][i];
                }
                i += 1;
            }
        }
        if bingo_on_row || bingo_on_col {
            return (current_round, card_score * drawn_number);
        }

        current_round += 1;
    }

    (current_round, 0)
}

fn main() {
    let file = File::open("bingo.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut drawn_numbers = vec![];
    let first_line = lines.next().unwrap().unwrap();
    for number in first_line.split(',') {
        drawn_numbers.push(number.parse::<u32>().unwrap());
    }
    lines.next();

    let mut first_winner = (u32::MAX, 0); // (winning round, score)
    let mut last_winner = (0, 0);

    let mut row = 0;
    let mut current_card = [[0u32; 5]; 5];
    let mut unmarked_card_score = 0;

    for line in lines {
        let card_line = line.unwrap();

        if card_line != "" {
            let mut col = 0;

            for number in card_line.split_whitespace() {
                current_card[row][col] = number.parse::<u32>().unwrap();
                unmarked_card_score += current_card[row][col];
                col += 1;
            }
            row += 1;

            if row == 5 {
                let (round, score) = card_results(&current_card, &drawn_numbers, unmarked_card_score);

                if round < first_winner.0 {
                    first_winner = (round, score);
                }
                if round > last_winner.0 {
                    last_winner = (round, score);
                }

                unmarked_card_score = 0;
                row = 0;
            }
        }
    }

    println!("Part One: {}\nPart Two: {}", first_winner.1, last_winner.1);
}
