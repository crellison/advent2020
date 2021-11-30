#[allow(dead_code)]
fn part_one(input: &str) -> u32 {
    let first_line = input.lines().next().unwrap();
    let mut commands: Vec<u32> = first_line
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // 1202 program alarm
    commands[1] = 12;
    commands[2] = 2;

    run_intcode(commands)
}

fn run_intcode(mut commands: Vec<u32>) -> u32 {
    let mut index = 0;
    while index < commands.len() {
        match commands[index] {
            1 => {
                let (i1, i2, i3) = (
                    commands[index + 1],
                    commands[index + 2],
                    commands[index + 3],
                );
                let sum = commands[i1 as usize] + commands[i2 as usize];
                commands[i3 as usize] = sum;
                index += 4;
            }
            2 => {
                let (i1, i2, i3) = (
                    commands[index + 1],
                    commands[index + 2],
                    commands[index + 3],
                );
                let sum = commands[i1 as usize] * commands[i2 as usize];
                commands[i3 as usize] = sum;
                index += 4;
            }
            99 => {
                index = commands.len();
            }
            _ => panic!("unexpected value! {}", commands[index]),
        }
    }
    commands[0]
}

#[allow(dead_code)]
fn part_two(input: &str) -> u32 {
    let first_line = input.lines().next().unwrap();
    let commands: Vec<u32> = first_line
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    for i in 1..99 {
        for j in 1..99 {
            let mut test_commands = commands.to_vec();
            test_commands[1] = i;
            test_commands[2] = j;
            let output = run_intcode(test_commands);
            if output == 19690720 {
                print!("i = {}; j = {}\n", i, j);
                return 100 * i + j;
            }
        }
    }
    panic!("No valid combination found!")
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    // examples only run with 1202 alarm removed

    // #[test]
    // fn test_part_one_ex() -> io::Result<()> {
    //     assert_eq!(part_one(&get_input(2019, 2, InputType::Example, 0)?), 3500);
    //     assert_eq!(part_one(&get_input(2019, 2, InputType::Example, 1)?), 2);
    //     assert_eq!(part_one(&get_input(2019, 2, InputType::Example, 2)?), 2);
    //     assert_eq!(part_one(&get_input(2019, 2, InputType::Example, 3)?), 2);
    //     assert_eq!(part_one(&get_input(2019, 2, InputType::Example, 4)?), 30);
    //     Ok(())
    // }

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2019, 2, InputType::Challenge, 0)?),
            2894520 // 406198
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2019, 2, InputType::Challenge, 0)?),
            9342
        );
        Ok(())
    }
}
