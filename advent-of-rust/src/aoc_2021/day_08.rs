use crate::utils::{get_input, InputType};
use regex::Regex;
use std::collections::HashSet;
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 8, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 8, InputType::Challenge, 0)?)
    );
    Ok(())
}

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

Update: this is faster done with supersets
*/
struct KeyCharMap {
    one: HashSet<char>,
    four: HashSet<char>,
    seven: HashSet<char>,
    eight: HashSet<char>,
}

impl KeyCharMap {
    pub fn new(ten_numbers: &str) -> Self {
        let mut key_char_map = Self {
            one: HashSet::new(),
            four: HashSet::new(),
            seven: HashSet::new(),
            eight: HashSet::new(),
        };

        for number in ten_numbers.split(" ") {
            let number_set: HashSet<char> = HashSet::from_iter(number.chars());
            match number.len() {
                2 => key_char_map.one.extend(number_set),
                3 => key_char_map.seven.extend(number_set),
                4 => key_char_map.four.extend(number_set),
                7 => key_char_map.eight.extend(number_set),
                _ => {}
            };
        }

        key_char_map
    }

    pub fn get_digit(&self, number: &str) -> i32 {
        let char_set: HashSet<char> = HashSet::from_iter(number.chars());
        match number.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                if char_set.is_superset(&self.one) {
                    3
                } else if char_set.is_superset(&self.four.difference(&self.one).copied().collect())
                {
                    5
                } else {
                    2
                }
            }
            6 => {
                if char_set.is_superset(&self.four) {
                    9
                } else if char_set.is_superset(&self.seven) {
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
