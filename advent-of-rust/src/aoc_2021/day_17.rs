use std::{collections::HashSet, io, ops::Range, time::{Instant, Duration}};

const EXAMPLE: (Range<i32>, Range<i32>) = (20..30 + 1, -10..-5 + 1);
const CHALLENGE: (Range<i32>, Range<i32>) = (207..263 + 1, -115..-63 + 1);
const BIG_CHALLENGE: (Range<i32>, Range<i32>) = (1000..2000 + 1, -2000..-1800 + 1);

pub fn main() -> io::Result<()> {
    println!("part one: {:?}", part_one(CHALLENGE));
    println!("part two: {:?}", part_two(BIG_CHALLENGE));
    Ok(())
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
                break
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
                step * (step + 1) / 2 + (dx-step) * step
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
        let (mut x,mut y) = (0,0);
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

fn part_one(target: (Range<i32>, Range<i32>)) -> i32 {
    let mut max = 0;
    let probe = ProbeTarget::new(&target);
    for (_, dy) in probe.hits_fast() {
        let height = dy * (dy + 1) / 2;
        if max < height {
            max = height
        }
    }
    max
}

fn part_two(target: (Range<i32>, Range<i32>)) -> usize {
    let probe = ProbeTarget::new(&target);

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
