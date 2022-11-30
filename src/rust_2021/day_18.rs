use regex::Regex;

use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 18, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 18, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn explode_num(snail_num: &String, index: usize) -> String {
    let ascii_digit = |c: char| c.is_ascii_digit();

    let mut exploded_string = String::new();

    // index is at '['
    let (prefix, close) = snail_num.split_at(index);
    let end_bracket = close.find(']').unwrap(); // I'm sorry, I know this exists

    let (nums, postfix) = close.split_at(end_bracket + 1);
    // println!("exploding {} at index {}", nums, index);
    let split_nums = nums.splitn(2, ",").collect::<Vec<&str>>();

    // parse a, b as numbers (oh the dreaded unwrap)
    let (a, b) = (
        split_nums[0].trim_start_matches("[").parse::<i32>().unwrap(),
        split_nums[1].trim_end_matches("]").parse::<i32>().unwrap(),
    );

    // there is a number before opening bracket
    if let Some((mut last_i, last_digit)) = prefix.rmatch_indices(ascii_digit).next() {
        let mut last_num = last_digit.to_string();
        if let Some(c) = snail_num.chars().nth(last_i - 1) {
            if c.is_ascii_digit() {
                last_num = format!("{}{}", c, last_num);
                last_i -= 1;
            }
        }
        let new_num = format!("{}", a + last_num.parse::<i32>().unwrap_or(0));

        // println!("changing {} to {}", last_num, new_num);
        let (save, change) = prefix.split_at(last_i);
        exploded_string.push_str(&save);
        exploded_string.push_str(&change.replacen(&last_num, &new_num, 1));
    } else { // no number before opening bracket
        exploded_string.push_str(&prefix);
    }

    exploded_string.push_str("0"); // postfix contains ']'

    // get next number after bracket
    if let Some((next_i, next_digit)) = postfix.match_indices(ascii_digit).next() {
        let mut next_num = next_digit.to_string();
        if let Some(c) = postfix.chars().nth(next_i + 1) {
            if c.is_ascii_digit() {
                next_num.push(c);
            }
        }
        let new_num = format!("{}", b + next_num.parse::<i32>().unwrap_or(0));
        exploded_string.push_str(&postfix.replacen(&next_num, &new_num, 1));
    } else { // no number after bracket
        exploded_string.push_str(&postfix);
    }

    exploded_string
}

fn reduce_snail_num(mut snail_num: String) -> String {
    let mut has_four_deep = true;
    let over_ten = Regex::new(r"(\d{2})").unwrap();

    
    while has_four_deep || over_ten.is_match(&snail_num) {
        if has_four_deep {
            let mut depth_counter = 0;
            for (index, char) in snail_num.clone().char_indices() {
                match char {
                    '[' => depth_counter += 1,
                    ']' => depth_counter -= 1,
                    _ => {}
                }
                if depth_counter == 5 {
                    snail_num = explode_num(&snail_num, index);
                    break;
                }

                if index == snail_num.len() - 1 {
                    has_four_deep = false;
                }
            }
        } else if let Some(cap) = over_ten.captures(&snail_num) {
            if let Ok(parsed) = cap[1].parse::<i32>() {
                let new = format!("[{},{}]", parsed / 2, (parsed + 1) / 2);
                // println!("splitting {} -> {}", &cap[1], new); 
                snail_num = snail_num.replacen(&cap[1], &new, 1);
            } else {
                panic!("failed to parse {}", &cap[1]);
            }
            // reset in case split group is four deep
            has_four_deep = true;
        }
    }

    snail_num
}

fn add_snail_nums(num_a: &str, num_b: &str) -> String {
    let snail_num = format!("[{},{}]", num_a, num_b);
    reduce_snail_num(snail_num)    
}

fn calc_magnitude(snail_num: &str) -> u32 {
    let mut shrinking = snail_num.to_string();
    let pair_match = Regex::new(r"\[(\d+),(\d+)\]").unwrap();

    while let Some(cap) = pair_match.captures(&shrinking) {
        let a = cap[1].parse::<u32>().unwrap();
        let b = cap[2].parse::<u32>().unwrap();

        let new_val = 3 * a + 2 * b;
        shrinking = shrinking.replace(&cap[0], &new_val.to_string());
    }

    shrinking.parse::<u32>().unwrap()
}

fn part_one(input: &str) -> u32 {
    let mut sum = String::new();
    for line in input.lines() {
        if sum.len() == 0 {
            sum = reduce_snail_num(line.to_string());
        } else {
            sum = add_snail_nums(&sum, line);
        }
    }
    calc_magnitude(&sum)
}

fn part_two(input: &str) -> u32 {
    let numbers: Vec<&str> = input.lines().collect();

    // largest sum **should** include the largest single number
    let biggest_num = *numbers.iter().reduce(|a,b| {
        if calc_magnitude(a) > calc_magnitude(b) { a } else { b }
    }).unwrap();

    let mut biggest_sum = 0;

    for i in 0..numbers.len() {
        let left_sum = calc_magnitude(&add_snail_nums(biggest_num, numbers[i]));
        let right_sum = calc_magnitude(&add_snail_nums(numbers[i], biggest_num));

        if left_sum > biggest_sum {
            biggest_sum = left_sum;
        }
        if right_sum > biggest_sum {
            biggest_sum = right_sum;
        }
    }

    biggest_sum
}
