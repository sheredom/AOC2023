use itertools::{EitherOrBoth, Itertools};
use slab_tree::{NodeId, Tree};
use std::{cmp::Ordering, fmt::Write};

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day13/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

fn parse(p: &str) -> Tree<Option<u8>> {
    let mut tree = Tree::new();
    let root = tree.set_root(None);

    let p = p.strip_prefix('[').unwrap().strip_suffix(']').unwrap();

    parse_into(&mut tree, root, p);

    tree
}

fn parse_into<'a>(tree: &mut Tree<Option<u8>>, node: NodeId, mut p: &'a str) -> &'a str {
    loop {
        if p.is_empty() {
            return p;
        } else if let Some(remaining_p) = p.strip_prefix('[') {
            let child = tree.get_mut(node).unwrap().append(None).node_id();
            p = parse_into(tree, child, remaining_p);
        } else if let Some(remaining_p) = p.strip_prefix(']') {
            return remaining_p;
        } else if p.starts_with(',') {
            p = &p[1..];
        } else {
            let unnumeric = p.find(|c: char| !c.is_numeric()).unwrap_or(p.len());

            let value = p[0..unnumeric].parse::<u8>().unwrap();

            tree.get_mut(node).unwrap().append(Some(value));
            p = &p[unnumeric..];
        }
    }
}

fn compare(tx: &Tree<Option<u8>>, ty: &Tree<Option<u8>>) -> bool {
    if let Some(r) = compare_with(tx, tx.root_id().unwrap(), ty, ty.root_id().unwrap()) {
        r
    } else {
        todo!()
    }
}

fn compare_with(
    tx: &Tree<Option<u8>>,
    nx: NodeId,
    ty: &Tree<Option<u8>>,
    ny: NodeId,
) -> Option<bool> {
    let nx = tx.get(nx).unwrap();
    let ny = ty.get(ny).unwrap();

    for either in nx.children().zip_longest(ny.children()) {
        match either {
            EitherOrBoth::Both(cx, cy) => {
                let xnone = cx.data().is_none();
                let ynone = cy.data().is_none();

                if xnone && ynone {
                    // both lists, recurse
                    let cx = cx.node_id();
                    let cy = cy.node_id();

                    if let Some(r) = compare_with(tx, cx, ty, cy) {
                        return Some(r);
                    }
                } else if xnone {
                    let cx = cx.node_id();

                    let mut ty = Tree::new();
                    let root = ty.set_root(None);
                    ty.get_mut(root).unwrap().append(*cy.data()).node_id();

                    if let Some(r) = compare_with(tx, cx, &ty, root) {
                        return Some(r);
                    }
                } else if ynone {
                    let mut tx = Tree::new();
                    let root = tx.set_root(None);
                    tx.get_mut(root).unwrap().append(*cx.data()).node_id();

                    let cy = cy.node_id();

                    if let Some(r) = compare_with(&tx, root, ty, cy) {
                        return Some(r);
                    }
                } else {
                    // neither are lists - compare!
                    let cx = cx.data().unwrap();
                    let cy = cy.data().unwrap();

                    if cx != cy {
                        return Some(cx < cy);
                    }
                }
            }
            EitherOrBoth::Left(_) => {
                // If the right list runs out of items first, the inputs are not in the right order.
                return Some(false);
            }
            EitherOrBoth::Right(_) => {
                // If the left list runs out of items first, the inputs are in the right order.
                return Some(true);
            }
        }
    }

    None
}

fn solver(data: &str) -> SolutionPair {
    let mut p1 = 0;

    for (index, mut chunk) in data
        .lines()
        .filter(|str| !str.is_empty())
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let x = chunk.next().unwrap();
        let y = chunk.next().unwrap();

        let tx = parse(x);
        assert_eq!(x, &dump(&tx));

        let ty = parse(y);
        assert_eq!(y, &dump(&ty));

        if compare(&tx, &ty) {
            // +1 because AoC is starting at 1 instead of 0
            p1 += (index as u32) + 1;
        }
    }

    let mut packets = Vec::new();

    packets.push(parse("[[2]]"));
    packets.push(parse("[[6]]"));

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        packets.push(parse(line));
    }

    packets.sort_by(|x, y| {
        if compare(x, y) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut p2 = 1;

    for (index, packet) in packets.iter().enumerate() {
        match dump(packet).as_str() {
            "[[2]]" | "[[6]]" => {
                p2 *= index + 1;
            }
            _ => (),
        }
    }

    (Solution::U32(p1), Solution::U32(p2 as u32))
}

fn dump(t: &Tree<Option<u8>>) -> String {
    let mut s = String::new();
    dump_with(t, t.root_id().unwrap(), &mut s);
    s
}

fn dump_with(t: &Tree<Option<u8>>, n: NodeId, s: &mut String) {
    let n = t.get(n).unwrap();

    if let Some(n) = n.data() {
        write!(s, "{}", n).unwrap();
    } else {
        write!(s, "[").unwrap();

        let mut first = true;
        for child in n.children() {
            if first {
                first = false;
            } else {
                write!(s, ",").unwrap();
            }

            dump_with(t, child.node_id(), s);
        }

        write!(s, "]").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 13);
            assert_eq!(p2, 140);
        } else {
            panic!();
        }
    }
}
