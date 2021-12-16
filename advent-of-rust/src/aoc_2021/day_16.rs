use crate::utils::{get_input, InputType};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 16, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 16, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn parse_bit_queue(input: &str) -> Vec<u32> {
    let mut bit_queue = Vec::new();
    input.chars().for_each(|character| {
        if let Some(char_val) = character.to_digit(16) {
            let char_to_bin = format!("{:b}", char_val);
            let padding_len = 4 - char_to_bin.len();
            bit_queue.extend(vec![0; padding_len]);
            bit_queue.extend(char_to_bin.chars().map(|l| if l == '0' { 0 } else { 1 }));
        }
    });
    bit_queue
}

fn bin_array_to_decimal(bin_array: &Vec<u32>) -> u64 {
    bin_array.iter().enumerate().fold(0, |acc, (i, bit)| {
        let power = bin_array.len() - 1 - i;
        if *bit == 1 {
            return acc + 2_u64.pow(power as u32);
        }
        acc
    })
}

#[derive(Debug, Default)]
struct Packet {
    version: u32,
    type_id: u32,
    value: Option<u64>,
    children: Option<Vec<Packet>>,
}

impl Packet {
    fn get_version_sum(&self) -> i32 {
        let mut sum = self.version as i32;
        if let Some(children) = &self.children {
            sum += children
                .iter()
                .map(|child| child.get_version_sum())
                .sum::<i32>();
        }
        sum
    }

    fn calc_packet_value(&self) -> u64 {
        let default_children = Vec::new();
        let children = self.children.as_ref().unwrap_or(&default_children);
        match self.type_id {
            0 => children.iter().map(|child| child.calc_packet_value()).sum(),
            1 => children
                .iter()
                .map(|child| child.calc_packet_value())
                .product(),
            2 => children.iter().map(|child| child.calc_packet_value()).min().unwrap_or(u64::MAX),
            3 => children.iter().map(|child| child.calc_packet_value()).max().unwrap_or(u64::MAX),
            4 => self.value.unwrap_or(0),
            5 => {
                if children[0].calc_packet_value() > children[1].calc_packet_value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if children[0].calc_packet_value() < children[1].calc_packet_value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if children[0].calc_packet_value() == children[1].calc_packet_value() {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

fn get_packet(bit_queue: &Vec<u32>, pointer: &usize) -> (Packet, usize) {
    let version =
        4 * bit_queue[pointer + 0] + 2 * bit_queue[pointer + 1] + 1 * bit_queue[pointer + 2];
    let type_id =
        4 * bit_queue[pointer + 3] + 2 * bit_queue[pointer + 4] + 1 * bit_queue[pointer + 5];

    let mut cursor = pointer + 6;
    if type_id == 4 {
        let mut bin_array = vec![];
        while bit_queue[cursor] != 0 {
            bin_array.extend((0..4).map(|i| bit_queue[cursor + 1 + i]));
            cursor += 5;
        }
        bin_array.extend((0..4).map(|i| bit_queue[cursor + 1 + i]));
        cursor += 5;

        let value = bin_array_to_decimal(&bin_array);
        return (
            Packet {
                type_id,
                value: Some(value),
                version,
                children: None,
            },
            cursor,
        );
    }

    let length_type = bit_queue[cursor];
    cursor += 1;
    if length_type == 0 {
        let length_subpackets =
            bin_array_to_decimal(&Vec::from_iter((0..15).map(|i| bit_queue[cursor + i])));
        cursor += 15;
        let expected_completion = cursor + (length_subpackets as usize);

        let mut children: Vec<Packet> = Vec::new();

        while cursor < expected_completion {
            let (next_child, new_cursor) = get_packet(bit_queue, &cursor);
            children.push(next_child);
            cursor = new_cursor
        }

        return (
            Packet {
                version,
                type_id,
                value: None,
                children: Some(children),
            },
            cursor,
        );
    }

    let sub_packet_count =
        bin_array_to_decimal(&Vec::from_iter((0..11).map(|i| bit_queue[cursor + i])));
    cursor += 11;
    let mut children: Vec<Packet> = Vec::new();
    while children.len() < (sub_packet_count as usize) {
        let (next_child, new_cursor) = get_packet(bit_queue, &cursor);
        children.push(next_child);
        cursor = new_cursor;
    }
    (
        Packet {
            type_id,
            version,
            value: None,
            children: Some(children),
        },
        cursor,
    )
}

fn part_one(input: &str) -> i32 {
    let bit_queue = parse_bit_queue(input);
    let (packet, _) = get_packet(&bit_queue, &0);
    packet.get_version_sum()
}

fn part_two(input: &str) -> u64 {
    let bit_queue = parse_bit_queue(input);
    let (packet, _) = get_packet(&bit_queue, &0);
    packet.calc_packet_value()
}
