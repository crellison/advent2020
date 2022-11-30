use crate::utils::{get_input, InputType};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 15, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 15, InputType::Challenge, 0)?)
    );
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|letter| letter.to_digit(10).unwrap_or(u32::MAX))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PointToCheck {
    cost: u32,
    position: (usize, usize),
    estimate: u32,
}

impl Ord for PointToCheck {
    fn cmp(&self, other: &PointToCheck) -> Ordering {
        (other.cost, other.estimate).cmp(&(self.cost, self.estimate))
    }
}

impl PartialOrd for PointToCheck {
    fn partial_cmp(&self, other: &PointToCheck) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
struct TwoDimMap {
    cost_map: Vec<Vec<u32>>,
    end: (usize, usize),
    start: (usize, usize),
    dim: (usize, usize),
}

impl TwoDimMap {
    fn new(cost_map: Vec<Vec<u32>>, end: (usize, usize), start: (usize, usize)) -> Self {
        let dim = (cost_map.len(), cost_map[0].len());
        Self {
            cost_map,
            end,
            start,
            dim,
        }
    }

    fn get_neighbors(&self, point: &PointToCheck) -> HashSet<PointToCheck> {
        let mut neighbors: HashSet<PointToCheck> = HashSet::new();

        if point.position.0 > 0 {
            let position = (point.position.0 - 1, point.position.1);
            neighbors.insert(PointToCheck {
                position,
                cost: point.cost + self.get_step_cost(&position),
                estimate: self.dist_to_end(&position),
            });
        }
        if point.position.0 < self.end.0 {
            let position = (point.position.0 + 1, point.position.1);
            neighbors.insert(PointToCheck {
                position,
                cost: point.cost + self.get_step_cost(&position),
                estimate: self.dist_to_end(&position),
            });
        }
        if point.position.1 > 0 {
            let position = (point.position.0, point.position.1 - 1);
            neighbors.insert(PointToCheck {
                position,
                cost: point.cost + self.get_step_cost(&position),
                estimate: self.dist_to_end(&position),
            });
        }
        if point.position.1 < self.end.0 {
            let position = (point.position.0, point.position.1 + 1);
            neighbors.insert(PointToCheck {
                position,
                cost: point.cost + self.get_step_cost(&position),
                estimate: self.dist_to_end(&position),
            });
        }

        neighbors
    }

    fn get_step_cost(&self, point: &(usize, usize)) -> u32 {
        let x = point.0 % self.dim.0;
        let y = point.1 % self.dim.1;
        let scale_factor = (point.0 / self.dim.0 + point.1 / self.dim.1) as u32;
        let new_val = (self.cost_map[x][y] + scale_factor) % 9;
        if new_val == 0 {
            9
        } else {
            new_val
        }
    }

    fn dist_to_end(&self, point: &(usize, usize)) -> u32 {
        (self.end.0 - point.0 + self.end.1 - point.1) as u32
    }

    fn find_cheapest_path_cost(&self) -> u32 {
        if self.start == self.end {
            return 0;
        }

        let mut seen_points: HashSet<(usize, usize)> = HashSet::new();
        let mut open: BinaryHeap<PointToCheck> = BinaryHeap::new();
        let start_point = PointToCheck {
            position: self.start,
            cost: 0,
            estimate: self.dist_to_end(&self.start),
        };
        open.push(start_point);

        while let Some(current) = open.pop() {
            if current.position == self.end {
                return current.cost;
            }

            for next in self.get_neighbors(&current) {
                if !seen_points.contains(&next.position) {
                    seen_points.insert(next.position);
                    open.push(next);
                }
            }
        }
        u32::MAX
    }
}

fn part_one(input: &str) -> u32 {
    let parsed = parse_input(input);
    let end = (parsed.len() - 1, parsed[0].len() - 1).clone();
    let map = TwoDimMap::new(parsed, end, (0, 0));
    map.find_cheapest_path_cost()
}

fn part_two(input: &str) -> u32 {
    let parsed = parse_input(input);
    let end = (parsed.len() * 5 - 1, parsed[0].len() * 5 - 1);
    let map = TwoDimMap::new(parsed, end, (0, 0));
    map.find_cheapest_path_cost()
}
