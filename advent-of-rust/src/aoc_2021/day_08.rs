use std::collections::HashSet;
use regex::Regex;

#[allow(dead_code)]
fn part_one(input: &str) -> i32 {
    let number_match = Regex::new(r"[a-g]+").unwrap();
    let mut count_unique_nums = 0;
    let unique_nums: [usize; 4] = [2, 3, 4, 7];
    for line in input.lines() {
        if let Some(output_nums) = line.split(" | ").nth(1) {
            count_unique_nums += number_match.captures_iter(output_nums).fold(0, |acc, cur| {
                acc + if unique_nums.contains(&cur[0].len()) {
                    1
                } else {
                    0
                }
            });
        }
    }
    count_unique_nums
}

const CHARACTERS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

#[allow(dead_code)]
fn part_two(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut line_chunked = line.split(" | ");

        let ten_numbers = line_chunked.next().unwrap().split(" ");
        let (mut one, mut four): (HashSet<char>, HashSet<char>) = (HashSet::new(), HashSet::new());
        let (mut set_235, mut set_069) = (HashSet::from(CHARACTERS), HashSet::from(CHARACTERS));

        for number in ten_numbers {
            let number_set: HashSet<char> = HashSet::from_iter(number.chars());
            match number.len() {
                2 => one.extend(number_set),
                4 => four.extend(number_set),
                5 => {
                    set_235 = set_235.intersection(&number_set).copied().collect();
                }
                6 => {
                    set_069 = set_069.intersection(&number_set).copied().collect();
                }
                _ => {}
            };
        }

        let pos_d = set_235.intersection(&four).next().unwrap();
        let pos_f = set_069.intersection(&one).next().unwrap();
        let pos_c = one.iter().filter(|letter| *letter != pos_f).next().unwrap();

        let get_digit = |number: &str| {
            let (has_c, has_d, has_f) = (
                number.contains(&pos_c.to_string()),
                number.contains(&pos_d.to_string()),
                number.contains(&pos_f.to_string()),
            );
            match number.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => {
                    if has_c && has_f {
                        3
                    } else if has_f {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    if has_c && has_f {
                        if has_d {
                            return 9;
                        }
                        0
                    } else {
                        6
                    }
                }
                7 => 8,
                _ => panic!("Unexpected length"),
            }
        };

        if let Some(output) = line_chunked.next() {
            let output_numbers: i32 = output
                .split(" ")
                .enumerate()
                .map(|(index, number)| get_digit(number) * 10_i32.pow((3 - index) as u32))
                .sum();
            // println!("{}\n", output_numbers);
            sum += output_numbers;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(part_one(&get_input(2021, 8, InputType::Challenge, 0)?), 239);
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 8, InputType::Challenge, 0)?),
            946346
        );
        Ok(())
    }
}
