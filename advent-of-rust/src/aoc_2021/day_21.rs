use memoise::memoise;
use std::io;

pub fn main() -> io::Result<()> {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
    Ok(())
}

fn deterministic_dirac(p1_start: u32, p2_start: u32) -> u32 {
    let target = 1000;
    let (mut p1, mut p2) = (p1_start - 1, p2_start - 1); // handle 1-indexed scores
    let (mut p1_score, mut p2_score) = (0, 0);
    for turn in 0..target * 10 {
        let step = ((turn * 3 + 1) * 3 + 3) % 100;
        if turn % 2 == 0 {
            p1 = (p1 + step) % 10;
            p1_score += p1 + 1; // handle 1-indexed scores
            if p1_score >= target {
                return p2_score * (turn + 1) * 3;
            }
        } else {
            p2 = (p2 + step) % 10;
            p2_score += p2 + 1; // handle 1-indexed scores
            if p2_score >= target {
                return p1_score * (turn + 1) * 3;
            }
        }
    }
    0
}

// score and count of ways
const QUANTUM_SCORES: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[memoise(p1_pos, p1_score, p2_pos, p2_score, p1_turn)]
fn quantum_dirac(p1_pos: u8, p1_score: u8, p2_pos: u8, p2_score: u8, p1_turn: bool) -> (u64, u64) {
    if p1_score >= 21 {
        return (1, 0);
    } else if p2_score >= 21 {
        return (0, 1);
    }

    QUANTUM_SCORES
        .map(|(roll, count)| -> (u64, u64) {
            let win_count: (u64, u64);
            if p1_turn {
                let new_p1 = (p1_pos + roll) % 10;
                win_count = quantum_dirac(new_p1, p1_score + new_p1 + 1, p2_pos, p2_score, false);
            } else {
                let new_p2 = (p2_pos + roll) % 10;
                win_count = quantum_dirac(p1_pos, p1_score, new_p2, p2_score + new_p2 + 1, true);
            }
            (win_count.0 * count, win_count.1 * count)
        })
        .iter()
        .fold((0, 0), |acc, cur| (acc.0 + cur.0, acc.1 + cur.1))
}

fn part_one() -> u32 {
    deterministic_dirac(8, 4)
}

fn part_two() -> u64 {
    let (p1, p2) = (8, 4);
    let win_counts = quantum_dirac(p1 - 1, 0, p2 - 1, 0, true);
    if win_counts.0 > win_counts.1 {
        win_counts.0
    } else {
        win_counts.1
    }
}
