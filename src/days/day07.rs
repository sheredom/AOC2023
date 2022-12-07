use core::panic;

use slab_tree::Tree;

use crate::{Solution, SolutionPair};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day07/input"));

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    solver(INPUT)
}

#[derive(Debug, Eq, PartialEq)]
enum Kind {
    Directory,
    File,
}

#[derive(Debug)]
struct TreeData<'a> {
    name: &'a str,
    size: u32,
    kind: Kind,
}

impl<'a> TreeData<'a> {
    pub fn directory(name: &'a str) -> Self {
        Self {
            name,
            size: 0,
            kind: Kind::Directory,
        }
    }

    pub fn file(name: &'a str, size: u32) -> Self {
        Self {
            name,
            size,
            kind: Kind::File,
        }
    }
}

fn solver(data: &str) -> SolutionPair {
    let mut tree = Tree::new();
    tree.set_root(TreeData::directory("/"));

    let mut directory = tree.root_id().unwrap();

    for line in data.lines() {
        if let Some(command) = line.strip_prefix("$ ") {
            if let Some(cd) = command.strip_prefix("cd ") {
                directory = match cd {
                    "/" => tree.root_id().unwrap(),
                    ".." => {
                        let node = tree.get(directory).unwrap();

                        node.ancestors().next().unwrap().node_id()
                    }
                    location => {
                        let node = tree.get(directory).unwrap();
                        let child = node
                            .children()
                            .find(|child| child.data().name == location)
                            .unwrap();

                        child.node_id()
                    }
                };
            } else if command.strip_prefix("ls").is_some() {
                // do nothing, the actual body of the ls is handled later
            } else {
                panic!("Unknown command '{}'", command);
            }
        } else if let Some(new_directory) = line.strip_prefix("dir ") {
            tree.get_mut(directory)
                .unwrap()
                .append(TreeData::directory(new_directory));
        } else {
            // we must be in an `ls` if we got here!
            let (size, name) = line.split_once(' ').unwrap();
            let size = size.parse::<u32>().unwrap();
            tree.get_mut(directory)
                .unwrap()
                .append(TreeData::file(name, size));
        }
    }

    {
        let mut visit = vec![tree.root_id().unwrap()];

        while let Some(node) = visit.pop() {
            let mut size = 0;

            for child in tree.get(node).unwrap().children() {
                let child_size = child.data().size;

                if 0 == child_size {
                    // push ourselves again, as we want to revisit us when we've processed the child!
                    visit.push(node);
                    visit.push(child.node_id());
                    break;
                } else {
                    size += child_size;
                }
            }

            tree.get_mut(node).unwrap().data().size = size;
        }
    }

    let mut p1 = 0;

    {
        let mut visit = vec![tree.root_id().unwrap()];

        while let Some(node) = visit.pop() {
            let node = tree.get(node).unwrap();

            let size = node.data().size;

            if size < 100000 {
                p1 += size;
            }

            for child in node.children() {
                if child.data().kind == Kind::Directory {
                    visit.push(child.node_id());
                }
            }
        }
    }

    let max_size = 70000000;
    let required_free_space = 30000000;

    let minimum_deletion_size_required =
        required_free_space - (max_size - (tree.root().unwrap().data().size));

    let p2;

    {
        let mut collected = Vec::new();

        let mut visit = vec![tree.root_id().unwrap()];

        while let Some(node) = visit.pop() {
            let node = tree.get(node).unwrap();

            collected.push(node.data());

            for child in node.children() {
                if child.data().kind == Kind::Directory {
                    visit.push(child.node_id());
                }
            }
        }

        collected.sort_by(|a, b| a.size.cmp(&b.size));

        let to_delete = collected
            .iter()
            .find(|item| item.size > minimum_deletion_size_required)
            .unwrap();

        p2 = to_delete.size;
    }

    (Solution::U32(p1), Solution::U32(p2))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::solver;

    static EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn it_works() {
        let solution = solver(EXAMPLE);

        if let (Solution::U32(p1), Solution::U32(p2)) = solution {
            assert_eq!(p1, 95437);
            assert_eq!(p2, 24933642);
        } else {
            panic!();
        }
    }
}
