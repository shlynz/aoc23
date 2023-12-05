use itertools::Itertools;
advent_of_code::solution!(5);

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
    modifier: i64,
}

impl Range {
    pub fn from(range_str: &str) -> Self {
        let mut parts = range_str
            .split_whitespace()
            .map(|str_nr| str_nr.parse::<u64>().unwrap());
        let to = parts.next().unwrap();
        let from = parts.next().unwrap();
        let length = parts.next().unwrap();
        Range {
            start: from,
            end: from.saturating_add(length - 1),
            modifier: to as i64 - from as i64,
        }
    }

    pub fn apply(&self, nr: &u64) -> u64 {
        nr.saturating_add_signed(self.modifier)
    }

    pub fn is_in_range(&self, nr: &u64) -> bool {
        nr <= &self.end && nr >= &self.start
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let chunks = input.split("\n\n").collect_vec();
    let seeds = chunks[0]
        .split(": ")
        .nth(1)?
        .split_whitespace()
        .map(|str_nr| str_nr.parse::<u64>().unwrap());

    let almanac = chunks
        .iter()
        .skip(1)
        .map(|rule| {
            rule.lines()
                .skip(1)
                .map(|range| Range::from(range))
                .collect_vec()
        })
        .collect_vec();

    Some(
        seeds
            .map(|seed| {
                almanac.iter().fold(seed, |nr, ranges| {
                    match ranges.iter().find(|range| range.is_in_range(&nr)) {
                        Some(range) => range.apply(&nr),
                        _ => nr,
                    }
                })
            })
            .min()
            .unwrap() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // The fuck
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
