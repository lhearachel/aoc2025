use std::cmp::{Reverse, max, min};

use aoc2025::aoc;

const DAY: u32 = 9;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

#[derive(Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> Vec<Node> {
    input.lines()
        .map(|line| {
            let p = line.split_once(',').unwrap();
            Node {
                x: p.0.parse().expect("could not parse x coordinate"),
                y: p.1.parse().expect("could not parse y coordinate"),
            }
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let nodes = parse(input);
    let mut area = 0;

    for (i, n1) in nodes.iter().enumerate() {
        for n2 in nodes[i + 1..].iter() {
            let w = n2.x.abs_diff(n1.x) + 1;
            let h = n2.y.abs_diff(n1.y) + 1;
            area = area.max(w * h);
        }
    }

    area
}

struct Rectangle {
    n1: Node,
    n2: Node,
    area: usize,
}

impl Rectangle {
    fn is_valid(&self, edges: &[(Node, Node)]) -> bool {
        let min_x = min(self.n1.x, self.n2.x);
        let max_x = max(self.n1.x, self.n2.x);
        let min_y = min(self.n1.y, self.n2.y);
        let max_y = max(self.n1.y, self.n2.y);

        !edges.iter().any(|(n1, n2)| {
            let min_edge_x = min(n1.x, n2.x);
            let max_edge_x = max(n1.x, n2.x);
            let min_edge_y = min(n1.y, n2.y);
            let max_edge_y = max(n1.y, n2.y);

            // Check if any edge overlaps the rectangle's interior
            min_x < max_edge_x
                && max_x > min_edge_x
                && min_y < max_edge_y
                && max_y > min_edge_y
        })
    }
}

fn part_two(input: &str) -> usize {
    let nodes = parse(input);

    let mut rects: Vec<_> = nodes.iter().enumerate()
        .flat_map(|(i, n1)| {
            nodes[i + 1..].iter()
                .map(move |n2| {
                    let w = n2.x.abs_diff(n1.x) + 1;
                    let h = n2.y.abs_diff(n1.y) + 1;
                    Rectangle {
                        n1: *n1,
                        n2: *n2,
                        area: w * h,
                    }
                })
        })
        .collect();
    rects.sort_by_key(|r| Reverse(r.area));

    let mut edges: Vec<_> = nodes.windows(2)
        .map(|w| (w[0], w[1]))
        .collect();
    edges.push((*nodes.last().unwrap(), *nodes.first().unwrap()));

    rects.iter().find(|r| r.is_valid(&edges)).map_or(0, |r| r.area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 50);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 24);
    }
}
