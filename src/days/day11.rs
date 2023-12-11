use std::collections::HashMap;

use itertools::Itertools;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solver(input: &str, empty_row_col_cost: usize) -> Solution {
    // -1 because we already count the row/col once implicitly.
    let empty_row_col_cost = empty_row_col_cost - 1;

    let mut board = Vec::new();
    let mut width = None;

    for line in input.lines() {
        if let Some(width) = width {
            assert_eq!(width, line.len());
        } else {
            width = Some(line.len());
        }

        for c in line.chars() {
            match c {
                '#' => {
                    board.push(true);
                }
                '.' => {
                    board.push(false);
                }
                _ => unreachable!(),
            }
        }
    }

    let board = board;
    let galaxies: Vec<usize> = board
        .iter()
        .enumerate()
        .filter(|(_, x)| **x)
        .map(|(index, _)| index)
        .collect();
    let width = width.unwrap();

    let mut expanded_rows = Vec::new();

    for row in 0..(board.len() / width) {
        let offset = row * width;
        expanded_rows.push(board[offset..(offset + width)].iter().all(|x| !*x));
    }

    let mut expanded_columns = Vec::new();

    for col in 0..width {
        expanded_columns.push(board[col..].iter().step_by(width).all(|x| !*x));
    }

    let expanded_rows = expanded_rows;
    let expanded_columns = expanded_columns;

    let mut shortest_distances = HashMap::new();

    for (g, o) in galaxies.iter().tuple_combinations() {
        let (gx, gy) = (g % width, g / width);
        let (ox, oy) = (o % width, o / width);

        let distance = gx.abs_diff(ox) + gy.abs_diff(oy);

        let column_expansion = expanded_columns[gx.min(ox)..=gx.max(ox)]
            .iter()
            .filter(|x| **x)
            .count();
        let row_expansion = expanded_rows[gy.min(oy)..=gy.max(oy)]
            .iter()
            .filter(|x| **x)
            .count();

        let distance = (distance + empty_row_col_cost * (column_expansion + row_expansion)) as u64;

        let mut pair = [g, o];
        pair.sort();

        let recorded_distance = shortest_distances.entry(pair).or_insert(distance);

        assert_eq!(*recorded_distance, distance);
    }

    Solution::U64(shortest_distances.values().sum::<u64>())
}

pub fn solve1(input: &str) -> Solution {
    solver(input, 2)
}

pub fn solve2(input: &str) -> Solution {
    solver(input, 1000000)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day11"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(super::solve1(input), Solution::U64(374));
        assert_eq!(super::solver(input, 10), Solution::U64(1030));
        assert_eq!(super::solver(input, 100), Solution::U64(8410));
    }
}
