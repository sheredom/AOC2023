use std::collections::VecDeque;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day12/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn solver(data: &str) -> SolutionPair {
    let mut grid = Vec::new();
    let mut width = None;
    let mut p1_start = None;
    let mut p2_starts = Vec::new();
    let mut end = None;

    for line in data.lines() {
        // +2 because we pad the boundary with low walls
        let len = (line.len() + 2) as i32;

        if let Some(width) = width {
            assert_eq!(width, len);
        } else {
            width = Some(len);
        }

        if grid.is_empty() {
            // add a false row at the top
            grid.resize(len as usize, 'z');
        }

        // add a false row to the left
        grid.push('z');

        for char in line.chars() {
            let char = match char {
                'S' => {
                    let len = grid.len() as i32;
                    let width = width.unwrap();
                    p1_start = Some((len % width, len / width));

                    // replace our 'S' with 'a' so that the algorithm later is much easier for processing
                    'a'
                }
                'E' => {
                    let len = grid.len() as i32;
                    let width = width.unwrap();
                    end = Some((len % width, len / width));
                    char
                }
                _ => char,
            };

            if char == 'a' {
                let len = grid.len() as i32;
                let width = width.unwrap();
                p2_starts.push((len % width, len / width));
            }

            grid.push(char);
        }

        // add a false row to the right
        grid.push('z');
    }

    let width = width.unwrap();
    let p1_start = p1_start.unwrap();
    let end = end.unwrap();

    // add a false row at the bottom
    grid.resize(grid.len() + (width as usize), 'z');

    let grid = grid;

    (
        solve_with(&grid, width, &[p1_start], end),
        solve_with(&grid, width, &p2_starts, end),
    )
}

fn solve_with(grid: &Vec<char>, width: i32, starts: &[(i32, i32)], end: (i32, i32)) -> Solution {
    let mut min_steps = Vec::new();
    min_steps.resize(grid.len(), u32::MAX);

    let mut paths = VecDeque::new();

    for start in starts {
        paths.push_back((0u32, *start));
    }

    while let Some(path) = paths.pop_front() {
        let (steps, (x, y)) = path;

        let i = y * width + x;

        if steps >= min_steps[i as usize] {
            // if it wasn't cheaper for us to reach this grid position, bail
            continue;
        }

        min_steps[i as usize] = steps;

        let my_value = grid[i as usize];

        if my_value == 'E' {
            // if we reach the end bail
            continue;
        }

        let my_value = my_value as u32;

        let offsets = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        for (x_offset, y_offset) in offsets {
            if x_offset == 0 && y_offset == 0 {
                // skip ourselves
                continue;
            }

            let explore = (x + x_offset, y + y_offset);
            let explore_i = explore.1 * width + explore.0;

            let explore_value = grid[explore_i as usize];

            let explore_value = if explore_value == 'E' {
                'z'
            } else {
                explore_value
            };

            let explore_value = explore_value as u32;

            if (my_value >= explore_value) || ((my_value + 1) == explore_value) {
                paths.push_back((steps + 1, explore));
            }
        }
    }

    let end_i = end.1 * width + end.0;

    Solution::U32(min_steps[end_i as usize])
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 31);
            assert_eq!(p2, 29);
        } else {
            panic!();
        }
    }
}
