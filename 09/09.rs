use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
    value: u32,
    visited: bool,
}

fn part_one(grid: &Vec<Vec<Point>>) -> Vec<(usize, usize)> {
    let mut low_points = vec![];
    let mut risk_levels = 0;

    for x in 1..=grid.len() - 2 {
        for y in 1..=grid[0].len() - 2 {
            if grid[x][y].value < grid[x + 1][y].value
                && grid[x][y].value < grid[x - 1][y].value
                && grid[x][y].value < grid[x][y - 1].value
                && grid[x][y].value < grid[x][y + 1].value
            {
                risk_levels += 1 + grid[x][y].value;
                low_points.push((x, y));
            }
        }
    }

    println!("Part One: {}", risk_levels);
    low_points
}

fn basin_size(grid: &mut Vec<Vec<Point>>, point: &(usize, usize)) -> u32 {
    let mut size = 1;

    let x = point.0;
    let y = point.1;

    if grid[x][y].value == 9 || grid[x][y].visited {
        return 0;
    }

    grid[x][y].visited = true;

    if x > 1 {
        size += basin_size(grid, &(x - 1, y));
    }
    if y > 1 {
        size += basin_size(grid, &(x, y - 1));
    }
    if x < grid.len() - 2 {
        size += basin_size(grid, &(x + 1, y));
    }
    if y < grid[0].len() - 2 {
        size += basin_size(grid, &(x, y + 1));
    }

    size
}

fn part_two(grid: &mut Vec<Vec<Point>>, low_points: &Vec<(usize, usize)>) -> u32 {
    let mut largest_basins = [0; 4];

    for point in low_points {
        let current_basin_size = basin_size(grid, point);

        if current_basin_size > largest_basins[0] {
            largest_basins[0] = current_basin_size;
            largest_basins.sort();
        }
    }

    largest_basins[1] * largest_basins[2] * largest_basins[3]
}

fn main() {
    let file = File::open("basins.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid = vec![];

    for line in reader.lines() {
        let row = line.unwrap();
        let mut current_row = vec![Point {
            value: 9,
            visited: false,
        }];

        for point in row.chars() {
            current_row.push(Point {
                value: point.to_digit(10).unwrap(),
                visited: false,
            });
        }

        current_row.push(Point {
            value: 9,
            visited: false,
        });
        grid.push(current_row);
    }

    let mut foo = vec![];
    let mut bar = vec![];
    for _ in 0..grid[0].len() {
        foo.push(Point {
            value: 9,
            visited: false,
        });
        bar.push(Point {
            value: 9,
            visited: false,
        });
    }
    grid.insert(0, foo);
    grid.push(bar);

    let low_points = part_one(&grid);
    println!("Part Two: {}", part_two(&mut grid, &low_points));
}
