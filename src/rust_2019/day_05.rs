use super::intcode::run_intcode;
use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2019, 5, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2019, 5, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn part_one(input: &str) -> i32 {
    let first_line = input.lines().next().unwrap();
    let commands: Vec<i32> = first_line
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let output = run_intcode(commands, [1].to_vec());
    print!("{:?}", output);
    *output.last().unwrap()
}

fn part_two(input: &str) -> i32 {
    let first_line = input.lines().next().unwrap();
    let commands: Vec<i32> = first_line
        .split(',')
        .map(|x| {
            let c = x.parse::<i32>();
            if c.is_err() {
                println!("bad int: {}", x);
                panic!();
            }
            c.unwrap()
        })
        .collect();

    let output = run_intcode(commands, [5].to_vec());
    print!("{:?}", output);
    *output.last().unwrap()
}
