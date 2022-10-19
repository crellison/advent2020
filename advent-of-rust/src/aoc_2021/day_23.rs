use crate::utils::{get_input, InputType};
use std::{cmp::Reverse, fmt, io, iter::Rev, ops::Index};

pub fn main() -> io::Result<()> {
    test();
    println!(
        "part one: {}",
        part_one()
    );
    println!(
        "part two: {}",
        part_two(&get_input(2021, 23, InputType::Challenge, 0)?)
    );
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Amphipod {
    EMPTY,
    AMBER,
    BRONZE,
    COPPER,
    DESERT,
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Amphipod::AMBER => "A",
                Amphipod::BRONZE => "B",
                Amphipod::COPPER => "C",
                Amphipod::DESERT => "D",
                Amphipod::EMPTY => ".",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct AmphipodHallway {
    score: usize,
    hallway: [Amphipod; 11],
    amber_home: [Amphipod; 2],
    bronze_home: [Amphipod; 2],
    copper_home: [Amphipod; 2],
    desert_home: [Amphipod; 2],
}

impl AmphipodHallway {
    const ILLEGAL_INDICES: [usize; 4] = [2, 4, 6, 8];
    const HOME_DEPTH: usize = 2;

    fn new(
        amber_home: [Amphipod; Self::HOME_DEPTH],
        bronze_home: [Amphipod; Self::HOME_DEPTH],
        copper_home: [Amphipod; Self::HOME_DEPTH],
        desert_home: [Amphipod; Self::HOME_DEPTH],
    ) -> Self {
        Self {
            score: 0,
            hallway: [Amphipod::EMPTY; 11],
            amber_home,
            bronze_home,
            copper_home,
            desert_home,
        }
    }

    fn set_home(&mut self, home_type: Amphipod, home: [Amphipod; Self::HOME_DEPTH]) {
        match home_type {
            Amphipod::AMBER => self.amber_home = home,
            Amphipod::BRONZE => self.bronze_home = home,
            Amphipod::COPPER => self.copper_home = home,
            Amphipod::DESERT => self.desert_home = home,
            Amphipod::EMPTY => panic!("Requested nonexistent home!"),
        };
    }

    fn is_solved(&self) -> bool {
        self.amber_home
            .iter()
            .all(|creature| creature == &Amphipod::AMBER)
            && self
                .bronze_home
                .iter()
                .all(|creature| creature == &Amphipod::BRONZE)
            && self
                .copper_home
                .iter()
                .all(|creature| creature == &Amphipod::COPPER)
            && self
                .desert_home
                .iter()
                .all(|creature| creature == &Amphipod::DESERT)
    }

    fn increment_score(&mut self, steps: usize, amphipod: Amphipod) {
        self.score += steps
            * match amphipod {
                Amphipod::AMBER => 1,
                Amphipod::BRONZE => 10,
                Amphipod::COPPER => 100,
                Amphipod::DESERT => 1000,
                Amphipod::EMPTY => panic!("Tried to move empty space!"),
            }
    }

    fn get_home_and_exit(&self, amphipod: Amphipod) -> ([Amphipod; 2], usize) {
        let (home, exit_index) = match amphipod {
            Amphipod::AMBER => (self.amber_home, Self::ILLEGAL_INDICES[0]),
            Amphipod::BRONZE => (self.bronze_home, Self::ILLEGAL_INDICES[1]),
            Amphipod::COPPER => (self.copper_home, Self::ILLEGAL_INDICES[2]),
            Amphipod::DESERT => (self.desert_home, Self::ILLEGAL_INDICES[3]),
            Amphipod::EMPTY => panic!("Requested nonexistent home!"),
        };

        (home, exit_index)
    }

    fn valid_moves_from_home(&self, home_type: Amphipod) -> Vec<usize> {
        let (home, exit_index) = self.get_home_and_exit(home_type);

        let mut valid_moves = vec![];

        // no valid moves if its empty or only of correct type
        if home
            .iter()
            .all(|creature| creature == &Amphipod::EMPTY || creature == &home_type)
        {
            return valid_moves;
        }

        // indices before
        for i in (0..exit_index).rev() {
            if self.hallway[i] != Amphipod::EMPTY {
                break;
            }
            if !Self::ILLEGAL_INDICES.contains(&i) {
                valid_moves.push(i);
            }
        }
        // indices after
        for i in (exit_index + 1..self.hallway.len() - 1).rev() {
            if self.hallway[i] != Amphipod::EMPTY {
                break;
            }
            if !Self::ILLEGAL_INDICES.contains(&i) {
                valid_moves.push(i);
            }
        }

        valid_moves
    }

    fn occupied_hallway_spaces(&self) -> Vec<usize> {
        let mut spaces = vec![];
        for (i, amphipod) in self.hallway.iter().enumerate() {
            if amphipod != &Amphipod::EMPTY {
                spaces.push(i);
            }
        }
        spaces
    }

    fn can_move_home(&self, index: usize) -> bool {
        if self.hallway[index] == Amphipod::EMPTY {
            return false;
        }
        let (home, exit_index) = self.get_home_and_exit(self.hallway[index]);

        let mut hallway_path = if index < exit_index {
            index + 1..exit_index + 1
        } else {
            exit_index..index
        };
        let path_empty = hallway_path.all(|i| self.hallway[i] == Amphipod::EMPTY);
        let home_ready = home
            .iter()
            .all(|animal| animal == &Amphipod::EMPTY || animal == &self.hallway[index]);

        path_empty && home_ready
    }

    fn is_gridlocked(&self) -> bool {
        self.valid_moves_from_home(Amphipod::AMBER).len() == 0
            && self.valid_moves_from_home(Amphipod::BRONZE).len() == 0
            && self.valid_moves_from_home(Amphipod::COPPER).len() == 0
            && self.valid_moves_from_home(Amphipod::DESERT).len() == 0
            && self
                .occupied_hallway_spaces()
                .iter()
                .all(|space| !self.can_move_home(*space))
    }

    fn leave_home(&mut self, home_type: Amphipod, location: usize) -> bool {
        if !self.valid_moves_from_home(home_type).contains(&location) {
            return false;
        }
        let (mut home, exit_loc) = self.get_home_and_exit(home_type);
        let home_offset = home.iter().position(|a| a != &Amphipod::EMPTY).unwrap_or(0);
        let moved_amphipod = home[home_offset];
        home[home_offset] = Amphipod::EMPTY;
        self.hallway[location] = moved_amphipod;
        self.set_home(home_type, home);
        let hallway_spaces = if location > exit_loc {
            location - exit_loc
        } else {
            exit_loc - location
        } + 1;

        self.increment_score(home_offset + hallway_spaces, moved_amphipod);

        true
    }

    fn return_home(&mut self, location: usize) -> bool {
        if !self.can_move_home(location) {
            return false;
        }
        let amphipod = self.hallway[location];
        let (mut home, exit_loc) = self.get_home_and_exit(amphipod);
        
        let home_offset = home.iter().position(|a| a != &Amphipod::EMPTY).unwrap_or(Self::HOME_DEPTH) - 1;
        home[home_offset] = amphipod;
        self.hallway[location] = Amphipod::EMPTY;
        self.set_home(amphipod, home);
        let hallway_spaces = if location > exit_loc {
            location - exit_loc
        } else {
            exit_loc - location
        } + 1;

        self.increment_score(home_offset + hallway_spaces, amphipod);

        true
    }

    fn print_board(&self) {
        println!("{}", vec!["#"; self.hallway.len() + 2].join(""));

        print!("#");
        print!(
            "{}",
            self.hallway.map(|amphipod| amphipod.to_string()).join("")
        );
        println!("#");

        for i in 0..Self::HOME_DEPTH {
            println!(
                "###{}#{}#{}#{}###",
                self.amber_home[i], self.bronze_home[i], self.copper_home[i], self.desert_home[i]
            );
        }

        println!("{}", vec!["#"; self.hallway.len() + 2].join(""));
    }

    fn find_min_score(&self) -> usize {
        if self.is_solved() {
            return self.score;
        } else if self.is_gridlocked() || self.score > 16000 {
            return usize::MAX;
        }
        let home_moves = [
            Amphipod::AMBER,
            Amphipod::BRONZE,
            Amphipod::COPPER,
            Amphipod::DESERT,
        ]
        .map(|amphipod| {
            let possible_moves = self.valid_moves_from_home(amphipod);
            let new_boards = possible_moves.iter().map(move |space| {
                let mut next_board = self.clone();
                next_board.leave_home(amphipod, *space);
                next_board
            });
            new_boards.collect::<Vec<_>>()
        });
        let hallway_moves = self.occupied_hallway_spaces();
        let mut possible_hallways: Vec<AmphipodHallway> = Vec::new();

        home_moves.iter().for_each(|move_set| {
            possible_hallways.extend(move_set);
        });

        for index in hallway_moves {
            if self.can_move_home(index) {
                let mut next_hallway = self.clone();
                next_hallway.return_home(index);
                possible_hallways.push(next_hallway);
            }
        }

        // println!("Possible choices");
        // for h in &possible_hallways {
        //     println!();
        //     h.print_board();
        //     println!();
        // }

        possible_hallways.iter().map(|hallway| hallway.find_min_score()).min().unwrap_or(0)
    }
}

fn test() {
    let mut game = AmphipodHallway::new(
        [Amphipod::BRONZE, Amphipod::AMBER],
        [Amphipod::AMBER, Amphipod::BRONZE],
        [Amphipod::COPPER, Amphipod::COPPER],
        [Amphipod::DESERT, Amphipod::DESERT],
    );
    println!("game solved? {}", game.is_solved());
    println!("game gridlocked? {}", game.is_gridlocked());
    game.print_board();
    // game.leave_home(Amphipod::AMBER, 1);
    // game.leave_home(Amphipod::AMBER, 3);
    // game.print_board();
    println!("min score is {}", game.find_min_score());
}

fn part_one() -> usize {
    let game = AmphipodHallway::new(
        [Amphipod::DESERT, Amphipod::COPPER],
        [Amphipod::BRONZE, Amphipod::COPPER],
        [Amphipod::BRONZE, Amphipod::DESERT],
        [Amphipod::AMBER, Amphipod::AMBER],
    );
    game.find_min_score()
    // 0
}

fn part_two(input: &str) -> &str {
    input
}
