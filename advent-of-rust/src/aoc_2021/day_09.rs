use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        map.push(
            line.chars()
                .map(|c| {
                    if let Some(num) = c.to_digit(10) {
                        return num;
                    }
                    panic!("Unable to parse {} into digit", c);
                })
                .collect(),
        );
    }
    map
}

fn get_local_minima(map: &Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let (x_max, y_max) = (map.len() - 1, map[0].len() - 1);
    let mut local_minima: HashSet<(usize, usize)> = HashSet::new();
    for (x, vector) in map.iter().enumerate() {
        for (y, height) in vector.iter().enumerate() {
            let is_minima = DIRECTIONS.iter().all(|(dx, dy)| {
                let (next_x, next_y): (i32, i32) = ((x as i32) + dx, (y as i32) + dy);
                if next_x < 0 || next_x > x_max as i32 || next_y < 0 || next_y > y_max as i32 {
                    return true;
                }

                map[next_x as usize][next_y as usize] > *height
            });

            if is_minima {
                local_minima.insert((x, y));
            }
        }
    }
    local_minima
}

fn get_basin_size(map: &Vec<Vec<u32>>, start_x: usize, start_y: usize) -> usize {
    let mut checked_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = vec![(start_x, start_y)];
    let (x_max, y_max) = (map.len() - 1, map[0].len() - 1);

    while queue.len() > 0 {
        if let Some((x, y)) = queue.pop() {
            if checked_locations.contains(&(x, y)) {
                continue;
            }
            checked_locations.insert((x, y));

            DIRECTIONS.iter().for_each(|(dx, dy)| {
                let (next_x, next_y): (i32, i32) = ((x as i32) + dx, (y as i32) + dy);
                if next_x < 0 || next_x > x_max as i32 || next_y < 0 || next_y > y_max as i32 {
                    return;
                }
                if map[next_x as usize][next_y as usize] != 9 {
                    queue.push((next_x as usize, next_y as usize));
                }
            });
        }
    }
    checked_locations.len()
}

#[allow(dead_code)]
fn part_one(input: &str) -> u32 {
    let map = parse_input(input);
    let local_minima = get_local_minima(&map);
    let local_minima_sum = local_minima
        .iter()
        .fold(0, |acc, (x, y)| acc + map[*x][*y] + 1);
    local_minima_sum
}

#[allow(dead_code)]
fn part_two(input: &str) -> usize {
    let map = parse_input(input);
    let local_minima = get_local_minima(&map);
    let mut basin_sizes: Vec<usize> = local_minima
        .iter()
        .map(|(x, y)| get_basin_size(&map, *x, *y))
        .collect();
    let basin_count = basin_sizes.len();
    basin_sizes.sort();

    basin_sizes[basin_count - 1] * basin_sizes[basin_count - 2] * basin_sizes[basin_count - 3]
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(part_one(&get_input(2021, 9, InputType::Challenge, 0)?), 439);
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 9, InputType::Challenge, 0)?),
            900900
        );
        Ok(())
    }
}
