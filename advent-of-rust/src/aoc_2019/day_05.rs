use super::intcode::run_intcode;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2019, 5, InputType::Challenge, 0)?),
            16574641
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2019, 5, InputType::Challenge, 0)?),
            15163975
        );
        Ok(())
    }

    // #[test]
    // fn test_part_two_ex() -> io::Result<()> {
    //     assert_eq!(
    //         part_two(&get_input(2019, 5, InputType::Example, 1)?),
    //         0
    //     );
    //     Ok(())
    // }
}
