use regex::Regex;

use crate::utils::{get_input, InputType};
use std::{
    collections::{HashMap, HashSet},
    io,
};

pub fn main() -> io::Result<()> {
    let input = &get_input(2021, 19, InputType::Challenge, 0)?;
    let (one, two) = part_one_and_two(input);
    println!("part one: {}", one);
    println!("part two: {}", two);
    Ok(())
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut beacons: HashSet<(i32, i32, i32)> = HashSet::new();
    let default_orientation = PointOrientation::new(ShiftCount::ZERO, false, false, false);

    let point_match = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();

    for line in input.lines() {
        if line.len() == 0 {
            scanners.push(Scanner::new(beacons, default_orientation));
            beacons = HashSet::new();
        } else if let Some(cap) = point_match.captures(line) {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            let z = cap[3].parse::<i32>().unwrap();

            beacons.insert((x, y, z));
        }
    }
    // last scanner
    if beacons.len() > 0 {
        scanners.push(Scanner::new(beacons, default_orientation));
    }

    scanners
}

#[derive(Debug, Clone, Copy)]
enum ShiftCount {
    ZERO,
    ONE,
    TWO,
}

impl ShiftCount {
    fn iterator() -> impl Iterator<Item = ShiftCount> {
        [ShiftCount::ZERO, ShiftCount::ONE, ShiftCount::TWO]
            .iter()
            .copied()
    }
}

#[derive(Debug, Clone, Copy)]
struct PointOrientation {
    shift_count: ShiftCount,
    swap: bool,
    flip_x: bool,
    flip_z: bool,
}

impl PointOrientation {
    fn new(shift_count: ShiftCount, swap: bool, flip_x: bool, flip_z: bool) -> Self {
        Self {
            shift_count,
            swap,
            flip_x,
            flip_z,
        }
    }

    fn orient_point(&self, point: &(i32, i32, i32)) -> (i32, i32, i32) {
        let mut new_point: (i32, i32, i32);
        match self.shift_count {
            ShiftCount::ZERO => new_point = point.clone(),
            ShiftCount::ONE => new_point = (point.1, point.2, point.0),
            ShiftCount::TWO => new_point = (point.2, point.0, point.1),
        }
        if self.swap {
            new_point = (new_point.0, -new_point.2, new_point.1);
        }
        if self.flip_x {
            new_point = (-new_point.0, -new_point.1, new_point.2);
        }
        if self.flip_z {
            new_point = (new_point.0, -new_point.1, -new_point.2);
        }
        new_point
    }

    fn all_orientations() -> Vec<PointOrientation> {
        let mut orientations = vec![];
        for count in ShiftCount::iterator() {
            for swap in [true, false] {
                for flip_x in [true, false] {
                    for flip_z in [true, false] {
                        orientations.push(PointOrientation::new(count, swap, flip_x, flip_z));
                    }
                }
            }
        }
        orientations
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: HashSet<(i32, i32, i32)>,
    orientation: PointOrientation,
}

impl Scanner {
    fn new(beacons: HashSet<(i32, i32, i32)>, orientation: PointOrientation) -> Self {
        Self {
            beacons,
            orientation,
        }
    }

    fn set_perspective(&mut self, orientation: &PointOrientation) {
        self.orientation = orientation.clone();
    }

    fn add_beacons(&mut self, new_beacons: HashSet<(i32, i32, i32)>) {
        self.beacons.extend(new_beacons.iter());
    }

    fn get_points(&self) -> HashSet<(i32, i32, i32)> {
        HashSet::from_iter(
            self.beacons
                .iter()
                .map(|point| self.orientation.orient_point(point)),
        )
    }

    fn get_overlap_delta(&self, scanner: &Scanner) -> Option<((i32, i32, i32), i32)> {
        let mut difference_map: HashMap<(i32, i32, i32), i32> = HashMap::new();

        for own in &self.get_points() {
            for other in &scanner.get_points() {
                let difference = (own.0 - other.0, own.1 - other.1, own.2 - other.2);
                difference_map.insert(
                    difference,
                    *difference_map.get(&difference).unwrap_or(&0) + 1,
                );
            }
        }

        if let Some((delta, count)) = difference_map.iter().max_by(|a, b| a.1.cmp(b.1)) {
            return Some((*delta, *count));
        }
        None
    }
}

fn part_one_and_two(input: &str) -> (usize, i32) {
    let scanners = parse_input(input);

    let mut anchored_scanners: HashMap<usize, (i32, i32, i32)> = HashMap::new();
    anchored_scanners.insert(0, (0, 0, 0));

    let mut anchor = scanners[0].clone();

    // max iterations are equal to count of scanners (almost always lower)
    for _ in 0..scanners.len() {
        if scanners.len() == anchored_scanners.len() {
            break;
        }

        for (index, scanner) in scanners.iter().enumerate() {
            if anchored_scanners.contains_key(&index) {
                continue;
            }
            let mut current = scanner.clone();
            'orientation: for orientation in PointOrientation::all_orientations() {
                current.set_perspective(&orientation);
                if let Some((delta, count)) = anchor.get_overlap_delta(&current) {
                    if count >= 12 {
                        // if we found a match, add new points to the anchor
                        // makes it easier to keep track of total points at the end
                        // and eliminates a loop over anchored scanners
                        let new_anchor_points = current
                            .get_points()
                            .iter()
                            .map(|point| (point.0 + delta.0, point.1 + delta.1, point.2 + delta.2))
                            .collect::<HashSet<_>>();

                        anchor.add_beacons(new_anchor_points);
                        anchored_scanners.insert(index, delta);
                        break 'orientation;
                    }
                }
            }
        }
    }

    let mut biggest_dist = 0;

    for a in anchored_scanners.values() {
        for b in anchored_scanners.values() {
            let dist = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
            if biggest_dist < dist {
                biggest_dist = dist
            }
        }
    }

    (anchor.get_points().len(), biggest_dist)
}
