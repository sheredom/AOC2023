use itertools::*;
use num::Zero;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve1(input: &str) -> Solution {
    let mut sol = 0;

    for line in input.lines() {
        let mut sequences = Vec::new();

        sequences.push(Vec::new());

        for element in line.split(' ') {
            let element = element.parse::<i32>().unwrap();
            sequences[0].push(element);
        }

        while !sequences.last().unwrap().iter().all(i32::is_zero) {
            let mut sequence = Vec::new();

            for (e, n) in sequences.last().unwrap().iter().tuple_windows() {
                sequence.push(n - e);
            }

            sequences.push(sequence);
        }

        let mut diff = 0;

        for sequence in sequences.iter().rev() {
            let last = sequence.last().unwrap();

            diff += last;
        }

        sol += diff;
    }

    Solution::I32(sol)
}

pub fn solve2(input: &str) -> Solution {
    let mut sol = 0;

    for line in input.lines() {
        let mut sequences = Vec::new();

        sequences.push(Vec::new());

        for element in line.split(' ') {
            let element = element.parse::<i32>().unwrap();
            sequences[0].push(element);
        }

        while !sequences.last().unwrap().iter().all(i32::is_zero) {
            let mut sequence = Vec::new();

            for (e, n) in sequences.last().unwrap().iter().tuple_windows() {
                sequence.push(n - e);
            }

            sequences.push(sequence);
        }

        let mut diff = 0;

        for sequence in sequences.iter().rev() {
            let first = sequence.first().unwrap();

            diff = first - diff;
        }

        sol += diff;
    }

    Solution::I32(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day09"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(super::solve1(input), Solution::I32(114));
        assert_eq!(super::solve2(input), Solution::I32(2));
    }
}
