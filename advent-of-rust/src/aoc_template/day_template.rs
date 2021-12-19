use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(0, 0, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(0, 0, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn part_one(input: &str) -> &str {
    input
}

fn part_two(input: &str) -> &str {
    input
}
