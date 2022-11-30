use advent_of_rust::{rust_2019, rust_2021};
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
            "2" => rust_2019::day_02::main(),
            "3" => rust_2019::day_03::main(),
            "4" => rust_2019::day_04::main(),
            "5" => rust_2019::day_05::main(),
            _ => panic!("unexpected year {}", day),
        },
        "2021" => match day.as_str() {
            "1" => rust_2021::day_01::main(),
            "2" => rust_2021::day_02::main(),
            "3" => rust_2021::day_03::main(),
            "4" => rust_2021::day_04::main(),
            "5" => rust_2021::day_05::main(),
            "6" => rust_2021::day_06::main(),
            "7" => rust_2021::day_07::main(),
            "8" => rust_2021::day_08::main(),
            "9" => rust_2021::day_09::main(),
            "10" => rust_2021::day_10::main(),
            "11" => rust_2021::day_11::main(),
            "12" => rust_2021::day_12::main(),
            "13" => rust_2021::day_13::main(),
            "14" => rust_2021::day_14::main(),
            "15" => rust_2021::day_15::main(),
            "16" => rust_2021::day_16::main(),
            "17" => rust_2021::day_17::main(),
            "18" => rust_2021::day_18::main(),
            "19" => rust_2021::day_19::main(),
            "20" => rust_2021::day_20::main(),
            "21" => rust_2021::day_21::main(),
            "22" => rust_2021::day_22::main(),
            "23" => rust_2021::day_23::main(),
            "24" => rust_2021::day_24::main(),
            "25" => rust_2021::day_25::main(),
            _ => panic!("unexpected year {}", day),
        },
        _ => panic!("unexpected year {}", year),
    }
}
