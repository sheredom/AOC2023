use crate::{Solution, SolutionPair};
use std::collections::HashMap;

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day16/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

struct Room {
    flow: usize,
    tunnels: Vec<usize>,
}

enum Action {
    Move,
    Open,
}

fn p1(map: &HashMap<usize, Room>, start: usize) -> usize {
    let mut cache = HashMap::new();
    let result = p1_action((Action::Move, start), map, Vec::new(), &mut cache, 1);
    result
}

fn p2(map: &HashMap<usize, Room>, start: usize) -> usize {
    let mut cache = HashMap::new();
    let result = p1_action((Action::Move, start), map, Vec::new(), &mut cache, 1);
    result
}

fn p1_action(
    (action, node): (Action, usize),
    map: &HashMap<usize, Room>,
    path: Vec<usize>,
    cache: &mut HashMap<(usize, Vec<usize>, usize), usize>,
    minutes: usize,
) -> usize {
    if minutes == 30 {
        return 0;
    }

    if let Some(cached) = cache.get(&(node, path.clone(), minutes)) {
        return *cached;
    }

    let result = match action {
        Action::Move => map[&node]
            .tunnels
            .iter()
            .map(|tunnel| {
                p1_action(
                    (Action::Open, *tunnel),
                    map,
                    path.clone(),
                    cache,
                    minutes + 1,
                )
            })
            .max()
            .unwrap(),
        Action::Open => {
            let can_open = path.iter().find(|n| **n == node).is_none();

            let mut result = p1_action((Action::Move, node), map, path.clone(), cache, minutes);

            if can_open && map[&node].flow != 0 {
                let mut path = path.clone();
                path.push(node);

                let open_result = (30 - minutes) * map[&node].flow
                    + p1_action((Action::Move, node), map, path, cache, minutes + 1);

                result = result.max(open_result);
            }

            result
        }
    };

    cache.insert((node, path.clone(), minutes), result);

    result
}

fn solver(data: &str) -> SolutionPair {
    let mut map = HashMap::new();

    let aa = {
        let mut str_map_index = 0;
        let mut str_map: HashMap<&str, usize> = HashMap::new();

        for line in data.lines() {
            let (valve, remaining) = line.split_once("has flow rate=").unwrap();
            let (flow, remaining) =
                if let Some((f, r)) = remaining.split_once("; tunnels lead to valves ") {
                    (f, r)
                } else {
                    remaining.split_once("; tunnel leads to valve ").unwrap()
                };

            let valve = valve.trim().strip_prefix("Valve ").unwrap();
            let flow = flow.parse::<usize>().unwrap();

            let mut tunnels = Vec::new();

            for tunnel in remaining.split(", ") {
                if !str_map.contains_key(tunnel) {
                    str_map.insert(tunnel, str_map_index);
                    str_map_index += 1;
                }

                tunnels.push(str_map[tunnel]);
            }

            if !str_map.contains_key(valve) {
                str_map.insert(valve, str_map_index);
                str_map_index += 1;
            }

            map.insert(str_map[valve], Room { flow, tunnels });
        }

        str_map["AA"]
    };

    let p1 = p1(&map, aa);
    let p2 = p2(&map, aa);

    (Solution::U64(p1 as u64), Solution::U64(p2 as u64))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U64(p1), Solution::U64(p2)) = solution {
            assert_eq!(p1, 1651);
            assert_eq!(p2, 1707);
        } else {
            panic!();
        }
    }
}
