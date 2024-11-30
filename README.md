# Advent of Code 2023

My solutions to the [Advent of Code 2023](https://adventofcode.com/2023) done in Rust, again.

## Running

This year I decided upon using [cargo-aoc](https://github.com/gobanos/cargo-aoc), so follow the `README.md` there for
specifics. In short, use the following commands

```sh
# install the lib
cargo install cargo-aoc

# If input not provided, set your own aoc token to be able to retrieve the input
cargo aoc credentials {token}

# Build the project, get the input of the latest day if not present, and run the latest day
cargo aoc

# Like previous, but -d to run a specific day and optionally -p to run specific part
cargo aoc -d {day} -p {part}

# Run the benchmarks for latest day, optionally add -o flag to automatically open it in your browser
# or run specific day and part with -d and -p respectively
cargo aoc bench
```
