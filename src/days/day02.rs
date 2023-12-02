use core::panic;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

// 12 red cubes, 13 green cubes, and 14 blue cubes max!
fn is_over(line: &str) -> bool {
    for draw in line.split(';') {
        for cubes in draw.split(',') {
            let (number, colour) = cubes.trim().split_once(' ').unwrap();

            let number = number.parse::<i32>().unwrap();

            let colour = match colour {
                "blue" => 14,
                "red" => 12,
                "green" => 13,
                _ => panic!(),
            };

            if number > colour {
                return false;
            }
        }
    }

    true
}

pub fn solve1(input: &str) -> Solution {
    let mut sol = 0;

    for line in input.lines() {
        let line = line.strip_prefix("Game ").unwrap();

        let (id, line) = line.split_once(':').unwrap();

        let id = id.parse::<u32>().unwrap();

        if is_over(line) {
            sol += id;
        }
    }

    Solution::U32(sol)
}

pub fn solve2(input: &str) -> Solution {
    let mut sol = 0;

    for line in input.lines() {
        let line = line.strip_prefix("Game ").unwrap();

        let (_, line) = line.split_once(':').unwrap();

        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for draw in line.split(';') {
            for cubes in draw.split(',') {
                let (number, colour) = cubes.trim().split_once(' ').unwrap();

                let number = number.parse::<u32>().unwrap();

                match colour {
                    "blue" => blue = blue.max(number),
                    "red" => red = red.max(number),
                    "green" => green = green.max(number),
                    _ => panic!(),
                }
            }
        }

        sol += blue * green * red;
    }

    Solution::U32(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day02"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let s1 = super::solve1(input);

        let s2 = super::solve2(input);

        assert_eq!(s1, Solution::U32(8));
        assert_eq!(s2, Solution::U32(2286));
    }
}
