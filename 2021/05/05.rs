use std::fs::File;
use std::io::{BufRead, BufReader};

const PART_B: bool = true;

struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn from_coordinates(coordinates: &mut std::str::Split<'_, char>) -> Point {
        let x = coordinates.next().unwrap().parse::<i16>().unwrap();
        let y = coordinates.next().unwrap().parse::<i16>().unwrap();

        Point { x, y }
    }
}

struct Line {
    start: Point,
    end: Point,
}

fn overlaps_on_line(line: &Line, plane: &mut [[i16; 1000]; 1000]) -> u32 {
    let mut new_overlaps = 0;
    let x_step: i16 = if (line.end.x - line.start.x) > 0 {
        1
    } else if (line.end.x - line.start.x) < 0 {
        -1
    } else {
        0
    };
    let y_step: i16 = if (line.end.y - line.start.y) > 0 {
        1
    } else if (line.end.y - line.start.y) < 0 {
        -1
    } else {
        0
    };
    let iterations: usize;

    if line.start.x == line.end.x {
        iterations = (line.start.y - line.end.y).abs() as usize;
    } else {
        iterations = (line.start.x - line.end.x).abs() as usize;
    }

    for i in 0..=iterations {
        let x = (line.start.x + x_step * i as i16) as usize;
        let y = (line.start.y + y_step * i as i16) as usize;

        plane[x][y] += 1;
        if plane[x][y] == 2 {
            new_overlaps += 1;
        }
    }

    new_overlaps
}

fn main() {
    let file = File::open("lines_on_plane.txt").unwrap();
    let reader = BufReader::new(file);

    let mut overlaps_per_point = [[0i16; 1000]; 1000];
    let mut overlaps: u32 = 0;

    for input_line in reader.lines() {
        let l = input_line.unwrap();
        let mut points = l.split(" -> ");

        let start = Point::from_coordinates(&mut points.next().unwrap().split(','));
        let end = Point::from_coordinates(&mut points.next().unwrap().split(','));

        if start.x == end.x
            || start.y == end.y
            || (start.x - end.x).abs() == (start.y - end.y).abs() && PART_B
        {
            overlaps += overlaps_on_line(&Line { start, end }, &mut overlaps_per_point);
        }
    }

    println!("{}", overlaps);
}
