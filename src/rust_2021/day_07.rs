use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 7, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 7, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn calc_changes(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .fold(0, |acc, cur| acc + (cur - target).abs())
}

fn calc_growing_changes(positions: &Vec<i32>, target: i32) -> i32 {
    positions.iter().fold(0, |acc, cur| {
        let steps = (cur - target).abs();
        acc + steps * (steps + 1) / 2
    })
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|elt| elt.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part_one(input: &str) -> i32 {
    let crabs = parse_input(input);
    let (upper, lower) = (*crabs.iter().max().unwrap(), *crabs.iter().min().unwrap());
    let possible_changes = (lower..upper).map(|target| calc_changes(&crabs, target));
    if let Some(min_moves) = possible_changes.min() {
        return min_moves;
    }
    0
}

fn part_two(input: &str) -> i32 {
    let crabs = parse_input(input);
    let (upper, lower) = (*crabs.iter().max().unwrap(), *crabs.iter().min().unwrap());
    let possible_changes = (lower..upper).map(|target| calc_growing_changes(&crabs, target));
    if let Some(min_moves) = possible_changes.min() {
        return min_moves;
    }
    0
}
