use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn all() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
    amount_same_dir: u8,
}

impl State {
    fn next(&self, bounds_x: usize, bounds_y: usize, part2: bool) -> Vec<Self> {
        if !part2 {
            let backwards = &self.dir.opposite();
            let mut all = Direction::all();
            all.retain(|dir| dir != backwards);
            if self.amount_same_dir >= 3 {
                all.retain(|dir| dir != &self.dir);
            }
            all.iter()
                .filter_map(|&dir| self.offset(dir, bounds_x, bounds_y))
                .collect_vec()
        } else {
            if self.amount_same_dir < 4 {
                if let Some(state) = self.offset(self.dir, bounds_x, bounds_y) {
                    vec![state]
                } else {
                    Vec::new()
                }
            } else {
                let backwards = &self.dir.opposite();
                let mut all = Direction::all();
                all.retain(|dir| dir != backwards);
                if self.amount_same_dir >= 10 {
                    all.retain(|dir| dir != &self.dir);
                }
                all.iter()
                    .filter_map(|&dir| self.offset(dir, bounds_x, bounds_y))
                    .collect_vec()
            }
        }
    }

    fn offset(&self, next_dir: Direction, bounds_x: usize, bounds_y: usize) -> Option<Self> {
        let next_coords = match next_dir {
            Direction::North if self.y <= 0 => None,
            Direction::North => Some((self.x, self.y - 1)),
            Direction::East if self.x >= bounds_x => None,
            Direction::East => Some((self.x + 1, self.y)),
            Direction::South if self.y >= bounds_y => None,
            Direction::South => Some((self.x, self.y + 1)),
            Direction::West if self.x <= 0 => None,
            Direction::West => Some((self.x - 1, self.y)),
        };

        if let Some((x, y)) = next_coords {
            let mut next_state = State {
                x,
                y,
                dir: next_dir,
                amount_same_dir: 1,
            };

            if self.dir == next_dir {
                next_state.amount_same_dir += self.amount_same_dir;
            }

            return Some(next_state);
        }
        None
    }
}

fn dijkstra(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize), part2: bool) -> usize {
    let mut visited: HashMap<State, usize> = HashMap::new();
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    let mut to_check: BinaryHeap<std::cmp::Reverse<(usize, State)>> = BinaryHeap::new();

    let (start_x, start_y) = start;
    let bounds_x = grid[0].len() - 1;
    let bounds_y = grid.len() - 1;

    to_check.push(std::cmp::Reverse((
        0,
        State {
            x: start_x,
            y: start_y,
            dir: Direction::East,
            amount_same_dir: 0,
        },
    )));

    while let Some(std::cmp::Reverse((curr_cost, state))) = to_check.pop() {
        let prev_cost = visited.get(&state).unwrap_or(&usize::MAX);
        if &curr_cost < prev_cost {
            visited.insert(state.clone(), curr_cost);
            state
                .next(bounds_x, bounds_y, part2)
                .iter()
                .for_each(|state| {
                    let next_cost = curr_cost + usize::from(grid[state.y][state.x]);
                    to_check.push(std::cmp::Reverse((next_cost, state.clone())));
                });
            let coords = (state.x, state.y);
            let prev_lowest_cost = costs.get(&coords).unwrap_or(&usize::MAX);
            if &curr_cost < prev_lowest_cost {
                if part2 && state.amount_same_dir < 4 {
                    continue;
                }
                costs.insert(coords, curr_cost);
            }
        }
    }
    /*
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter()
            .enumerate()
            .for_each(|(x, _)| print!("-{}-", costs.get(&(x, y)).unwrap_or(&0_usize)));
        println!("")
    });
    */
    *costs.get(&end).unwrap_or(&usize::MAX)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.bytes().map(|byte| u8::from(byte - 48)).collect_vec())
        .collect_vec();
    let end = (grid[0].len() - 1, grid.len() - 1);
    Some(dijkstra(&grid, (0, 0), end, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    // Both examples work, but the input doesn't
    let grid = input
        .lines()
        .map(|line| line.bytes().map(|byte| u8::from(byte - 48)).collect_vec())
        .collect_vec();
    let end = (grid[0].len() - 1, grid.len() - 1);
    Some(dijkstra(&grid, (0, 0), end, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
