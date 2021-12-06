fn count_fish_after_days(fish_timer: i32, days: i32) -> i64 {
    if days <= fish_timer {
        return 1;
    }
    let days_left = days - fish_timer;
    return count_fish_after_days(7, days_left) + count_fish_after_days(9, days_left);
}

fn count_fish_after_days_fast(mut fish_pop: [i64; 9], days: i32) -> i64 {
    let mut next_fish_pop: [i64; 9];
    for _ in 0..days {
        next_fish_pop = [0_i64; 9];
        for (fish_life, count) in fish_pop.iter().enumerate() {
            if fish_life == 0 {
                next_fish_pop[6] += count;
                next_fish_pop[8] += count;
            } else {
                next_fish_pop[fish_life-1] += count;
            }
        }

        fish_pop = next_fish_pop;
    }

    fish_pop.iter().sum()
}

fn count_fish_from_input(input: &str, days: i32) -> i64 {
    input
        .split(",")
        .map(|d| {
            if let Ok(fish_state) = d.trim_end().parse::<i32>() {
                return count_fish_after_days(fish_state, days);
            }
            0
        })
        .sum()
}

#[allow(dead_code)]
fn part_one(input: &str) -> i64 {
    count_fish_from_input(input, 80)
}

#[allow(dead_code)]
fn part_two(input: &str) -> i64 {
    let mut fish_pop = [0_i64; 9];
    input.split(",").for_each(| initial_fish| {
        if let Ok(fish_state) = initial_fish.trim_end().parse::<usize>() {
            fish_pop[fish_state] += 1;
        }
    });

    count_fish_after_days_fast(fish_pop, 256)
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::{
        aoc_2021::day_06::count_fish_after_days,
        utils::{get_input, InputType},
    };
    use std::io;

    #[test]
    fn test_count_fish_after_days() -> io::Result<()> {
        assert_eq!(count_fish_after_days(1, 1), 1);
        assert_eq!(count_fish_after_days(1, 2), 2);
        assert_eq!(count_fish_after_days(3, 11), 3);
        assert_eq!(count_fish_after_days(3, 14), 4);
        assert_eq!(count_fish_after_days(3, 18), 5);
        Ok(())
    }

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 6, InputType::Challenge, 0)?),
            371379
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(part_two(&get_input(2021, 6, InputType::Challenge, 0)?), 1674303997472);
        Ok(())
    }
}
