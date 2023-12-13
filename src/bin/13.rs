use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<bool>>,
}

impl Pattern {
    fn transpose(&self) -> Self {
        let mut result = Vec::new();
        for col_nr in 0..self.pattern[0].len() {
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
        self.pattern.iter().for_each(|line| {
            println!(
                "{}",
                line.iter()
                    .map(|&nr| if nr { '#' } else { '.' })
                    .collect::<String>()
            )
        })
    }

    fn get_symmetry_score(&self, smudges: u32) -> usize {
        let mut result = 0;
        if let Some(row) = self.get_horizontal_symmetry_pos(smudges) {
            result += row * 100;
        } else if let Some(col) = self.transpose().get_horizontal_symmetry_pos(smudges) {
            result += col;
        } else {
            panic!("No symmetry found");
        }
        result
    }

    fn get_horizontal_symmetry_pos(&self, smudges: u32) -> Option<usize> {
        let to_byte = |bits: &Vec<bool>| -> usize {
            bits.iter()
                .fold(0, |acc, &curr| acc * 2 + usize::from(curr))
        };
        let get_diffs = |first: &Vec<bool>, second: &Vec<bool>| -> u32 {
            (to_byte(first) ^ to_byte(second)).count_ones()
        };
        'MAIN: for index in 0..self.pattern.len() - 1 {
            let mut diffs = get_diffs(&self.pattern[index], &self.pattern[index + 1]);
            if diffs <= smudges {
                for (first, second) in (0..index).rev().zip(index + 2..self.pattern.len()) {
                    diffs += get_diffs(&self.pattern[first], &self.pattern[second]);
                    if diffs > smudges {
                        continue 'MAIN;
                    }
                }
                if diffs != smudges {
                    continue 'MAIN;
                }
                return Some(index + 1);
            }
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| Pattern {
                pattern: pattern
                    .lines()
                    .map(|line| {
                        line.bytes()
                            .map(|b| if b == b'.' { false } else { true })
                            .collect_vec()
                    })
                    .collect_vec(),
            })
            .map(|pattern| pattern.get_symmetry_score(0))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| Pattern {
                pattern: pattern
                    .lines()
                    .map(|line| {
                        line.bytes()
                            .map(|b| if b == b'.' { false } else { true })
                            .collect_vec()
                    })
                    .collect_vec(),
            })
            .map(|pattern| pattern.get_symmetry_score(1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
