use crate::utils::{get_input, InputType};
use std::{collections::HashSet, io};

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 20, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 20, InputType::Challenge, 0)?)
    );
    Ok(())
}

#[derive(Debug)]
struct Image {
    key: Vec<bool>,
    image: HashSet<(i32, i32)>,
    edge_val: bool,
}

impl Image {
    fn new(key: Vec<bool>, image: HashSet<(i32, i32)>) -> Self {
        Self {
            key,
            image,
            edge_val: false,
        }
    }

    fn len(&self) -> usize {
        self.image.len()
    }

    fn iterate_image(&mut self) {
        let mut next_image = HashSet::new();

        let ((min_x, min_y), (max_x, max_y)) = self.get_min_max_xy();

        for y in min_y - 2..max_y + 2 {
            for x in min_x - 2..max_x + 2 {
                let mut point_sum = 0;
                for (i, dy) in (-1..2).enumerate() {
                    for (j, dx) in (-1..2).enumerate() {
                        let current = (x + dx, y + dy);
                        let power = 8 - (i * 3 + j);
                        if self.image.contains(&current) {
                            point_sum += 1 << power;
                            continue;
                        }

                        let inside_edge = current.0 >= min_x
                            && current.0 <= max_x
                            && current.1 >= min_y
                            && current.1 <= max_y;

                        // then it is the edge!
                        if !inside_edge && self.edge_val {
                            point_sum += 1 << power;
                        }
                    }
                }

                if self.key[point_sum] {
                    next_image.insert((x, y));
                }
            }
        }

        self.edge_val = self.key[if self.edge_val { self.key.len() - 1 } else { 0 }];
        self.image = next_image
    }

    fn get_min_max_xy(&self) -> ((i32, i32), (i32,i32)) {
        let ((max_x, max_y), (min_x, min_y)) = self.image.iter().fold(
            ((i32::MIN, i32::MIN), (i32::MAX, i32::MAX)),
            |(min, max), (x, y)| {
                return (
                    (
                        if min.0 < *x { *x } else { min.0 },
                        if min.1 < *y { *y } else { min.1 },
                    ),
                    (
                        if max.0 > *x { *x } else { max.0 },
                        if max.1 > *y { *y } else { max.1 },
                    ),
                );
            },
        );

        ((min_x, min_y), (max_x, max_y))
    }
}

fn parse_input(input: &str) -> Image {
    if let Some(split_index) = input.find("\n\n") {
        let (key, image) = input.split_at(split_index);
        let mut image_positive = HashSet::new();
        for (y, line) in image.lines().enumerate() {
            for (x, letter) in line.char_indices() {
                if letter == '#' {
                    image_positive.insert((x as i32, (y as i32)));
                }
            }
        }
        return Image::new(
            key.chars().map(|c| c == '#').collect::<Vec<bool>>(),
            image_positive,
        );
    }
    panic!("Malformed input");
}

fn part_one(input: &str) -> usize {
    let mut image = parse_input(input);

    for _ in 0..2 {
        image.iterate_image();
    }
    image.len()
}

fn part_two(input: &str) -> usize {
    let mut image = parse_input(input);

    for _ in 0..50 {
        image.iterate_image();
    }
    image.len()
}
