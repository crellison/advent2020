use std::{collections::HashMap, io, ops::Range};

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one()
    );
    println!(
        "part two: {}",
        part_two()
    );
    Ok(())
}

// input: 197487-673251

fn part_one() -> usize {
    let possible_pw: Range<u32> = 197487..673251;
    let pw_iter = possible_pw.into_iter().filter(|x| is_legal_pw(*x));
    pw_iter.count()
}

fn is_legal_pw(num: u32) -> bool {
    let mut has_double = false;
    let mut indecies = (0..5).into_iter();
    let increasing = indecies.all(|i: u32| {
        let first = get_digit(num, i);
        let second = get_digit(num, i + 1);
        if first == second {
            has_double = true;
            return true;
        }
        return first > second;
    });
    has_double && increasing
}

fn is_legal_pw_2(num: u32) -> bool {
    let mut runs: HashMap<u32, u32> = HashMap::new();
    let mut indecies = (0..5).into_iter();
    let increasing = indecies.all(|i: u32| {
        let first = get_digit(num, i);
        let second = get_digit(num, i + 1);
        if first == second {
            if let Some(k) = runs.get_mut(&first) {
                *k += 1;
            } else {
                runs.insert(first, 2);
            }
            return true;
        }
        return first > second;
    });
    runs.values().any(|x| *x == 2) && increasing
}

fn get_digit(num: u32, digit: u32) -> u32 {
    num / (10_u32.pow(digit)) % 10_u32
}

fn part_two() -> usize {
    let possible_pw: Range<u32> = 197487..673251;
    let pw_iter = possible_pw.into_iter().filter(|x| is_legal_pw_2(*x));
    pw_iter.count()
}
