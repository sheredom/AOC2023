use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day01/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn solver(data: &str) -> SolutionPair {
    let mut elves = Vec::new();

    let mut elf = 0u32;

    for line in data.lines() {
        let line = line.trim();

        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<u32>().unwrap();
        }
    }

    elves.push(elf);

    elves.sort();

    let p1 = *elves.last().unwrap();

    let p2 = elves[(elves.len() - 3)..].iter().sum();

    (Solution::U32(p1), Solution::U32(p2))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"
    1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 24000);
            assert_eq!(p2, 45000);
        } else {
            panic!();
        }
    }
}
