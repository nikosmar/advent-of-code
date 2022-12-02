use std::fs::File;
use std::io::{BufRead, BufReader};

fn flash(grid: &mut Vec<Vec<u32>>, row: usize, col: usize) -> i32 {
    let mut new_flashes = 0;
    let mut rows_to_check: Vec<usize> = vec![];
    let mut cols_to_check: Vec<usize> = vec![];

    match row.checked_sub(1) {
        Some(valid_row) => rows_to_check.push(valid_row),
        None => (),
    }
    match col.checked_sub(1) {
        Some(valid_col) => cols_to_check.push(valid_col),
        None => (),
    }

    for i in 0..2 {
        if row + i < grid.len() {
            rows_to_check.push(row + i);
        }
        if col + i < grid[0].len() {
            cols_to_check.push(col + i);
        }
    }

    for neigh_row in rows_to_check {
        for neigh_col in &cols_to_check {
            grid[neigh_row][*neigh_col] += 1;
            if grid[neigh_row][*neigh_col] == 10 {
                new_flashes += flash(grid, neigh_row, *neigh_col) + 1;
            }
        }
    }

    new_flashes
}

fn main() {
    let file = File::open("octopuses.txt").unwrap();
    let reader = BufReader::new(file);

    let mut flashes = 0;
    let mut grid = vec![];

    for line in reader.lines() {
        let row = line.unwrap();
        let mut current_row = vec![];
        for octopus in row.chars() {
            current_row.push(octopus.to_digit(10).unwrap());
        }
        grid.push(current_row);
    }

    let grid_rows = grid.len();
    let grid_cols = grid[0].len();
    let grid_size = (grid_rows * grid_cols) as i32;
    let mut octopuses_reset = grid_size;
    let mut step = 0;

    while octopuses_reset > 0 {
        // step a - increase energy levels
        for row in 0..grid_rows {
            for col in 0..grid_cols {
                grid[row][col] += 1;
                // step b - flash
                if grid[row][col] == 10 {
                    flashes += flash(&mut grid, row, col) + 1;
                }
            }
        }

        // step c - reset high energy levels
        octopuses_reset = grid_size;
        for row in 0..grid_rows {
            for col in 0..grid_cols {
                if grid[row][col] > 9 {
                    grid[row][col] = 0;
                    octopuses_reset -= 1;
                }
            }
        }

        step += 1;
        if step == 100 {
            println!("Part One: {}", flashes);
        }
    }

    println!("Part Two: {}", step);
}
