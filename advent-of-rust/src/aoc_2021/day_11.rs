use crate::utils::{get_input, InputType};
use queues::{IsQueue, Queue};
use std::{collections::HashSet, io};

pub fn main() -> io::Result<()> {
    let (part_one, part_two) = part_one_and_two(&get_input(2021, 11, InputType::Challenge, 0)?);
    println!(
        "part one: {}",
        part_one
    );
    println!(
        "part two: {}",
        part_two
    );
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| -> Vec<u32> {
            line.chars()
                .map(|letter| -> u32 {
                    if let Some(octopus_val) = letter.to_digit(10) {
                        return octopus_val;
                    }
                    panic!("Unable to parse character: {}", letter);
                })
                .collect()
        })
        .collect()
}

fn get_positions_to_check(
    (x, y): (usize, usize),
    (max_x, max_y): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut positions_to_check = Vec::new();
    if y > 0 {
        positions_to_check.push((x, y - 1));
        if x > 0 {
            positions_to_check.push((x - 1, y - 1));
        }
        if x < max_x {
            positions_to_check.push((x + 1, y - 1));
        }
    }
    if y < max_y {
        positions_to_check.push((x, y + 1));
        if x > 0 {
            positions_to_check.push((x - 1, y + 1));
        }
        if x < max_y {
            positions_to_check.push((x + 1, y + 1));
        }
    }
    if x > 0 {
        positions_to_check.push((x - 1, y));
    }
    if x < max_x {
        positions_to_check.push((x + 1, y));
    }

    positions_to_check
}

fn part_one_and_two(input: &str) -> (usize, i32) {
    let mut map = parse_input(input);
    let mut count_flashes = 0;

    let (dim_x, dim_y) = (map.len(), map[0].len());

    for step in 1..1000 {
        let mut queue: Queue<(usize, usize)> = Queue::new();
        let mut queue_set: HashSet<(usize, usize)> = HashSet::new();

        for x in 0..dim_x {
            for y in 0..dim_y {
                map[x][y] += 1;
                if map[x][y] > 9 {
                    queue_set.insert((x, y));
                    let added = queue.add((x, y));
                    if added.is_err() {
                        panic!("Failed to add {:?} to queue", (x, y));
                    }
                }
            }
        }
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        while queue.size() > 0 {
            if let Ok((x, y)) = queue.remove() {
                queue_set.remove(&(x,y));
                if map[x][y] > 9 {
                    flashed.insert((x, y));
                    let neighbors = get_positions_to_check((x, y), (dim_x - 1, dim_y - 1));
                    neighbors.iter().for_each(|position| {
                        if flashed.contains(&position) {
                            return;
                        }
                        map[position.0][position.1] += 1;

                        if map[position.0][position.1] <= 9 || queue_set.contains(&position){
                            return;
                        }
                        queue_set.insert(*position);
                        let added = queue.add(*position);
                        if added.is_err() {
                            panic!("Unable to add {:?} to queue", position);
                        }
                    })
                }
            }
        }


        // println!("flashed {} times in map after step {}", flashed.len(), step);
        if step <= 100 {
            count_flashes += flashed.len();
        }

        if flashed.len() == dim_x * dim_y {
            return (count_flashes, step);
        }

        flashed.iter().for_each(|(x, y)| {
            map[*x][*y] = 0;
        });
    }
    (0,0)
}
