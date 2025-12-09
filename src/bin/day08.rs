use std::cmp::{Reverse, max, min};

use aoc2025::aoc;

const DAY: u32 = 8;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata, 1000));
    println!("Part 2: {}", part_two(&fdata));
}

#[derive(PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    z: usize,
}

fn parse(input: &str) -> Vec<Node> {
    input.lines()
        .map(|s| {
            let v = s.split_once(',').unwrap();
            let w = v.1.split_once(',').unwrap();

            Node {
                x: v.0.parse().unwrap(),
                y: w.0.parse().unwrap(),
                z: w.1.parse().unwrap(),
            }
        })
        .collect()
}

impl Node {
    fn dist(&self, other: &Node) -> usize {
        let x = self.x.abs_diff(other.x).pow(2);
        let y = self.y.abs_diff(other.y).pow(2);
        let z = self.z.abs_diff(other.z).pow(2);
        x + y + z
    }

    fn connect<'a>(&'a self, to: &'a Node, connected: &mut Vec<Vec<&'a Node>>) {
        let pos_this = connected.iter().position(|c| c.contains(&self));
        let pos_that = connected.iter().position(|c| c.contains(&to));

        match (pos_this, pos_that) {
            (Some(idx_this), Some(idx_that)) => {
                if idx_this == idx_that {
                    return;
                }

                // NOTE: Must use `max` and `min` here to avoid a potential panic.
                // Always remove the latter-occurring element first.
                let f = connected.remove(max(idx_this, idx_that));
                let t = connected.remove(min(idx_this, idx_that));
                let c = f.into_iter().chain(t).collect();
                connected.push(c);
            },
            (Some(idx_this), None) => {
                connected.get_mut(idx_this).unwrap().push(to);
            },
            (None, Some(idx_that)) => {
                connected.get_mut(idx_that).unwrap().push(self);
            },
            (None, None) => {
                connected.push(Vec::from([self, to]));
            },
        }
    }
}

fn pair_nodes(nodes: &[Node]) -> Vec<(&Node, &Node)> {
    let mut pairs: Vec<_> = nodes.iter().enumerate()
        .flat_map(|(i, n1)| nodes[i + 1..].iter().map(move |n2| (n1, n2)))
        .collect();
    pairs.sort_by_key(|p| p.0.dist(p.1));
    pairs
}

fn part_one(input: &str, n_connections: usize) -> usize {
    let nodes = parse(input);
    let pairs = pair_nodes(&nodes);

    let mut connected: Vec<Vec<&Node>> = Vec::new();
    pairs.into_iter().take(n_connections).for_each(|(f, t)| f.connect(t, &mut connected));

    connected.sort_by_key(|c| Reverse(c.len())); // descending order
    connected[0].len() * connected[1].len() * connected[2].len()
}

fn part_two(input: &str) -> usize {
    let nodes = parse(input);
    let pairs = pair_nodes(&nodes);

    let mut connected: Vec<Vec<&Node>> = Vec::new();
    let last_connected = pairs.into_iter()
        .find(|(f, t)| {
            f.connect(t, &mut connected);
            connected[0].len() == nodes.len()
        })
        .unwrap();

    last_connected.0.x * last_connected.1.x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY), 10), 40);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 25272);
    }
}
