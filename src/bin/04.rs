use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let nr_match = Regex::new(r"(?: {0,2})(?<nr>\d+)").unwrap();
    Some(
        input
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
            .map(|split| {
                (
                    nr_match
                        .captures_iter(split.0)
                        .map(|cap| cap.name("nr").unwrap().as_str().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                    nr_match
                        .captures_iter(split.1)
                        .map(|cap| cap.name("nr").unwrap().as_str().parse::<i32>().unwrap()),
                )
            })
            .map(|numbers| numbers.1.filter(|nr| numbers.0.contains(nr)).count() as u32)
            .filter(|amount| amount > &0)
            .map(|amount| 2_i32.pow(amount - 1))
            .sum::<i32>()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let nr_match = Regex::new(r"(?: {0,2})(?<nr>\d+)").unwrap();
    let mut wins_per_card = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.split_once(" | ").unwrap())
        .map(|split| {
            (
                nr_match
                    .captures_iter(split.0)
                    .map(|cap| cap.name("nr").unwrap().as_str().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
                nr_match
                    .captures_iter(split.1)
                    .map(|cap| cap.name("nr").unwrap().as_str().parse::<i32>().unwrap()),
            )
        })
        .map(|numbers| {
            (
                1_i32,
                numbers.1.filter(|nr| numbers.0.contains(nr)).count() as i32,
            )
        })
        .collect_vec();

    for pos in 0..wins_per_card.len() {
        let (amount, wins) = wins_per_card[pos];
        for i in 0..wins {
            let new_pos = pos + i as usize + 1_usize;
            if new_pos < wins_per_card.len() {
                wins_per_card[new_pos] =
                    (wins_per_card[new_pos].0 + amount, wins_per_card[new_pos].1);
            }
        }
    }
    Some(wins_per_card.iter().map(|cards| cards.0 as u32).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
