use regex::Regex;

use crate::utils::{get_input, InputType};
use std::{collections::HashSet, io};

pub fn main() -> io::Result<()> {
    parse_folds(&get_input(2021, 13, InputType::Challenge, 0)?);
    Ok(())
}

fn parse_folds(input: &str) {
    let mut dots: HashSet<(u32, u32)> = HashSet::new();
    let mut fold_count = 0;
    let fold_re = Regex::new(r"([xy])=(\d+)").unwrap();
    // input
    for line in input.lines() {
        if line.contains(",") {
            add_dots_from_line(line, &mut dots);
        } else if line.contains("fold along") {
            if let Some(instructions) = fold_re.captures(line) {
                if let Ok(fold_line) = instructions[2].parse::<u32>() {
                    let delta = match &instructions[1] {
                        "x" => (fold_line, 0),
                        "y" => (0, fold_line),
                        _ => panic!("Unable to parse fold line from: {:?}", instructions),
                    };

                    fold_dots(&mut dots, delta);
                    fold_count += 1;
                    println!("length after fold {}: {}", fold_count, dots.len());

                    continue;
                }
            }
            panic!("Unable to parse fold instruction: {}", line);
        }
    }
    print_dots(dots);
}

fn add_dots_from_line(line: &str, dots: &mut HashSet<(u32, u32)>) {
    let mut coord_string = line.split(",");
    if let (Some(x_str), Some(y_str)) = (coord_string.next(), coord_string.next()) {
        match (x_str.parse::<u32>(), y_str.parse::<u32>()) {
            (Ok(x), Ok(y)) => {
                dots.insert((x, y));
            }
            _ => panic!("Unable to parse: {} and {} into u32", x_str, y_str),
        }
    } else {
        panic!("Unable to parse coord string from: {}", line);
    }
}

fn fold_dots(dots: &mut HashSet<(u32, u32)>, delta: (u32, u32)) {
    let mut points_to_move = dots.clone();
    let (dx, dy) = delta;
    points_to_move.retain(|point| point.0 >= dx && point.1 >= dy);
    dots.retain(|point| !(point.0 >= dx && point.1 >= dy));

    points_to_move.iter().for_each(|(x, y)| {
        // println!("moving point ({}, {})", x, y);
        let new_x = if dx > 0 { dx - (x - dx) } else { *x };
        let new_y = if dy > 0 { dy - (y - dy) } else { *y };

        dots.insert((new_x, new_y));
    });
}

fn print_dots(dots: HashSet<(u32, u32)>) {
    let (max_x, max_y) = dots.iter().fold((0, 0), |a, b| {
        (
            if a.0 > b.0 { a.0 } else { b.0 },
            if a.1 > b.1 { a.1 } else { b.1 },
        )
    });

    println!();
    for y in 0..max_y + 1 {
        let mut row = String::new();
        for x in 0..max_x + 1 {
            row.push(if dots.contains(&(x, y)) { '#' } else { ' ' });
        }
        println!("{}", row);
    }
}
