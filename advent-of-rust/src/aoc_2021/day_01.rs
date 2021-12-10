use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 1, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 1, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn part_one(input: &str) -> u16 {
    let mut increases: u16 = 0;
    let mut last_depth = u16::MAX;
    for line in input.lines() {
        if let Ok(next_depth) = line.parse::<u16>() {
            if next_depth > last_depth {
                increases += 1;
            }
            last_depth = next_depth;
        }
    }
    increases
}

fn part_two(input: &str) -> u16 {
    let mut increases: u16 = 0;

    let mut last_three = (u16::MAX, u16::MAX, u16::MAX);
    let mut last_group_sum = u16::MAX;
    for line in input.lines() {
        if let Ok(next_depth) = line.parse::<u16>() {
            let next_group_sum: u16;
            if last_three.1 == u16::MAX || last_three.2 == u16::MAX {
                next_group_sum = u16::MAX;
            } else {
                next_group_sum = last_three.1 + last_three.2 + next_depth;
            }
            if next_group_sum > last_group_sum {
                increases += 1;
            }
            last_three = (last_three.1, last_three.2, next_depth);
            last_group_sum = next_group_sum;
        }
    }
    increases
}
