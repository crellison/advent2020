use std::collections::HashMap;

#[allow(dead_code)]
fn part_one(input: &str) -> isize {
    let mut line_iter = input.lines();
    let first = line_iter.next().unwrap();
    let line_len = first.len();
    let mut bit_counter = vec![0; line_len];
    for line in input.lines() {
        let char_zipper: Vec<(i32, char)> = (0..).zip(line.chars()).collect();
        for (index, char) in char_zipper {
            bit_counter[index as usize] += if char == '0' { -1 } else { 1 };
        }
    }
    let get_gamma_epsilon = |is_gamma: bool| -> isize {
        let lambda = |&x: &i32| {
            if is_gamma {
                return if x > 0 { "1" } else { "0" };
            }
            return if x > 0 { "0" } else { "1" };
        };
        let num = bit_counter
            .iter()
            .map(lambda)
            .collect::<Vec<&str>>()
            .join("");
        isize::from_str_radix(&num, 2).unwrap()
    };

    get_gamma_epsilon(true) + get_gamma_epsilon(false)
}

#[allow(dead_code)]
fn part_two(input: &str) -> isize {
    let lines: Vec<(i32, Vec<i32>)> = (0..)
        .zip(
            input
                .lines()
                .map(|line| -> Vec<i32> { line.chars().map(|c| c as i32 - 48).collect() }),
        )
        .collect();

    let get_common_num = |most_common: bool| -> isize {
        let mut valid_indices: HashMap<i32, bool> = HashMap::new();
        for (index, _) in &lines {
            valid_indices.insert(*index, true);
        }

        let mut cur_char_index: usize = 0;
        while valid_indices.values().len() > 1 {

            let mut common_val = 0;
            let mut num_vals = 0;
            for (index, line) in &lines {
                if valid_indices.contains_key(&index) {
                    common_val += line[cur_char_index];
                    num_vals += 1;
                }
            }
            let keep_val = if common_val * 2 >= num_vals {
                if most_common { 1 } else { 0 }
            } else {
                if most_common { 0 } else { 1 }
            };

            valid_indices.retain(|&key, _| lines[key as usize].1[cur_char_index] == keep_val);
            cur_char_index += 1;
        }
        let valid_key = valid_indices.keys().collect::<Vec<&i32>>()[0];
        let num = lines[*valid_key as usize]
            .1
            .iter()
            .map(|digit| digit.to_string())
            .collect::<Vec<String>>()
            .join("");
        isize::from_str_radix(&num, 2).unwrap()
    };

    get_common_num(true) * get_common_num(false)
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(part_one(&get_input(2021, 3, InputType::Challenge, 0)?), 4095);
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(part_two(&get_input(2021, 3, InputType::Challenge, 0)?), 3379326);
        Ok(())
    }
}
