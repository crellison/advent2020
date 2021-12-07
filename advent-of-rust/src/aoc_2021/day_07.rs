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

#[allow(dead_code)]
fn part_one(input: &str) -> i32 {
    let crabs = parse_input(input);
    let (upper, lower) = (*crabs.iter().max().unwrap(), *crabs.iter().min().unwrap());
    let possible_changes = (lower..upper).map(|target| calc_changes(&crabs, target));
    if let Some(min_moves) = possible_changes.min() {
        return min_moves;
    }
    0
}

#[allow(dead_code)]
fn part_two(input: &str) -> i32 {
    let crabs = parse_input(input);
    let (upper, lower) = (*crabs.iter().max().unwrap(), *crabs.iter().min().unwrap());
    let possible_changes = (lower..upper).map(|target| calc_growing_changes(&crabs, target));
    if let Some(min_moves) = possible_changes.min() {
        return min_moves;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{calc_changes, part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_calc_changes() -> io::Result<()> {
        let test_vec = vec![16, 5, 4, 1];
        assert_eq!(calc_changes(&test_vec, 3), 18);
        assert_eq!(calc_changes(&test_vec, 4), 16);
        assert_eq!(calc_changes(&test_vec, 5), 16);
        assert_eq!(calc_changes(&test_vec, 6), 18);
        Ok(())
    }

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 7, InputType::Challenge, 0)?),
            336040
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 7, InputType::Challenge, 0)?),
            94813675
        );
        Ok(())
    }
}
