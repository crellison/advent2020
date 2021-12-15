use crate::utils::{get_input, InputType};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2021, 15, InputType::Example, 2)?)
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct PointToCheck {
    cost: u32,
    position: (usize, usize),
}

impl Ord for PointToCheck {
    fn cmp(&self, other: &PointToCheck) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PointToCheck {
    fn partial_cmp(&self, other: &PointToCheck) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
struct TwoDimMap {
    map: Vec<Vec<u32>>,
    end: (usize, usize),
    start: (usize, usize),
}

impl TwoDimMap {
    fn new(map: Vec<Vec<u32>>, end: (usize, usize), start: (usize, usize)) -> Self {
        Self { map, end, start }
    }

    fn get_neighbors(&self, point: &(usize, usize)) -> HashSet<(usize, usize)> {
        let mut neighbors: HashSet<(usize, usize)> = HashSet::new();

        if point.0 > 0 {
            neighbors.insert((point.0 - 1, point.1));
        }
        if point.0 < self.end.0 {
            neighbors.insert((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            neighbors.insert((point.0, point.1 - 1));
        }
        if point.1 < self.end.0 {
            neighbors.insert((point.0, point.1 + 1));
        }

        neighbors
    }

    fn get_step_cost(&self, point: &(usize, usize)) -> u32 {
        let dim = (self.map.len(), self.map[0].len());
        let x = point.0 % dim.0;
        let y = point.1 % dim.1;
        let scale_factor = (point.0 / dim.0 + point.1 / dim.1) as u32;
        let new_val = (self.map[x][y] + scale_factor) % 9;
        if new_val == 0 {
            9
        } else {
            new_val
        }
    }

    fn find_cheapest_path_cost(&self) -> u32 {
        if self.start == self.end {
            return 0;
        }

        let expensive_point = PointToCheck {
            cost: u32::MAX,
            position: self.start,
        };
        let mut a_star_costs: HashMap<(usize, usize), PointToCheck> = HashMap::new();
        a_star_costs.insert(
            self.start,
            PointToCheck {
                cost: 0,
                position: self.start,
            },
        );

        let dist_to_end =
            |point: &(usize, usize)| (self.end.0 - point.0 + self.end.1 - point.1) as u32;

        let mut open: BinaryHeap<PointToCheck> = BinaryHeap::new();
        open.push(PointToCheck {
            position: self.start,
            cost: dist_to_end(&self.start),
        });

        let mut cost_to_end = u32::MAX;

        while let Some(PointToCheck {
            cost: _estimated_cost,
            position,
        }) = open.pop()
        {
            let PointToCheck { cost, position: _ } = a_star_costs.get(&position).unwrap().clone();

            if cost > cost_to_end {
                continue;
            }

            for next_point in self.get_neighbors(&position) {
                let next_cost = cost + self.get_step_cost(&next_point);

                if !a_star_costs.contains_key(&next_point)
                    || a_star_costs
                        .get(&next_point)
                        .unwrap_or(&expensive_point)
                        .cost
                        > next_cost
                {
                    a_star_costs.insert(
                        next_point,
                        PointToCheck {
                            cost: next_cost,
                            position,
                        },
                    );
                    open.push(PointToCheck {
                        position: next_point,
                        cost: next_cost + dist_to_end(&next_point),
                    });
                }
            }

            if position == self.end {
                cost_to_end = a_star_costs.get(&self.end).unwrap_or(&expensive_point).cost;
            }
        }

        cost_to_end
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
