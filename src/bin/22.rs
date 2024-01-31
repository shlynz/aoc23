use std::{collections::HashMap, ops::RangeBounds};

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(22);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn down(&mut self) {
        self.z -= 1;
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    start: Coord,
    end: Coord,
}

impl Brick {
    fn down(&mut self) {
        self.start.down();
        self.end.down();
    }

    fn contains(&self, coord: &Coord) -> bool {
        (self.start.x..=self.end.x).contains(&coord.x)
            && (self.start.y..=self.end.y).contains(&coord.y)
            && (self.start.z..=self.end.z).contains(&coord.z)
    }
}

fn parse(input: &str) -> Vec<Brick> {
    let re =
        Regex::new(r"(?<x1>\d+),(?<y1>\d+),(?<z1>\d+)~(?<x2>\d+),(?<y2>\d+),(?<z2>\d+)").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|captures| Brick {
            start: Coord {
                x: captures.name("x1").unwrap().as_str().parse().unwrap(),
                y: captures.name("y1").unwrap().as_str().parse().unwrap(),
                z: captures.name("z1").unwrap().as_str().parse().unwrap(),
            },
            end: Coord {
                x: captures.name("x2").unwrap().as_str().parse().unwrap(),
                y: captures.name("y2").unwrap().as_str().parse().unwrap(),
                z: captures.name("z2").unwrap().as_str().parse().unwrap(),
            },
        })
        .collect_vec()
}

fn compress(bricks: &mut Vec<Brick>) -> Vec<usize> {
    let mut changed_brick_nrs = Vec::new();
    'BRICKS: for brick_nr in 0..bricks.len() {
        let brick = bricks[brick_nr];
        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y..=brick.end.y {
                for z in brick.start.z..=brick.end.z {
                    if z == 1
                        || bricks.iter().enumerate().rev().any(|(nr, brick)| {
                            nr != brick_nr && brick.contains(&Coord { x, y, z: z - 1 })
                        })
                    {
                        continue 'BRICKS;
                    }
                }
            }
        }
        bricks[brick_nr].down();
        changed_brick_nrs.push(brick_nr)
    }
    changed_brick_nrs.dedup();
    changed_brick_nrs
}

fn compress_fully(bricks: &mut Vec<Brick>) -> usize {
    let mut times_compressed = 0;
    while compress(bricks).len() > 0 {
        times_compressed += 1;
    }
    times_compressed
}

fn compress_fully_count(bricks: &mut Vec<Brick>) -> usize {
    let mut changed_brick_nrs = Vec::new();
    let mut done = false;
    while !done {
        let mut changes = compress(bricks);
        if changes.len() == 0 {
            done = true
        } else {
            changed_brick_nrs.append(&mut changes);
            changed_brick_nrs.sort();
            changed_brick_nrs.dedup();
        }
    }
    changed_brick_nrs.len()
}

// max size seems to be ca 10x10x250
pub fn part_one(input: &str) -> Option<usize> {
    let mut bricks = parse(input);
    compress_fully(&mut bricks);

    let mut removeables = 0;
    for brick_nr in 0..bricks.len() {
        let mut bricks_clone = bricks.clone();
        bricks_clone.remove(brick_nr);

        let can_compress = compress(&mut bricks_clone).len() > 0;
        if !can_compress {
            removeables += 1;
        }
    }

    Some(removeables)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut bricks = parse(input);
    compress_fully(&mut bricks);

    let mut affected_bricks = Vec::new();
    for brick_nr in 0..bricks.len() {
        let mut bricks_clone = bricks.clone();
        bricks_clone.remove(brick_nr);

        affected_bricks.push(compress_fully_count(&mut bricks_clone));
    }

    Some(affected_bricks.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
