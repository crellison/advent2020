use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct BingoSpace {
    x: usize,
    y: usize,
    pub called: bool,
}

impl BingoSpace {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            called: false,
        }
    }

    pub fn call_space(&mut self) {
        self.called = true;
    }

    pub fn loc(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct BingoBoard {
    board: HashMap<u32, BingoSpace>,
    won: bool,
}

impl BingoBoard {
    pub fn new(numbers: &str) -> Self {
        let mut board: HashMap<u32, BingoSpace> = HashMap::new();
        let nums_to_vectors = numbers.split("\n").map(|line| {
            line.split_whitespace()
                .map(|group| group.parse::<u32>().unwrap())
        });
        for (x, row) in nums_to_vectors.enumerate() {
            for (y, num) in row.enumerate() {
                if x > 5 || y > 5 {
                    panic!("Indices out of range for 5x5: {} {}", x, y);
                }
                board.insert(num, BingoSpace::new(x, y));
            }
        }

        Self { board, won: false }
    }

    pub fn call_num(&mut self, num: &u32) -> bool {
        if let Some(entry) = self.board.get_mut(num) {
            entry.call_space();
        }
        self.is_win()
    }

    fn is_win(&self) -> bool {
        let board_spaces: Vec<(usize, usize)> =
            self.board.values().filter(|space| space.called).map(|x| x.loc()).collect();
        let horiz_win = (0..5_usize)
            .any(|x| (0..5_usize).fold(true, |acc, y| acc && board_spaces.contains(&(x, y))));
        let vert_win = (0..5_usize)
            .any(|x| (0..5_usize).fold(true, |acc, y| acc && board_spaces.contains(&(y, x))));
        horiz_win || vert_win
    }

    pub fn sum_unmarked_numbers(&self) -> u32 {
        self.board.keys().filter(|key| !self.board.get(key).unwrap().called).sum()
    }
}

#[allow(dead_code)]
fn part_one(input: &str) -> u32 {
    let mut all_input_data = input.split("\n\n");
    let numbers_called = all_input_data
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap());

    let mut boards: Vec<BingoBoard> = all_input_data.map(|input| BingoBoard::new(input)).collect();

    for number in numbers_called {
        for i in 0..boards.len() {
            if boards[i].call_num(&number) {
                println!("{:?}", boards[i]);
                return boards[i].sum_unmarked_numbers() * number
            }
        }
    }
    0
}

#[allow(dead_code)]
fn part_two(input: &str) -> u32 {
    let mut all_input_data = input.split("\n\n");
    let numbers_called = all_input_data
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap());

    let mut boards: Vec<BingoBoard> = all_input_data.map(|input| BingoBoard::new(input)).collect();
    
    let mut last_win_val: u32 = 0;
    let mut won_boards: HashSet<usize> = HashSet::new();

    for number in numbers_called {
        for i in 0..boards.len() {
            if won_boards.contains(&i) {
                continue;
            }
            if boards[i].call_num(&number) {
                won_boards.insert(i);
                last_win_val = number * boards[i].sum_unmarked_numbers();
            }
        }
    }
    last_win_val
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    use crate::utils::{get_input, InputType};
    use std::io;

    #[test]
    fn test_part_one() -> io::Result<()> {
        assert_eq!(
            part_one(&get_input(2021, 4, InputType::Challenge, 0)?),
            69579
        );
        Ok(())
    }

    #[test]
    fn test_part_two() -> io::Result<()> {
        assert_eq!(
            part_two(&get_input(2021, 4, InputType::Challenge, 0)?),
            1924
        );
        Ok(())
    }
}
