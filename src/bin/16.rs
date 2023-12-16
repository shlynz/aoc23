use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(&self, current_mirror: char) -> Vec<Direction> {
        match current_mirror {
            '.' => vec![self.clone()],
            '/' => match self {
                Direction::North => vec![Direction::East],
                Direction::East => vec![Direction::North],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
            },
            '\\' => match self {
                Direction::North => vec![Direction::West],
                Direction::East => vec![Direction::South],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
            },
            '|' if self == &Direction::North || self == &Direction::South => vec![self.clone()],
            '|' if self == &Direction::East || self == &Direction::West => {
                vec![Direction::North, Direction::South]
            }
            '-' if self == &Direction::East || self == &Direction::West => vec![self.clone()],
            '-' if self == &Direction::North || self == &Direction::South => {
                vec![Direction::East, Direction::West]
            }
            _ => unreachable!(),
        }
    }

    fn offset(
        &self,
        current_x: usize,
        current_y: usize,
        bounds_x: usize,
        bounds_y: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::North if current_y <= 0 => None,
            Direction::North => Some((current_x, current_y - 1)),
            Direction::East if current_x >= bounds_x => None,
            Direction::East => Some((current_x + 1, current_y)),
            Direction::South if current_y >= bounds_y => None,
            Direction::South => Some((current_x, current_y + 1)),
            Direction::West if current_x <= 0 => None,
            Direction::West => Some((current_x - 1, current_y)),
        }
    }
}

// Rust ran into a Stack Overflow with this recursive attempt
// I'm unsure if the algo has errors or just simply ran too deep...
#[allow(dead_code)]
fn step(
    visited: &mut HashMap<(usize, usize), Vec<Direction>>,
    grid: &Vec<Vec<char>>,
    bounds_x: usize,
    bounds_y: usize,
    curr_x: usize,
    curr_y: usize,
    dir: &Direction,
) {
    if let Some(prev_dirs) = visited.get_mut(&(curr_x, curr_y)) {
        if prev_dirs.contains(dir) {
            return;
        }
        prev_dirs.push(dir.clone());
    } else {
        visited.insert((curr_x, curr_y), vec![dir.clone()]);
    }
    let next_dirs = dir.next(grid[curr_y][curr_x]);
    next_dirs.iter().for_each(|next_dir| {
        if let Some((next_x, next_y)) = next_dir.offset(curr_x, curr_y, bounds_x, bounds_y) {
            step(visited, grid, bounds_x, bounds_y, next_x, next_y, &next_dir)
        }
    });
}

fn search(
    grid: &Vec<Vec<char>>,
    initial_x: usize,
    initial_y: usize,
    initial_dir: Direction,
) -> usize {
    let mut visited: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let bounds_x = grid[0].len() - 1;
    let bounds_y = grid.len() - 1;
    let mut next_states = Vec::new();
    next_states.push((initial_x, initial_y, initial_dir));
    while let Some(curr_state) = next_states.pop() {
        let (curr_x, curr_y, dir) = curr_state;
        if let Some(prev_dirs) = visited.get_mut(&(curr_x, curr_y)) {
            if prev_dirs.contains(&dir) {
                continue;
            }
            prev_dirs.push(dir.clone());
        } else {
            visited.insert((curr_x, curr_y), vec![dir.clone()]);
        }
        let next_dirs = dir.next(grid[curr_y][curr_x]);
        next_dirs.iter().for_each(|next_dir| {
            if let Some((next_x, next_y)) = next_dir.offset(curr_x, curr_y, bounds_x, bounds_y) {
                next_states.push((next_x, next_y, next_dir.clone()));
            }
        });
    }
    visited.len()
}
pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    Some(search(&grid, 0, 0, Direction::East))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut results = Vec::new();
    for row_nr in 0..grid.len() {
        results.push(search(&grid, 0, row_nr, Direction::East));
        results.push(search(&grid, grid[0].len() - 1, row_nr, Direction::West));
    }
    for col_nr in 0..grid[0].len() {
        results.push(search(&grid, col_nr, 0, Direction::South));
        results.push(search(&grid, col_nr, grid.len() - 1, Direction::North));
    }
    Some(results.iter().max().unwrap().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
