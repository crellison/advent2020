use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 2, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 2, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn part_one(input: &str) -> i32 {
    let (mut horizonal, mut depth) = (0, 0);
    for line in input.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let dir = tokens[0];

        if let Ok(quant) = tokens[1].parse::<i32>() {
            match dir {
                "forward" => horizonal += quant,
                "down" => depth += quant,
                "up" => depth -= quant,
                _ => panic!("Unexpected direction: {}", quant),
            }
        }
    }
    horizonal * depth
}

fn part_two(input: &str) -> i32 {
    let (mut horizonal, mut depth, mut aim) = (0, 0, 0);
    for line in input.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let dir = tokens[0];
        if let Ok(quant) = tokens[1].parse::<i32>() {
            match dir {
                "forward" => {
                    horizonal += quant;
                    depth += aim * quant;
                }
                "down" => aim += quant,
                "up" => aim -= quant,
                _ => panic!("Unexpected direction: {}", quant),
            }
        }
    }
    horizonal * depth
}
