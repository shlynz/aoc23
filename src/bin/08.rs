use itertools::{FoldWhile, Itertools};
use num_integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Node<'a> {
    left: &'a [u8],
    right: &'a [u8],
}
pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = input.split_once("\n\n").unwrap();
    let nodes: HashMap<_, _> = nodes
        .lines()
        .map(str::as_bytes)
        .map(|bytes| {
            (
                &bytes[0..=2],
                Node {
                    left: &bytes[7..=9],
                    right: &bytes[12..=14],
                },
            )
        })
        .collect();

    let starting_node = nodes.get("AAA".as_bytes()).unwrap();

    Some(
        directions
            .chars()
            .cycle()
            .fold_while((0, starting_node), |(steps, last_node), next_dir| {
                let steps = steps + 1;
                let next_node_name = if next_dir == 'R' {
                    last_node.right
                } else {
                    last_node.left
                };
                let next_node = nodes.get(next_node_name).unwrap();
                if next_node_name == "ZZZ".as_bytes() {
                    FoldWhile::Done((steps, next_node))
                } else {
                    FoldWhile::Continue((steps, next_node))
                }
            })
            .into_inner()
            .0 as u32,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (directions, node_lines) = input.split_once("\n\n").unwrap();
    let node_lines = node_lines.lines().map(str::as_bytes).collect_vec();
    let nodes: HashMap<_, _> = node_lines
        .iter()
        .map(|bytes| {
            (
                &bytes[0..=2],
                Node {
                    left: &bytes[7..=9],
                    right: &bytes[12..=14],
                },
            )
        })
        .collect();

    Some(
        node_lines
            .iter()
            .filter(|line| line[2] == b'A')
            .map(|line| &line[0..=2])
            .map(|node_name| nodes.get(node_name).unwrap())
            .map(|node| {
                directions
                    .chars()
                    .cycle()
                    .fold_while((0, node), |(steps, last_node), next_dir| {
                        let steps = steps + 1;
                        let next_node_name = if next_dir == 'R' {
                            last_node.right
                        } else {
                            last_node.left
                        };
                        let next_node = nodes.get(next_node_name).unwrap();
                        if next_node_name[2] == b'Z' {
                            FoldWhile::Done((steps, next_node))
                        } else {
                            FoldWhile::Continue((steps, next_node))
                        }
                    })
                    .into_inner()
                    .0 as u64
            })
            .reduce(|acc, curr| lcm(acc, curr))
            .unwrap() as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
