use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    let (pt_one, pt_two) = part_one_and_two(&get_input(2021, 10, InputType::Challenge, 0)?);
    println!("part one: {}", pt_one);
    println!("part two: {}", pt_two);
    Ok(())
}

const OPEN_CHARS: [char; 4] = ['{', '[', '(', '<'];
const CLOSE_CHARS: [char; 4] = ['}', ']', ')', '>'];

fn part_one_and_two(input: &str) -> (u32, u64) {
    let mut syntax_error_score = 0;
    let mut incomplete_scores: Vec<u64> = Vec::new();
    for line in input.lines() {
        let mut char_queue: Vec<char> = Vec::new();
        let mut is_corrupt = false;
        for character in line.chars() {
            // if opener, add to char queue
            if let Some(index) = OPEN_CHARS.iter().position(|elt| elt == &character) {
                char_queue.push(CLOSE_CHARS[index]);

            // otherwise it's a closing paren
            } else if let Some(expected) = char_queue.pop() {
                if expected != character {
                    is_corrupt = true;
                    syntax_error_score += match character {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("Unexpected character: {}", character),
                    };
                    break;
                }
            } else {
                panic!("Unexpected closing char: {}", character);
            }
        }
        // calc incomplete score if not corrupt and expected chars
        if !is_corrupt && char_queue.len() > 0 {
            char_queue.reverse();
            let score = char_queue.iter().fold(0, |acc, letter| {
                acc * 5
                    + match letter {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Unexpected character: {}", letter),
                    }
            });
            incomplete_scores.push(score);
        }
    }
    incomplete_scores.sort();
    (
        syntax_error_score,
        incomplete_scores[incomplete_scores.len() / 2],
    )
}
