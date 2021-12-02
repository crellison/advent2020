#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_examples() -> io::Result<()> {
        assert_eq!(part_one(&get_input(2021, 2, InputType::Example, 0)?), 150);
        assert_eq!(part_two(&get_input(2021, 2, InputType::Example, 0)?), 900);
        Ok(())
    }

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 2, InputType::Challenge, 0)?),
            1762050
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 2, InputType::Challenge, 0)?),
            1855892637
        );
        Ok(())
    }
}
