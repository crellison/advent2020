use std::{
    collections::HashSet,
    io,
    ops::Range,
    time::{Duration, Instant},
};

use regex::Regex;

use crate::utils::{get_input, InputType};

pub fn main() -> io::Result<()> {
    let input = &get_input(2021, 17, InputType::Challenge, 0)?;
    let target = parse_input(input);
    println!("part one: {:?}", part_one(&target));
    println!("part two: {:?}", part_two(&target));
    Ok(())
}

fn parse_input(input: &str) -> (Range<i32>, Range<i32>) {
    let x_re = Regex::new(r"x=(-?\d+)..(-?\d+)").unwrap();
    let y_re = Regex::new(r"y=(-?\d+)..(-?\d+)").unwrap();

    let mut x_target = 0..0;
    let mut y_target = 0..0;
    if let (Some(x_cap), Some(y_cap)) = (x_re.captures(input), y_re.captures(input)) {
        if let (Ok(x_min), Ok(x_max)) = (x_cap[1].parse::<i32>(), x_cap[2].parse::<i32>()) {
            x_target = x_min..x_max + 1;
        }
        if let (Ok(y_min), Ok(y_max)) = (y_cap[1].parse::<i32>(), y_cap[2].parse::<i32>()) {
            y_target = y_min..y_max + 1;
        }
    }

    (x_target, y_target)
}

#[derive(Debug, Default)]
struct ProbeTarget {
    y_target: Range<i32>,
    x_target: Range<i32>,
}

impl ProbeTarget {
    fn new(target: &(Range<i32>, Range<i32>)) -> Self {
        Self {
            x_target: target.0.clone(),
            y_target: target.1.clone(),
        }
    }

    fn possible_steps(&self, dy: i32) -> HashSet<i32> {
        let mut steps = HashSet::new();
        let (mut y, mut cur_dy) = (0, dy);
        for step in 0.. {
            if y < self.y_target.start {
                break;
            }

            if self.y_target.contains(&y) {
                steps.insert(step);
            }
            y += cur_dy;
            cur_dy -= 1;
        }
        steps
    }

    fn possible_dx(&self, step: i32) -> HashSet<i32> {
        let mut dx_values = HashSet::new();
        for dx in 0..self.x_target.end {
            let x = if step >= dx {
                dx * (dx + 1) / 2
            } else {
                step * (step + 1) / 2 + (dx - step) * step
            };

            if self.x_target.contains(&x) {
                dx_values.insert(dx);
            }
        }
        dx_values
    }

    fn hits_fast(&self) -> HashSet<(i32, i32)> {
        let mut hits = HashSet::new();
        for dy in self.y_target.start..(-self.y_target.start) {
            for step in self.possible_steps(dy) {
                for dx in self.possible_dx(step) {
                    hits.insert((dx, dy));
                }
            }
        }
        hits
    }

    fn hits(&self) -> HashSet<(i32, i32)> {
        let mut hits: HashSet<(i32, i32)> = HashSet::new();
        for dy in self.y_target.start..(-self.y_target.start) {
            for dx in 0..self.x_target.end {
                if self.will_hit(dx, dy) {
                    hits.insert((dx, dy));
                }
            }
        }
        hits
    }

    fn will_hit(&self, initial_dx: i32, initial_dy: i32) -> bool {
        let (mut x, mut y) = (0, 0);
        let (mut dx, mut dy) = (initial_dx, initial_dy);

        while x < self.x_target.end && y > self.y_target.start {
            if self.x_target.contains(&x) && self.y_target.contains(&y) {
                return true;
            }
            x += dx;
            y += dy;
            dx = if dx > 0 { dx - 1 } else { 0 };
            dy -= 1;
        }

        self.x_target.contains(&x) && self.y_target.contains(&y)
    }
}

fn part_one(target: &(Range<i32>, Range<i32>)) -> i32 {
    let mut max = 0;
    let probe = ProbeTarget::new(target);
    for (_, dy) in probe.hits_fast() {
        let height = dy * (dy + 1) / 2;
        if max < height {
            max = height
        }
    }
    max
}

fn part_two(target: &(Range<i32>, Range<i32>)) -> usize {
    let probe = ProbeTarget::new(target);

    let mut elapsed: Duration;
    let mut now = Instant::now();
    probe.hits();
    elapsed = now.elapsed();
    println!("took {} micros with hits", elapsed.as_micros());

    now = Instant::now();
    probe.hits_fast();
    elapsed = now.elapsed();
    println!("took {} micros with hits_fast", elapsed.as_micros());

    probe.hits_fast().len()
}
