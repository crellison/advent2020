use std::{collections::HashSet, io, ops::Range};

// const EXAMPLE: (Range<i32>, Range<i32>) = (20..30 + 1, -10..-5 + 1);
const CHALLENGE: (Range<i32>, Range<i32>) = (207..263 + 1, -115..-63 + 1);

pub fn main() -> io::Result<()> {
    println!("part one: {:?}", part_one(CHALLENGE));
    println!("part two: {:?}", part_two(CHALLENGE));
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
    for (_, dy) in probe.hits() {
        let height = dy * (dy + 1) / 2;
        if max < height {
            max = height
        }
    }
    max
}

fn part_two(target: (Range<i32>, Range<i32>)) -> usize {
    let probe = ProbeTarget::new(&target);
    probe.hits().len()
}
