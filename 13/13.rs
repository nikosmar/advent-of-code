use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn point_from_coordinates(coordinates: &mut std::str::Split<'_, char>) -> (i16, i16) {
    let x = coordinates.next().unwrap().parse::<i16>().unwrap();
    let y = coordinates.next().unwrap().parse::<i16>().unwrap();

    (x, y)
}

fn part_one(grid: &mut HashSet<(i16, i16)>, folds: &Vec<(char, i16)>, points: &Vec<(i16, i16)>) -> u32 {
    let mut marks = 0;

    for point in points {
        let folded_point ;

        if folds[0].0 == 'x' {
            folded_point = (folds[0].1 - (folds[0].1 - point.0).abs(), point.1);
        } else {
            folded_point = (point.0, folds[0].1 - (folds[0].1 - point.1).abs());
        }

        if !grid.contains(&folded_point) {
            marks += 1;
            grid.insert(folded_point);
        }
    }

    marks
}

fn part_two(grid: &HashSet<(i16, i16)>, folds: &Vec<(char, i16)>) {
    // calculate final grid dimensions using the last vertical/horizontal folds
    let mut row = 0;
    let mut col = 0;
    for fold in folds {
        if fold.0 == 'x' {
            col = fold.1;
        } else {
            row = fold.1;
        }
    }
    let mut final_grid = vec![vec!['.'; col as usize]; row as usize];

    for point in grid {
        let mut x = point.0;
        let mut y = point.1;
        for fold in folds.iter().skip(1) {
            if fold.0 == 'x' {
                x = fold.1 - (fold.1 - x).abs();
            } else {
                y = fold.1 - (fold.1 - y).abs();
            }
        }

        final_grid[y as usize][x as usize] = '#';
    }

    for row in final_grid {
        println!("{:?}", row);
    }
}

fn main() {
    let file = File::open("folds.txt").unwrap();
    let reader = BufReader::new(file);

    let mut folds = vec![];
    let mut points = vec![];
    let mut push_points = true;
    let mut folded_grid = HashSet::new();

    for line in reader.lines() {
        let l = line.unwrap();

        if l != "" && push_points {
            points.push(point_from_coordinates(&mut l.split(',')));
        } else if l != "" {
            let mut fold_split = l.split("=");

            let axis_to_fold = fold_split.next().unwrap();
            let fold_on_line = fold_split.next().unwrap().parse::<i16>().unwrap();

            let axis = axis_to_fold.chars().rev().next().unwrap();

            folds.push((axis, fold_on_line));
        } else {
            push_points = false;
        }
    }
    
    println!("Part One: {}", part_one(&mut folded_grid, &folds, &points));
    part_two(&folded_grid, &folds);
}
