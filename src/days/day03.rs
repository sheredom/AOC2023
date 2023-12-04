use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve1(input: &str) -> Solution {
    let mut sol = 0;

    let mut len = None;

    let mut board = Vec::new();

    for line in input.lines() {
        if let Some(len) = len {
            assert_eq!(len, line.len() + 2);
        } else {
            // +2 for the edges!
            len = Some(line.len() + 2);

            // Before the first row we spam in a lot of empty tiles ('.').
            board.resize(len.unwrap(), '.');
        }

        // Push a left hand boundary.
        board.push('.');

        for c in line.chars() {
            board.push(c);
        }

        // Push a right hand boundary.
        board.push('.');
    }

    let len = len.unwrap();

    // After the last row we spam in a lot of empty tiles ('.').
    board.resize(board.len() + len, '.');

    // Only loop over the non-boundary tiles.
    for y in 1..(len - 1) {
        for x in 1..(len - 1) {
            let index = y * len + x;

            let tile = board[index];

            // We are only looking for actual symbols here.
            if tile.is_ascii_digit() || '.' == tile {
                continue;
            }

            for other_index in [
                index - len - 1,
                index - len,
                index - len + 1,
                index - 1,
                index + 1,
                index + len - 1,
                index + len,
                index + len + 1,
            ] {
                let other = board[other_index];

                // Only looking for digits here.
                if !other.is_ascii_digit() {
                    continue;
                }

                // We need to go left on the board until we hit a non digit.
                let mut start_index = other_index;

                while board[start_index].is_ascii_digit() {
                    start_index -= 1;
                }

                // We'll be one behind the actual start, so bump now (and un-mut ourselves).
                let start_index = start_index + 1;

                let mut end_index = other_index;

                while board[end_index].is_ascii_digit() {
                    end_index += 1;
                }

                let mut part_number = 0;

                for tile in board.iter_mut().take(end_index).skip(start_index) {
                    part_number *= 10;
                    part_number += tile.to_digit(10).unwrap();

                    // Wipe out the part number so that we don't double count!
                    *tile = '.';
                }

                sol += part_number;
            }
        }
    }

    Solution::U32(sol)
}

pub fn solve2(input: &str) -> Solution {
    let mut len = None;

    let mut board = Vec::new();

    let mut gears = HashMap::new();

    for line in input.lines() {
        if let Some(len) = len {
            assert_eq!(len, line.len() + 2);
        } else {
            // +2 for the edges!
            len = Some(line.len() + 2);

            // Before the first row we spam in a lot of empty tiles ('.').
            board.resize(len.unwrap(), '.');
        }

        // Push a left hand boundary.
        board.push('.');

        for c in line.chars() {
            board.push(c);
        }

        // Push a right hand boundary.
        board.push('.');
    }

    let len = len.unwrap();

    // After the last row we spam in a lot of empty tiles ('.').
    board.resize(board.len() + len, '.');

    // Only loop over the non-boundary tiles.
    for y in 1..(len - 1) {
        for x in 1..(len - 1) {
            let index = y * len + x;

            let tile = board[index];

            // We are only looking for gears.
            if '*' != tile {
                continue;
            }

            for other_index in [
                index - len - 1,
                index - len,
                index - len + 1,
                index - 1,
                index + 1,
                index + len - 1,
                index + len,
                index + len + 1,
            ] {
                let other = board[other_index];

                // Only looking for digits here.
                if !other.is_ascii_digit() {
                    continue;
                }

                // We need to go left on the board until we hit a non digit.
                let mut start_index = other_index;

                while board[start_index].is_ascii_digit() {
                    start_index -= 1;
                }

                // We'll be one behind the actual start, so bump now (and un-mut ourselves).
                let start_index = start_index + 1;

                let mut end_index = other_index;

                while board[end_index].is_ascii_digit() {
                    end_index += 1;
                }

                let mut part_number = 0;

                for tile in board.iter_mut().take(end_index).skip(start_index) {
                    part_number *= 10;
                    part_number += tile.to_digit(10).unwrap();

                    // Wipe out the part number so that we don't double count!
                    *tile = '.';
                }

                gears.entry(index).or_insert_with(Vec::new);

                gears.get_mut(&index).unwrap().push(part_number);
            }
        }
    }

    let mut sol = 0;

    for (_, gear) in gears {
        if 2 != gear.len() {
            continue;
        }

        sol += gear.iter().product::<u32>();
    }

    Solution::U32(sol)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day03"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(super::solve1(input), Solution::U32(4361));
        assert_eq!(super::solve2(input), Solution::U32(467835));
    }
}
