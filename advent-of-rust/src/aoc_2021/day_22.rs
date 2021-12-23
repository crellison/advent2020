use regex::Regex;

use crate::utils::{get_input, InputType};
use std::collections::HashSet;
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 22, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 22, InputType::Challenge, 0)?)
    );
    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct ConwayRange {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl ConwayRange {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self {
        if x_max < x_min || y_max < y_min || z_max < z_min {
            panic!(
                "Invalid args for ConwayRange: expected {} <= {}, {} <= {}, {} <= {}",
                x_min, x_max, y_min, y_max, z_min, z_max,
            )
        }
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    fn from_tuples(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Self {
        Self::new(x.0, x.1, y.0, y.1, z.0, z.1)
    }

    fn from_big_tuple(
        (x_min, x_max, y_min, y_max, z_min, z_max): (i32, i32, i32, i32, i32, i32),
    ) -> Self {
        Self::new(x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn len(&self) -> usize {
        ((self.x_max - self.x_min + 1)
            * (self.y_max - self.y_min + 1)
            * (self.z_max - self.z_min + 1)) as usize
    }

    fn in_range(&self, min: i32, max: i32) -> bool {
        self.x_min.min(self.y_min).min(self.z_min) >= min
            && self.x_max.max(self.y_max).max(self.z_max) <= max
    }

    fn intersects(&self, other: &ConwayRange) -> bool {
        let x_intersects = self.x_max >= other.x_min && self.x_min <= other.x_max;
        let y_intersects = self.y_max >= other.y_min && self.y_min <= other.y_max;
        let z_intersects = self.z_max >= other.z_min && self.z_min <= other.z_max;
        x_intersects && y_intersects && z_intersects
    }

    /** Subtracts one cube from this one
     *
     * Assumes that cubes overlap (use `.intersects()`)
     *
     */
    fn subtract(&self, comp: &ConwayRange) -> Vec<ConwayRange> {
        let mut x = [self.x_min, self.x_max, comp.x_min, comp.x_max];
        x.sort();
        let mut y = [self.y_min, self.y_max, comp.y_min, comp.y_max];
        y.sort();
        let mut z = [self.z_min, self.z_max, comp.z_min, comp.z_max];
        z.sort();

        let x_ranges = [(x[0], x[1] - 1), (x[1], x[2]), (x[2] + 1, x[3])];
        let y_ranges = [(y[0], y[1] - 1), (y[1], y[2]), (y[2] + 1, y[3])];
        let z_ranges = [(z[0], z[1] - 1), (z[1], z[2]), (z[2] + 1, z[3])];

        let mut remainder = Vec::new();

        // could be more intelligent here and maybe combine ranges if they are overlapping
        for x_span in x_ranges {
            for y_span in y_ranges {
                for z_span in z_ranges {
                    if x_span.0 > x_span.1 || y_span.0 > y_span.1 || z_span.0 > z_span.1 {
                        // skip invalid cubes
                        continue;
                    }
                    let current_range = ConwayRange::from_tuples(x_span, y_span, z_span);
                    if current_range.intersects(self) && !current_range.intersects(comp) {
                        remainder.push(current_range);
                    }
                }
            }
        }

        remainder
    }
}

fn parse_line(line: &str) -> (i32, i32, i32, i32, i32, i32) {
    let xyz_match =
        Regex::new(r"x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    if let Some(cap) = xyz_match.captures(line) {
        let (x_min, x_max, y_min, y_max, z_min, z_max) = (
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
            cap[5].parse::<i32>().unwrap(),
            cap[6].parse::<i32>().unwrap(),
        );

        return (x_min, x_max, y_min, y_max, z_min, z_max);
    }
    panic!("Invalid line: {}", line);
}

fn part_one(input: &str) -> usize {
    let mut conway_map: HashSet<(i32, i32, i32)> = HashSet::new();
    for line in input.lines() {
        let on = line.contains("on");
        let (x_min, x_max, y_min, y_max, z_min, z_max) = parse_line(line);

        if x_min.min(y_min).min(z_min) < -50 || x_max.max(y_max).max(z_max) > 50 {
            break;
        }

        for x in x_min..x_max + 1 {
            for y in y_min..y_max + 1 {
                for z in z_min..z_max + 1 {
                    if on {
                        conway_map.insert((x, y, z));
                    } else {
                        conway_map.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    conway_map.len()
}

fn part_two(input: &str) -> usize {
    let mut conway_map: HashSet<ConwayRange> = HashSet::new();
    for line in input.lines() {
        let mut next_conway_map: HashSet<ConwayRange> = conway_map.clone();
        let on = line.contains("on");
        let new_range = ConwayRange::from_big_tuple(parse_line(line));

        if !new_range.in_range(-50, 50) {
            break;
        }

        if on && conway_map.len() == 0 {
            conway_map.insert(new_range);
            continue;
        }

        let mut intersects_a_range = false;
        for range in conway_map {
            if range.intersects(&new_range) {
                intersects_a_range = true;
                if on {
                    // keep everything
                    next_conway_map.extend(new_range.subtract(&range));
                } else {
                    next_conway_map.extend(range.subtract(&new_range));
                    next_conway_map.remove(&range);
                }
            }
        }
        if !intersects_a_range {
            next_conway_map.insert(new_range);
        }
        conway_map = next_conway_map;
    }
    let count = conway_map.len();
    println!("finished! map has {} elements", count);
    println!(
        "guessed size is {}",
        conway_map.iter().fold(0, |a, c| a + c.len())
    );
    0
    // println!("{:?}", points);
    // conway_map.iter().fold(0, |acc, cur| acc + cur.len())
    // 6616803199286 (too low)
    // 21350373047342664 (too high)
    // 25138526634640680 (too high)
}
