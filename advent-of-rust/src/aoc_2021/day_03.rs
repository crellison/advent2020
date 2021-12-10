use std::collections::HashSet;

#[allow(dead_code)]
fn part_one(input: &str) -> i32 {
    let mut counter: Vec<i32> = vec![];
    for line in input.lines() {
        if counter.len() == 0 {
            counter = vec![0; line.len()];
        }
        for (index, char) in line.chars().enumerate() {
            counter[index] += if char == '0' { -1 } else { 1 };
        }
    }

    counter.reverse();

    let gamma = counter.iter().enumerate().fold(0, |acc, (index, x)| {
        let bit_val = if x > &0 { 1 } else { 0 };
        acc + bit_val * 2_i32.pow(index as u32) as i32
    });

    let epsilon = counter.iter().enumerate().fold(0, |acc, (index, x)| {
        let bit_val = if x <= &0 { 1 } else { 0 };
        acc + bit_val * 2_i32.pow(index as u32) as i32
    });

    gamma + epsilon
}

#[allow(dead_code)]
fn part_two(input: &str) -> i32 {
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - 48).collect::<Vec<i32>>())
        .collect();

    let num_bits = lines[0].len();

    let life_support_rating: i32 = [true, false]
        .map(|keep_common| -> i32 {
            let mut valid_indices: HashSet<usize> = HashSet::new();
            valid_indices.extend(0..lines.len());

            let mut char_index = 0;

            while char_index < num_bits && valid_indices.len() > 1 {
                let num_vals = valid_indices.len() as i32;
                let count_ones: i32 = valid_indices
                    .iter()
                    .fold(0, |acc, cur| acc + lines[*cur][char_index]);
                let mut expected_digit = if keep_common { 0 } else { 1 };
                if count_ones * 2 >= num_vals {
                    expected_digit = if keep_common { 1 } else { 0 };
                }
                valid_indices.retain(|&index| lines[index][char_index] == expected_digit);
                char_index += 1;
            }

            if let Some(index) = valid_indices.iter().next() {
                println!("final number is {:?}", lines[*index]);
                return lines[*index]
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (index, bit)| -> i32 {
                        acc + (2_i32.pow((num_bits - 1 - index) as u32) * bit)
                    });
            }
            0
        })
        .iter()
        .product();

    life_support_rating
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 3, InputType::Challenge, 0)?),
            4095
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 3, InputType::Challenge, 0)?),
            3379326
        );
        Ok(())
    }
}
