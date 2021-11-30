#[allow(dead_code)]
fn part_one(input: &str) -> &str {
    input
    // for line in input.lines() {
    //   print!("{}", line)
    // }
}

#[allow(dead_code)]
fn part_two(input: &str) -> &str {
    input
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 13, InputType::Challenge, 0)?),
            "test\n"
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 13, InputType::Challenge, 0)?),
            "test\n"
        );
        Ok(())
    }
}
