🎄🦀🎄 Advent of Code 2023 Solutions 🎄🦀🎄

Here is my Rust-based Advent of Code 2023 solutions.

### Advent of Code project template
A Rust template for Advent of Code that I made to easily run any day or combination of days and measure the execution time.

Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

To run: `cargo run --release [days...]`

Use: `static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/dayXX"));` each day.