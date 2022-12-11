use core::panic;

use itertools::Itertools;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day11/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

#[derive(Debug, Clone)]
enum Operation {
    Squared,
    Multiply(usize),
    Add(usize),
}

#[derive(Debug, Clone)]
struct Monkey {
    inspected: usize,
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    true_monkey: usize,
    false_monkey: usize,
}

fn solver(data: &str) -> SolutionPair {
    let mut monkeys = Vec::new();

    for monkey_str in data.split("Monkey") {
        if monkey_str.trim().is_empty() {
            continue;
        }

        let mut items = Vec::new();
        let mut operation = None;
        let mut test = None;
        let mut true_monkey = None;
        let mut false_monkey = None;

        for line in monkey_str.lines().skip(1) {
            let line = line.trim();

            if let Some(data) = line.strip_prefix("Starting items:") {
                for item in data.split(',') {
                    let item = item.trim();
                    let item = item.parse::<usize>().unwrap();

                    items.push(item);
                }
            } else if let Some(data) = line.strip_prefix("Operation: new = ") {
                if let Some((x, y)) = data.split_once('*') {
                    let x = x.trim();
                    let y = y.trim();

                    assert_eq!(x, "old");

                    if y == "old" {
                        operation = Some(Operation::Squared);
                    } else {
                        let y = y.parse::<usize>().unwrap();

                        operation = Some(Operation::Multiply(y));
                    }
                } else if let Some((x, y)) = data.split_once('+') {
                    let x = x.trim();
                    let y = y.trim();

                    assert_eq!(x, "old");

                    let y = y.parse::<usize>().unwrap();

                    operation = Some(Operation::Add(y));
                } else {
                    panic!();
                }
            } else if let Some(data) = line.strip_prefix("Test: divisible by ") {
                test = Some(data.parse::<usize>().unwrap());
            } else if let Some(data) = line.strip_prefix("If true: throw to monkey ") {
                true_monkey = Some(data.parse::<usize>().unwrap());
            } else if let Some(data) = line.strip_prefix("If false: throw to monkey ") {
                false_monkey = Some(data.parse::<usize>().unwrap());
            }
        }

        let operation = operation.unwrap();
        let test = test.unwrap();
        let true_monkey = true_monkey.unwrap();
        let false_monkey = false_monkey.unwrap();

        let monkey = Monkey {
            inspected: 0,
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
        };

        monkeys.push(monkey);
    }

    (part1(monkeys.clone()), part2(monkeys))
}

fn part1(mut monkeys: Vec<Monkey>) -> Solution {
    for _ in 0..20 {
        for i in 0..(monkeys.len()) {
            let mut pendings = Vec::new();
            pendings.resize(monkeys.len(), Vec::new());

            {
                let monkey = &mut monkeys[i];

                while !monkey.items.is_empty() {
                    let item = monkey.items.remove(0);
                    monkey.inspected += 1;

                    let item = match monkey.operation {
                        Operation::Squared => item * item,
                        Operation::Multiply(constant) => item * constant,
                        Operation::Add(constant) => item + constant,
                    };

                    let item = item / 3;

                    if (item % monkey.test) == 0 {
                        pendings[monkey.true_monkey].push(item);
                    } else {
                        pendings[monkey.false_monkey].push(item);
                    }
                }
            }

            for (index, pending) in pendings.iter_mut().enumerate() {
                monkeys[index].items.append(pending);
            }
        }
    }

    let p1: usize = monkeys
        .iter()
        .map(|item| item.inspected)
        .sorted()
        .rev()
        .take(2)
        .product();

    Solution::U64(p1 as u64)
}

fn part2(mut monkeys: Vec<Monkey>) -> Solution {
    let worry_divider = monkeys.iter().fold(1, |r, m| r * m.test);

    for _ in 0..10000 {
        for i in 0..(monkeys.len()) {
            let mut pendings = Vec::new();
            pendings.resize(monkeys.len(), Vec::new());

            {
                let monkey = &mut monkeys[i];

                while !monkey.items.is_empty() {
                    let item = monkey.items.remove(0);
                    monkey.inspected += 1;

                    let item = match monkey.operation {
                        Operation::Squared => item * item,
                        Operation::Multiply(constant) => item * constant,
                        Operation::Add(constant) => item + constant,
                    };

                    let item = item % worry_divider;

                    if (item % monkey.test) == 0 {
                        pendings[monkey.true_monkey].push(item);
                    } else {
                        pendings[monkey.false_monkey].push(item);
                    }
                }
            }

            for (index, pending) in pendings.iter_mut().enumerate() {
                monkeys[index].items.append(pending);
            }
        }
    }

    let result: usize = monkeys
        .iter()
        .map(|item| item.inspected)
        .sorted()
        .rev()
        .take(2)
        .product();

    Solution::U64(result as u64)
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U64(p1), Solution::U64(p2)) = solution {
            assert_eq!(p1, 10605);
            assert_eq!(p2, 2713310158);
        } else {
            panic!();
        }
    }
}
