use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pattern {
    pattern: Vec<Vec<char>>,
}

impl Pattern {
    fn rotate(&self) -> Self {
        let mut result = Vec::new();
        for col_nr in (0..self.pattern[0].len()).rev() {
            let mut line = Vec::new();
            for row_nr in (0..self.pattern.len()).rev() {
                line.push(self.pattern[row_nr][col_nr]);
            }
            result.push(line);
        }
        result.reverse();
        Pattern { pattern: result }
    }

    fn rotate_rev(&self) -> Self {
        let mut result = Vec::new();
        for col_nr in (0..self.pattern[0].len()).rev() {
            let mut line = Vec::new();
            for row_nr in 0..self.pattern.len() {
                line.push(self.pattern[row_nr][col_nr]);
            }
            result.push(line);
        }
        Pattern { pattern: result }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.pattern
            .iter()
            .for_each(|line| println!("{}", line.iter().collect::<String>()))
    }

    fn get_weight(&self) -> usize {
        self.pattern
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, line)| {
                acc + line.iter().filter(|&char| char == &'O').count() * (index + 1)
            })
    }

    fn tilt_up(&self) -> Self {
        Pattern {
            pattern: self
                .rotate_rev()
                .pattern
                .iter()
                .map(|line| {
                    let mut last_barricade = 0;
                    let mut new_line = line.clone();
                    for pointer in 0..line.len() {
                        match line[pointer] {
                            '#' => {
                                last_barricade = pointer;
                            }
                            'O' => {
                                new_line[pointer] = '.';
                                let new_pos = new_line[last_barricade..]
                                    .iter()
                                    .position(|&char| char == '.')
                                    .unwrap()
                                    + last_barricade;
                                new_line[new_pos] = 'O'
                            }
                            _ => continue,
                        }
                    }
                    new_line
                })
                .collect_vec(),
        }
        .rotate()
    }

    fn cycle(&self) -> Self {
        let mut next_iter = self.clone();
        for _ in 0..4 {
            next_iter = next_iter.tilt_up().rotate();
        }
        next_iter
    }

    fn get_weight_after_cycles(&self, cycle_amount: usize) -> usize {
        let mut cache = HashMap::new();
        let mut curr_pattern = self.clone();
        for counter in 0..cycle_amount {
            if let Some(last_iteration_encountered) = cache.get(&curr_pattern) {
                if (cycle_amount - counter) % (counter - last_iteration_encountered) == 0 {
                    return curr_pattern.get_weight();
                }
            }
            cache.insert(curr_pattern.clone(), counter);
            curr_pattern = curr_pattern.cycle();
        }
        curr_pattern.get_weight()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        Pattern {
            pattern: input
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        }
        .tilt_up()
        .get_weight(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        Pattern {
            pattern: input
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        }
        .get_weight_after_cycles(1000000000),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
