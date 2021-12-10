#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 1, InputType::Challenge, 0)?),
            1527
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 1, InputType::Challenge, 0)?),
            1575
        );
        Ok(())
    }
}
