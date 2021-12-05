use std::collections::HashMap;

struct FloorMap {
    walls: HashMap<(i32, i32), i32>,
}

impl FloorMap {
    pub fn new() -> Self {
        Self {
            walls: HashMap::new(),
        }
    }

    pub fn add_wall_unit(&mut self, coord: (i32, i32)) {
        if let Some(height) = self.walls.get_mut(&coord) {
            *height += 1;
        } else {
            self.walls.insert(coord, 1);
        }
    }

    pub fn sum_walls(&self, min_height: i32) -> i32 {
        self.walls
            .values()
            .fold(0, |acc, cur| acc + if cur >= &min_height { 1 } else { 0 })
    }
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut parsed = line.split(" -> ").map(|coord| {
        let mut locations = coord
            .split(",")
            .map(|num: &str| num.parse::<i32>().unwrap());
        (locations.next().unwrap(), locations.next().unwrap())
    });
    (parsed.next().unwrap(), parsed.next().unwrap())
}

fn diff_dir(a: i32, b: i32) -> i32 {
    return if a < b {
        1
    } else if b < a {
        -1
    } else {
        0
    };
}

#[allow(dead_code)]
fn part_one(input: &str) -> i32 {
    let mut walls = FloorMap::new();

    for line in input.lines() {
        let (start, end) = parse_line(line);
        let step = (diff_dir(start.0, end.0), diff_dir(start.1, end.1));
        // skip diagonals
        if step.0 != 0 && step.1 != 0 {
            continue;
        }
        // let mut step_count = 0;
        let mut next_loc = (start.0, start.1);
        while next_loc != (end.0 + step.0, end.1 + step.1) {
            walls.add_wall_unit(next_loc);
            next_loc = (next_loc.0 + step.0, next_loc.1 + step.1);
        }
    }

    walls.sum_walls(2)
}

#[allow(dead_code)]
fn part_two(input: &str) -> i32 {
    let mut walls = FloorMap::new();

    for line in input.lines() {
        let (start, end) = parse_line(line);
        let step = (diff_dir(start.0, end.0), diff_dir(start.1, end.1));
        // let mut step_count = 0;
        let mut next_loc = (start.0, start.1);
        while next_loc != (end.0 + step.0, end.1 + step.1) {
            walls.add_wall_unit(next_loc);
            next_loc = (next_loc.0 + step.0, next_loc.1 + step.1);
        }
    }
    walls.sum_walls(2)
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 5, InputType::Challenge, 0)?),
            5585
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 5, InputType::Challenge, 0)?),
            17193
        );
        Ok(())
    }
}
