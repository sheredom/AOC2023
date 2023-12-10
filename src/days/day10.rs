use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Pipe {
    Start,
    None,
    LeftAndRight,
    UpAndDown,
    LeftAndUp,
    LeftAndDown,
    RightAndUp,
    RightAndDown,
}

impl Pipe {
    fn is_up(&self) -> bool {
        matches!(self, Pipe::UpAndDown | Pipe::LeftAndUp | Pipe::RightAndUp)
    }

    fn is_down(&self) -> bool {
        matches!(
            self,
            Pipe::UpAndDown | Pipe::LeftAndDown | Pipe::RightAndDown
        )
    }

    fn is_left(&self) -> bool {
        matches!(
            self,
            Pipe::LeftAndRight | Pipe::LeftAndUp | Pipe::LeftAndDown
        )
    }

    fn is_right(&self) -> bool {
        matches!(
            self,
            Pipe::LeftAndRight | Pipe::RightAndUp | Pipe::RightAndDown
        )
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '-' => Self::LeftAndRight,
            '|' => Self::UpAndDown,
            'L' => Self::RightAndUp,
            'J' => Self::LeftAndUp,
            '7' => Self::LeftAndDown,
            'F' => Self::RightAndDown,
            'S' => Self::Start,
            '.' => Self::None,
            x => panic!("{}", x),
        }
    }
}

pub fn solve1(input: &str) -> Solution {
    let mut board = Vec::new();
    let mut width = None;
    let mut start = None;

    for line in input.lines() {
        if let Some(width) = width {
            assert_eq!(width, line.len());
        } else {
            width = Some(line.len());
        }

        for c in line.chars() {
            let pipe = Pipe::from(c);

            if Pipe::Start == pipe {
                assert!(start.is_none());
                start = Some(board.len());
            }

            board.push(pipe);
        }
    }

    let width = width.unwrap();
    let start = start.unwrap();

    // Replace start with the pipe it is underneath!
    let (x, y) = (start % width, start / width);

    let mut positions = Vec::new();

    // If there is a pipe coming from the left (that pipe will be a right!).
    if (0 != x) && board[(y * width) + (x - 1)].is_right() {
        positions.push((Direction::Left, x - 1, y));
    }

    // If there is a pipe coming from the right (that pipe will be a left!).
    if ((width - 1) != x) && board[(y * width) + (x + 1)].is_left() {
        positions.push((Direction::Right, x + 1, y));
    }

    // If there is a pipe coming from up (that pipe will be a down!).
    if (0 != y) && board[((y - 1) * width) + x].is_down() {
        positions.push((Direction::Up, x, y - 1));
    }

    // If there is a pipe coming from down (that pipe will be an up!).
    if ((width - 1) != y) && board[((y + 1) * width) + x].is_up() {
        positions.push((Direction::Down, x, y + 1));
    }

    assert_eq!(2, positions.len());

    let mut steps = 1;

    // While the positions do not match, we need to keep calculating.
    while (positions[0].1 != positions[1].1) || (positions[0].2 != positions[1].2) {
        for position in positions.iter_mut() {
            let direction = position.0;
            let x = position.1;
            let y = position.2;

            let index = (y * width) + x;

            *position = match direction {
                Direction::Up => match board[index] {
                    Pipe::UpAndDown => (Direction::Up, x, y - 1),
                    Pipe::LeftAndDown => (Direction::Left, x - 1, y),
                    Pipe::RightAndDown => (Direction::Right, x + 1, y),
                    x => panic!("{:?}", x),
                },
                Direction::Down => match board[index] {
                    Pipe::UpAndDown => (Direction::Down, x, y + 1),
                    Pipe::LeftAndUp => (Direction::Left, x - 1, y),
                    Pipe::RightAndUp => (Direction::Right, x + 1, y),
                    x => panic!("{:?}", x),
                },
                Direction::Left => match board[index] {
                    Pipe::LeftAndRight => (Direction::Left, x - 1, y),
                    Pipe::RightAndUp => (Direction::Up, x, y - 1),
                    Pipe::RightAndDown => (Direction::Down, x, y + 1),
                    x => panic!("{:?}", x),
                },
                Direction::Right => match board[index] {
                    Pipe::LeftAndRight => (Direction::Right, x + 1, y),
                    Pipe::LeftAndUp => (Direction::Up, x, y - 1),
                    Pipe::LeftAndDown => (Direction::Down, x, y + 1),
                    x => panic!("{:?}", x),
                },
            };
        }

        steps += 1;
    }

    Solution::U32(steps)
}

pub fn solve2(input: &str) -> Solution {
    let mut board = Vec::new();
    let mut width = None;
    let mut start = None;

    for line in input.lines() {
        if let Some(width) = width {
            assert_eq!(width, line.len());
        } else {
            width = Some(line.len());
        }

        for c in line.chars() {
            let pipe = Pipe::from(c);

            if Pipe::Start == pipe {
                assert!(start.is_none());
                start = Some(board.len());
            }

            board.push(pipe);
        }
    }

    let width = width.unwrap();
    let start = start.unwrap();

    // Replace start with the pipe it is underneath!
    let (x, y) = (start % width, start / width);

    let mut positions = Vec::new();

    // If there is a pipe coming from the left (that pipe will be a right!).
    if (0 != x) && board[(y * width) + (x - 1)].is_right() {
        positions.push((Direction::Left, x - 1, y));
    }

    // If there is a pipe coming from the right (that pipe will be a left!).
    if ((width - 1) != x) && board[(y * width) + (x + 1)].is_left() {
        positions.push((Direction::Right, x + 1, y));
    }

    // If there is a pipe coming from up (that pipe will be a down!).
    if (0 != y) && board[((y - 1) * width) + x].is_down() {
        positions.push((Direction::Up, x, y - 1));
    }

    // If there is a pipe coming from down (that pipe will be an up!).
    if ((width - 1) != y) && board[((y + 1) * width) + x].is_up() {
        positions.push((Direction::Down, x, y + 1));
    }

    assert_eq!(2, positions.len());

    positions.sort();

    board[start] = match positions[0].0 {
        Direction::Up => match positions[1].0 {
            Direction::Down => Pipe::UpAndDown,
            Direction::Left => Pipe::LeftAndUp,
            Direction::Right => Pipe::RightAndUp,
            _ => unreachable!(),
        },
        Direction::Down => match positions[1].0 {
            Direction::Left => Pipe::LeftAndDown,
            Direction::Right => Pipe::RightAndDown,
            _ => unreachable!(),
        },
        Direction::Left => match positions[1].0 {
            Direction::Right => Pipe::LeftAndRight,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    let mut main_loop = Vec::new();
    main_loop.resize(board.len(), false);
    main_loop[start] = true;

    // While the positions do not match, we need to keep calculating.
    let mut position = positions[0];

    loop {
        let direction = position.0;
        let x = position.1;
        let y = position.2;

        let index = (y * width) + x;

        if main_loop[index] {
            break;
        }

        let pipe = board[index];

        main_loop[index] = true;

        position = match direction {
            Direction::Up => match pipe {
                Pipe::UpAndDown => (Direction::Up, x, y - 1),
                Pipe::LeftAndDown => (Direction::Left, x - 1, y),
                Pipe::RightAndDown => (Direction::Right, x + 1, y),
                x => panic!("{:?}", x),
            },
            Direction::Down => match pipe {
                Pipe::UpAndDown => (Direction::Down, x, y + 1),
                Pipe::LeftAndUp => (Direction::Left, x - 1, y),
                Pipe::RightAndUp => (Direction::Right, x + 1, y),
                x => panic!("{:?}", x),
            },
            Direction::Left => match pipe {
                Pipe::LeftAndRight => (Direction::Left, x - 1, y),
                Pipe::RightAndUp => (Direction::Up, x, y - 1),
                Pipe::RightAndDown => (Direction::Down, x, y + 1),
                x => panic!("{:?}", x),
            },
            Direction::Right => match pipe {
                Pipe::LeftAndRight => (Direction::Right, x + 1, y),
                Pipe::LeftAndUp => (Direction::Up, x, y - 1),
                Pipe::LeftAndDown => (Direction::Down, x, y + 1),
                x => panic!("{:?}", x),
            },
        };
    }

    let mut inside_count = 0;

    for y in 0..(board.len() / width) {
        let mut outside = true;

        for x in 0..width {
            let index = y * width + x;

            if main_loop[index] {
                if matches!(
                    board[index],
                    Pipe::UpAndDown | Pipe::LeftAndDown | Pipe::RightAndDown
                ) {
                    outside = !outside;
                }
            } else if !outside {
                inside_count += 1;
            }
        }
    }

    Solution::U32(inside_count)
}

pub fn solve() -> SolutionPair {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day10"));

    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let input = r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(super::solve1(input), Solution::U32(4));

        let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(super::solve1(input), Solution::U32(8));

        let input = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(super::solve2(input), Solution::U32(4));

        let input = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(super::solve2(input), Solution::U32(4));

        let input = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(super::solve2(input), Solution::U32(8));

        let input = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(super::solve2(input), Solution::U32(10));
    }
}
