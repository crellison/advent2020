const OPEN_CHARS: [char; 4] = ['{', '[', '(', '<'];
const CLOSE_CHARS: [char; 4] = ['}', ']', ')', '>'];

#[allow(dead_code)]
fn part_one(input: &str) -> u32 {
    let mut syntax_error_score = 0;
    for line in input.lines() {
        let mut char_queue: Vec<char> = Vec::new();
        for character in line.chars() {
            // if opener, add to char queue
            if let Some(index) = OPEN_CHARS.iter().position(|elt| elt == &character) {
                char_queue.push(CLOSE_CHARS[index]);

            // otherwise it's a closing paren
            } else if let Some(expected) = char_queue.pop() {
                if expected != character {
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
    }
    syntax_error_score
}

#[allow(dead_code)]
fn part_two(input: &str) -> u64 {
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
                    break;
                }
            } else {
                panic!("Unexpected closing char: {}", character);
            }
        }
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
    incomplete_scores[incomplete_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 10, InputType::Challenge, 0)?),
            464991
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 10, InputType::Challenge, 0)?),
            3662008566
        );
        Ok(())
    }
}
