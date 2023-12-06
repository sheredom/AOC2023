use itertools::*;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve1(input: &str) -> Solution {
    let (time, distance) = input.lines().tuples().next().unwrap();

    let time = time.strip_prefix("Time:").unwrap();
    let distance = distance.strip_prefix("Distance:").unwrap();

    let time = time.split(' ').filter(|x| !x.is_empty());
    let distance = distance.split(' ').filter(|x| !x.is_empty());

    let mut sol = 1;

    for (time, distance) in time.zip_eq(distance) {
        let time = time.parse::<u32>().unwrap();
        let distance = distance.parse::<u32>().unwrap();

        let mut hits = 0;

        for t in 0..time {
            let total = t * (time - t);

            if total > distance {
                hits += 1;
            }
        }

        sol *= hits;
    }

    Solution::I32(sol)
}

pub fn solve2(input: &str) -> Solution {
    let (time, distance) = input.lines().tuples().next().unwrap();

    let time = time.strip_prefix("Time:").unwrap();
    let distance = distance.strip_prefix("Distance:").unwrap();

    let time = time
        .chars()
        .filter(|x| x.is_ascii_digit())
        .fold(0, |acc, x| (acc * 10) + (x.to_digit(10).unwrap() as u64));
    let distance = distance
        .chars()
        .filter(|x| x.is_ascii_digit())
        .fold(0, |acc, x| (acc * 10) + (x.to_digit(10).unwrap() as u64));

    let mut sol = 1;

    let mut hits = 0;

    for t in 0..time {
        let total = t * (time - t);

        if total > distance {
            hits += 1;
        }
    }

    sol *= hits;

    Solution::I32(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day06"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(super::solve1(input), Solution::I32(288));
        assert_eq!(super::solve2(input), Solution::I32(71503));
    }
}
