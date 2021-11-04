# Advent of Rust

> Disclaimer: When writing this code, I was rather unfamiliar with the Rust language.
> I probably still am unfamiliar with the language.

Here are some solutions to AoC problems in Rust.
I tried to keep the setup fairly clean, and have decided to opt for test-driven-development to help this effort.
To this end, I have made it so running tests is the method for checking against inputs.

```sh
$ cargo test aoc_template::day_template
   Compiling advent_of_rust v0.1.0 (/path/to/advent-of-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.64s
     Running unittests (target/debug/deps/advent_of_rust-[hash]])

running 2 tests
test aoc_template::day_template::tests::test_part_two ... ok
test aoc_template::day_template::tests::test_part_one ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
```
