use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

struct Number {
    row: i32,
    col_start: i32,
    value: u32,
}

impl Number {
    fn new(row: i32, col_start: i32, value: u32) -> Number {
        Number {
            row,
            col_start,
            value,
        }
    }

    fn append_digit(&mut self, digit: u32) {
        self.value = self.value * 10 + digit;
    }
}

fn solve_part_a(
    symbols: &HashMap<(i32, i32), char>,
    numbers: &HashMap<(i32, i32), Rc<RefCell<Number>>>,
) -> u32 {
    let mut part_numbers_sum = 0;
    let mut adjacent_numbers: HashMap<(i32, i32), Rc<RefCell<Number>>> = HashMap::new();

    for (pos_x, pos_y) in symbols.keys() {
        for i in -1..=1 {
            for j in -1..=1 {
                if let Some(number) = numbers.get(&(pos_x + i, pos_y + j)) {
                    adjacent_numbers.insert(
                        (number.borrow().row, number.borrow().col_start),
                        number.clone(),
                    );
                }
            }
        }

        for number in adjacent_numbers.values() {
            part_numbers_sum += number.borrow().value;
        }

        adjacent_numbers.clear();
    }

    part_numbers_sum
}

fn solve_part_b(
    symbols: &HashMap<(i32, i32), char>,
    numbers: &HashMap<(i32, i32), Rc<RefCell<Number>>>,
) -> u32 {
    let mut gear_ratios_sum = 0;
    let mut adjacent_numbers: HashMap<(i32, i32), Rc<RefCell<Number>>> = HashMap::new();

    for (position, symbol) in symbols.iter() {
        if *symbol != '*' {
            continue;
        }

        for i in -1..=1 {
            for j in -1..=1 {
                if let Some(number) = numbers.get(&(position.0 + i, position.1 + j)) {
                    adjacent_numbers.insert(
                        (number.borrow().row, number.borrow().col_start),
                        number.clone(),
                    );
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            let mut gear_ratio = 1;
            for number in adjacent_numbers.values() {
                gear_ratio *= number.borrow().value;
            }
            gear_ratios_sum += gear_ratio;
        }

        adjacent_numbers.clear();
    }

    gear_ratios_sum
}

fn main() {
    let file = File::open("../input_files/03_engine.txt").unwrap();
    let reader = BufReader::new(file);

    let mut part_numbers: HashMap<(i32, i32), Rc<RefCell<Number>>> = HashMap::new();
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();

    let mut x: i32;
    let mut y: i32 = 0;
    let mut current_number: Option<Rc<RefCell<Number>>> = None;

    for line in reader.lines() {
        x = 0;
        let line_str = line.unwrap();

        for point in line_str.chars() {
            match point {
                '.' => {
                    current_number = None;
                }
                '0'..='9' => {
                    let digit = point.to_digit(10).unwrap();

                    match current_number {
                        Some(ref number) => {
                            number.borrow_mut().append_digit(digit);
                            part_numbers.insert((x, y), number.clone());
                        }
                        None => {
                            let new_number = Rc::new(RefCell::new(Number::new(y, x, digit)));
                            part_numbers.insert((x, y), new_number.clone());
                            current_number = Some(new_number.clone());
                        }
                    }
                }
                _ => {
                    current_number = None;
                    symbols.insert((x, y), point);
                }
            }

            x += 1;
        }

        current_number = None;
        y += 1;
    }

    println!("Part A: {}", solve_part_a(&symbols, &part_numbers));
    println!("Part B: {}", solve_part_b(&symbols, &part_numbers));
}
