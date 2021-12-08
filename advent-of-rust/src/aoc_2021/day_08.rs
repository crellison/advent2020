use regex::Regex;
use std::collections::HashSet;

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
/**
KeyCharMap - creates a character map to parse seven-segment numbers

Number Format shown below

```
 aaa
b   c
b   c
 ddd
e   f
e   f
 ggg
```

* 1, 4, 7, and 8 can all be identified by segment count
* 2, 3, and 5 all have five segments
* 0, 6, and 9 all have six segments

The latter two groups can be differentiated if you know which segment corresponds
with the c, d, and f segments in the chart above (2,3,5 only need c and f).
*/
struct KeyCharMap {
    c: String,
    d: String,
    f: String,
}

impl KeyCharMap {
    pub fn new(ten_numbers: &str) -> Self {
        let (mut one, mut four): (HashSet<char>, HashSet<char>) = (HashSet::new(), HashSet::new());
        let (mut set_235, mut set_069) = (HashSet::from(CHARACTERS), HashSet::from(CHARACTERS));

        for number in ten_numbers.split(" ") {
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

        if let (Some(pos_d), Some(pos_f)) = (
            set_235.intersection(&four).next(),
            set_069.intersection(&one).next(),
        ) {
            if let Some(pos_c) = one.iter().filter(|letter| *letter != pos_f).next() {
                return Self {
                    c: pos_c.to_string(),
                    d: pos_d.to_string(),
                    f: pos_f.to_string(),
                };
            }
        }
        panic!("Unable to find positions from input: {}", ten_numbers);
    }

    pub fn get_digit(&self, number: &str) -> i32 {
        let (has_c, has_d, has_f) = (
            number.contains(&self.c),
            number.contains(&self.d),
            number.contains(&self.f),
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
    }
}

#[allow(dead_code)]
fn part_two(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut line_chunked = line.split(" | ");

        if let Some(ten_numbers) = line_chunked.next() {
            let key_char_map = KeyCharMap::new(ten_numbers);
            if let Some(output) = line_chunked.next() {
                let output_numbers: i32 = output
                    .split(" ")
                    .enumerate()
                    .map(|(index, number)| {
                        // always four digits
                        key_char_map.get_digit(number) * 10_i32.pow((3 - index) as u32)
                    })
                    .sum();
                sum += output_numbers;
            }
        }
    }
    return sum;
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
