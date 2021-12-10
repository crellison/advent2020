use advent_of_rust::{aoc_2019, aoc_2021};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <year> <day>");
        return Ok(());
    }

    let year = &args[1];
    let day = &args[2];

    match year.as_str() {
        "2019" => match day.as_str() {
            "2" => aoc_2019::day_02::main(),
            "3" => aoc_2019::day_03::main(),
            "4" => aoc_2019::day_04::main(),
            "5" => aoc_2019::day_05::main(),
            _ => panic!("unexpected year {}", day),
        },
        "2021" => match day.as_str() {
            "1" => aoc_2021::day_01::main(),
            "2" => aoc_2021::day_02::main(),
            "3" => aoc_2021::day_03::main(),
            "4" => aoc_2021::day_04::main(),
            "5" => aoc_2021::day_05::main(),
            "6" => aoc_2021::day_06::main(),
            "7" => aoc_2021::day_07::main(),
            "8" => aoc_2021::day_08::main(),
            "9" => aoc_2021::day_09::main(),
            "10" => aoc_2021::day_10::main(),
            "11" => aoc_2021::day_11::main(),
            "12" => aoc_2021::day_12::main(),
            "13" => aoc_2021::day_13::main(),
            "14" => aoc_2021::day_14::main(),
            "15" => aoc_2021::day_15::main(),
            "16" => aoc_2021::day_16::main(),
            "17" => aoc_2021::day_17::main(),
            "18" => aoc_2021::day_18::main(),
            "19" => aoc_2021::day_19::main(),
            "20" => aoc_2021::day_20::main(),
            "21" => aoc_2021::day_21::main(),
            "22" => aoc_2021::day_22::main(),
            "23" => aoc_2021::day_23::main(),
            "24" => aoc_2021::day_24::main(),
            "25" => aoc_2021::day_25::main(),
            _ => panic!("unexpected year {}", day),
        },
        _ => panic!("unexpected year {}", year),
    }
}
