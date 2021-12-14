use regex::Regex;

use crate::utils::{get_input, InputType};
use std::{collections::HashMap, io};

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 14, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 14, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn build_mapping(mapping_rules: &str) -> HashMap<String, String> {
    let rule_match = Regex::new(r"([A-Z]+) -> ([A-Z])").unwrap();
    let mut rules: HashMap<String, String> = HashMap::new();

    for line in mapping_rules.lines() {
        if let Some(instructions) = rule_match.captures(line) {
            let anchor = instructions[1].to_string();
            let value = instructions[2].to_string();

            rules.insert(anchor, value);
        }
    }

    rules
}

#[derive(Default)]
struct SlowMap {
    template: Vec<char>,
    mapping: HashMap<String, String>,
}

impl SlowMap {
    fn new(input: &str) -> Self {
        let template = input.lines().collect::<Vec<&str>>()[0]
            .chars()
            .collect::<Vec<char>>();
        let mapping = build_mapping(input);
        Self { template, mapping }
    }

    fn iterate_template(&mut self, steps: u32) {
        for _ in 0..steps {
            let mut tmp: Vec<char> = Vec::new();
            for i in 0..self.template.len() - 1 {
                tmp.push(self.template[i]);
                let window = String::from_iter([self.template[i], self.template[i + 1]]);
                if let Some(val) = self.mapping.get(&window) {
                    tmp.extend(val.chars());
                }
            }
            tmp.push(self.template[self.template.len() - 1]);
            self.template = tmp;
        }
    }

    fn get_character_counts(&self) -> HashMap<char, u32> {
        self.template
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<char, u32>, cur| {
                if let Some(count) = acc.get_mut(cur) {
                    *count += 1;
                } else {
                    acc.insert(*cur, 0);
                }
                acc
            })
    }
}

fn part_one(input: &str) -> u32 {
    let mut slow_map = SlowMap::new(input);
    slow_map.iterate_template(10);
    let counts = slow_map.get_character_counts();

    if let (Some(max), Some(min)) = (counts.values().max(), counts.values().min()) {
        return max - min;
    }
    0
}

struct FastMap {
    char_pairs: HashMap<String, u64>,
    first_letter: char,
    last_letter: char,
    mapping: HashMap<String, String>,
}

impl FastMap {
    fn new(input: &str) -> Self {
        let first_line = input.lines().collect::<Vec<&str>>()[0]
            .chars()
            .collect::<Vec<char>>();
        let mut char_pairs: HashMap<String, u64> = HashMap::new();
        for i in 0..first_line.len() - 1 {
            let pair = String::from_iter([first_line[i], first_line[i + 1]]);
            if let Some(count) = char_pairs.get_mut(&pair) {
                *count += 1
            } else {
                char_pairs.insert(pair, 1);
            }
        }
        let mapping = build_mapping(input);

        Self {
            char_pairs,
            first_letter: first_line[0],
            last_letter: first_line[first_line.len() - 1],
            mapping,
        }
    }

    fn iterate_pairs(&mut self, steps: u32) {
        for _ in 0..steps {
            let mut tmp: HashMap<String, u64> = HashMap::new();

            let mut insert_pair = |pair: &String, pair_count: &u64| {
                if let Some(count) = tmp.get_mut(pair) {
                    *count += pair_count
                } else {
                    tmp.insert(pair.to_string(), *pair_count);
                }
            };

            for (pair, pair_count) in &self.char_pairs {
                if let Some(middle_char) = self.mapping.get(pair) {
                    let pair_elts = pair.chars().map(|c| c.to_string()).collect::<Vec<String>>();
                    let first_pair =
                        String::from_iter([pair_elts[0].to_string(), middle_char.to_string()]);
                    let second_pair =
                        String::from_iter([middle_char.to_string(), pair_elts[1].to_string()]);
                    insert_pair(&first_pair, pair_count);
                    insert_pair(&second_pair, pair_count);
                } else {
                    insert_pair(pair, pair_count);
                }
            }

            self.char_pairs = tmp.clone();
        }
    }

    fn get_character_counts(&self) -> HashMap<char, u64> {
        let mut char_counts: HashMap<char, u64> = HashMap::new();
        if self.first_letter == self.last_letter {
            char_counts.insert(self.first_letter, 2);
        } else {
            char_counts.insert(self.first_letter, 1);
            char_counts.insert(self.last_letter, 1);
        }
    
        for (pair, pair_count) in &self.char_pairs {
            for letter in pair.chars() {
                if let Some(count) = char_counts.get_mut(&letter) {
                    *count += pair_count;
                } else {
                    char_counts.insert(letter, *pair_count);
                }
            }
        }
    
        for count in char_counts.values_mut() {
            *count /= 2;
        }
    
        char_counts
    }
}

fn part_two(input: &str) -> u64 {
    let mut fast_map = FastMap::new(input);
    fast_map.iterate_pairs(40);
    let char_counts = fast_map.get_character_counts();

    if let (Some(max), Some(min)) = (char_counts.values().max(), char_counts.values().min()) {
        return max - min;
    }
    0
}
