use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve1(input: &str) -> Solution {
    let mut sol1 = 0;

    for line in input.lines() {
        println!("{}", line);

        let first = line.chars().find(|x| x.is_ascii_digit()).unwrap();
        let last = line.chars().rev().find(|x| x.is_ascii_digit()).unwrap();

        let v = (first.to_digit(10).unwrap() * 10) + last.to_digit(10).unwrap();

        sol1 += v;
    }

    Solution::U32(sol1)
}

pub fn solve2(input: &str) -> Solution {
    let mut sol2 = 0;

    for line in input.lines() {
        let variants = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let first_index = variants.iter().fold(usize::MAX, |acc, x| {
            acc.min(line.find(x).unwrap_or(usize::MAX))
        });
        let first_index = first_index.min(
            line.find(|x: char| x.is_ascii_digit())
                .unwrap_or(usize::MAX),
        );

        let last_index = variants.iter().fold(usize::MIN, |acc, x| {
            acc.max(line.rfind(x).unwrap_or(usize::MIN))
        });
        let last_index = last_index.max(
            line.rfind(|x: char| x.is_ascii_digit())
                .unwrap_or(usize::MIN),
        );

        let parser = |x: &str| {
            println!("Parser found '{}'", x);

            if x.starts_with("one") {
                1
            } else if x.starts_with("two") {
                2
            } else if x.starts_with("three") {
                3
            } else if x.starts_with("four") {
                4
            } else if x.starts_with("five") {
                5
            } else if x.starts_with("six") {
                6
            } else if x.starts_with("seven") {
                7
            } else if x.starts_with("eight") {
                8
            } else if x.starts_with("nine") {
                9
            } else {
                x.chars().next().unwrap().to_digit(10).unwrap()
            }
        };

        let first = parser(&line[first_index..]);
        let last = parser(&line[last_index..]);

        let v = (first * 10) + last;

        sol2 += v;
    }

    Solution::U32(sol2)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day01"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let r = super::solve2(
            r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(r, Solution::U32(281));
    }
}
