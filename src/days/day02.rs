use core::panic;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day02/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

fn score(opponents_move: Game, my_move: Game) -> u32 {
    let mut score = match my_move {
        Game::Rock => 1,
        Game::Paper => 2,
        Game::Scissors => 3,
    };

    if opponents_move == my_move {
        // a draw
        score += 3;
    } else if match opponents_move {
        Game::Rock => matches!(my_move, Game::Paper),
        Game::Paper => matches!(my_move, Game::Scissors),
        Game::Scissors => matches!(my_move, Game::Rock),
    } {
        score += 6;
    }

    score
}

fn solver(data: &str) -> SolutionPair {
    let mut p1_scores = 0u32;
    let mut p2_scores = 0u32;

    for line in data.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let opponents_move = line.chars().next().unwrap();
        let my_move = line.chars().nth(2).unwrap();

        let opponents_move = match opponents_move {
            'A' => Game::Rock,
            'B' => Game::Paper,
            'C' => Game::Scissors,
            _ => panic!(),
        };

        p1_scores += score(
            opponents_move,
            match my_move {
                'X' => Game::Rock,
                'Y' => Game::Paper,
                'Z' => Game::Scissors,
                _ => panic!(),
            },
        );

        let my_move = match my_move {
            'X' => match opponents_move {
                Game::Rock => Game::Scissors,
                Game::Paper => Game::Rock,
                Game::Scissors => Game::Paper,
            },
            'Y' => opponents_move,
            'Z' => match opponents_move {
                Game::Rock => Game::Paper,
                Game::Paper => Game::Scissors,
                Game::Scissors => Game::Rock,
            },
            _ => panic!(),
        };

        p2_scores += score(opponents_move, my_move);
    }

    (Solution::U32(p1_scores), Solution::U32(p2_scores))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"
A Y
B X
C Z
"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 15);
            assert_eq!(p2, 12);
        } else {
            panic!();
        }
    }
}
