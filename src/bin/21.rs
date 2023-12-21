use std::{
    collections::{HashMap, HashSet, VecDeque},
    isize, usize,
};

use itertools::Itertools;

advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: isize,
    y: isize,
    unwrapped_x: isize,
    unwrapped_y: isize,
}

impl Coords {
    fn new(x: isize, y: isize) -> Self {
        Coords {
            x,
            y,
            unwrapped_x: x,
            unwrapped_y: y,
        }
    }

    fn new_wrapped(x: isize, y: isize, original_x: isize, original_y: isize) -> Self {
        Coords {
            x,
            y,
            unwrapped_x: original_x,
            unwrapped_y: original_y,
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn get_coords(&self, char_to_find: char) -> Option<Coords> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == char_to_find {
                    return Some(Coords::new(x as isize, y as isize));
                }
            }
        }
        None
    }

    fn get_adjacent(&self, location: Coords, wrap: bool) -> Vec<Coords> {
        let adjacent = vec![
            Coords::new(0, -1),
            Coords::new(1, 0),
            Coords::new(0, 1),
            Coords::new(-1, 0),
        ]
        .iter()
        .map(|&offset| location + offset)
        .collect_vec();

        if wrap {
            adjacent
                .into_iter()
                .map(|new_pos| self.wrap(new_pos))
                .filter(|&new_wrapped_pos| self.check_valid(new_wrapped_pos))
                .collect_vec()
        } else {
            adjacent
                .into_iter()
                .filter(|&new_pos| self.check_valid(new_pos))
                .collect_vec()
        }
    }

    fn wrap(&self, coords: Coords) -> Coords {
        let mut wrapped = false;
        let max_x = self.get_max_x();
        let new_x = if coords.x < 0 {
            wrapped = true;
            coords.x + max_x
        } else if coords.x >= max_x {
            wrapped = true;
            coords.x - max_x
        } else {
            coords.x
        };
        let max_y = self.get_max_y();
        let new_y = if coords.y < 0 {
            wrapped = true;
            coords.y + max_y
        } else if coords.y >= max_y {
            wrapped = true;
            coords.y - max_y
        } else {
            coords.y
        };
        if wrapped {
            Coords::new_wrapped(new_x, new_y, coords.unwrapped_x, coords.unwrapped_y)
        } else {
            coords
        }
    }

    fn check_valid(&self, coords: Coords) -> bool {
        if coords.x >= 0
            && coords.y >= 0
            && coords.x < self.get_max_x()
            && coords.y < self.get_max_y()
        {
            self.grid[coords.y as usize][coords.x as usize] != '#'
        } else {
            false
        }
    }

    fn get_max_x(&self) -> isize {
        self.grid[0].len() as isize
    }

    fn get_max_y(&self) -> isize {
        self.grid.len() as isize
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Grid {
            grid: value
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        }
    }
}

impl std::ops::Add for Coords {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            unwrapped_x: self.unwrapped_x + rhs.unwrapped_x,
            unwrapped_y: self.unwrapped_y + rhs.unwrapped_y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    coords: Coords,
    steps_taken: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    let mut seen: HashSet<State> = HashSet::new();
    let mut to_check: Vec<State> = Vec::new();
    let mut result: HashSet<Coords> = HashSet::new();

    let starting_coords = grid.get_coords('S').unwrap();
    to_check.push(State {
        coords: starting_coords,
        steps_taken: 0,
    });
    while let Some(state) = to_check.pop() {
        if state.steps_taken == 64 {
            result.insert(state.coords);
            continue;
        }
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        for adjacent in grid.get_adjacent(state.coords, false) {
            to_check.push(State {
                coords: adjacent,
                steps_taken: state.steps_taken + 1,
            });
        }
    }
    Some(result.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    let mut seen: HashSet<State> = HashSet::new();
    let mut to_check: Vec<State> = Vec::new();
    let mut result: HashSet<Coords> = HashSet::new();

    let starting_coords = grid.get_coords('S').unwrap();
    to_check.push(State {
        coords: starting_coords,
        steps_taken: 0,
    });
    while let Some(state) = to_check.pop() {
        if state.steps_taken == 26501365 {
            result.insert(state.coords);
            continue;
        }
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        for adjacent in grid.get_adjacent(state.coords, true) {
            to_check.push(State {
                coords: adjacent,
                steps_taken: state.steps_taken + 1,
            });
        }
    }
    Some(result.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6536));
    }
}
