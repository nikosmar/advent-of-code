use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn solve_part_b(card: usize, cards: &mut Vec<(u32, u32)>) -> u32 {
    let mut total = 1;

    if cards[card].0 == 0 {
        return 1;
    }

    if cards[card].1 > 0 {
        return cards[card].1;
    }

    for next_card in (card + 1)..=(card + cards[card].0 as usize) {
        total += solve_part_b(next_card, cards)
    }

    cards[card].1 = total;

    total
}

fn main() {
    let file = File::open("../input_files/04_scratchcards.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_points = 0;
    let mut cards: Vec<(u32, u32)> = vec![];
    let mut total_cards = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut wins = 0;

        let card_str: Vec<_> = line_str.split(':').collect();
        let numbers: Vec<_> = card_str[1].split('|').collect();

        let winning_numbers: HashSet<u32> = numbers[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let playing_numbers: Vec<u32> = numbers[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for num in playing_numbers {
            if winning_numbers.contains(&num) {
                wins += 1;
            }
        }

        if wins > 0 {
            total_points += 1 << (wins - 1);
        }

        cards.push((wins, 0));
    }

    for i in 0..cards.len() {
        total_cards += solve_part_b(i, &mut cards);
    }

    println!("Part A: {}", total_points);
    println!("Part B: {}", total_cards);
}
