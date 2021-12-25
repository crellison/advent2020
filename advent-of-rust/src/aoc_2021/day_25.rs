use crate::utils::{get_input, InputType};
use std::{collections::HashSet, io};

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 25, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn parse_input(input: &str) -> Cucumbers {
    let (mut south, mut east) = (HashSet::new(), HashSet::new());
    let lines = input.lines().collect::<Vec<&str>>();
    let (south_max, east_max) = (lines.len(), lines[0].len());
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.char_indices() {
            match character {
                '>' => east.insert((x, y)),
                'v' => south.insert((x, y)),
                _ => false,
            };
        }
    }
    Cucumbers {
        east,
        south,
        east_max,
        south_max,
    }
}

struct Cucumbers {
    east: HashSet<(usize, usize)>,
    south: HashSet<(usize, usize)>,
    east_max: usize,
    south_max: usize,
}

impl Cucumbers {
    fn move_cucumbers(&mut self) -> bool {
        let mut moved = false;
        let mut next_east: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in &self.east {
            let next_x = if x + 1 == self.east_max { 0 } else { x + 1 };
            if !self.east.contains(&(next_x, *y)) && !self.south.contains(&(next_x, *y)) {
                moved = true;
                next_east.insert((next_x, *y));
            } else {
                next_east.insert((*x, *y));
            }
        }
        self.east = next_east;
        let mut next_south: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in &self.south {
            let next_y = if y + 1 == self.south_max { 0 } else { y + 1 };
            if !self.east.contains(&(*x, next_y)) && !self.south.contains(&(*x, next_y)) {
                moved = true;
                next_south.insert((*x, next_y));
            } else {
                next_south.insert((*x, *y));
            }
        }
        self.south = next_south;
        moved
    }
}

fn part_one(input: &str) -> u32 {
    let mut cucumbers = parse_input(input);
    let mut steps = 1;
    while cucumbers.move_cucumbers() {
        steps += 1
    }
    steps
}
