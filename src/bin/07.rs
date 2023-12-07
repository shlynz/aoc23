use itertools::{FoldWhile, Itertools};
use std::cmp::Ordering;

advent_of_code::solution!(7);

#[derive(Debug, Eq)]
struct Hand {
    kind: String,
    cards: Vec<u8>,
    bet: i32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_result = self.kind.cmp(&other.kind);
        match cmp_result {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .fold_while(Ordering::Equal, |acc, curr| {
                    let (a, b) = curr;
                    match a.cmp(b) {
                        Ordering::Less => FoldWhile::Done(Ordering::Less),
                        Ordering::Equal => FoldWhile::Continue(acc),
                        Ordering::Greater => FoldWhile::Done(Ordering::Greater),
                    }
                })
                .into_inner(),
            _ => cmp_result,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards == other.cards
    }
}

fn byte_to_card_value(byte: &u8, is_part_two: bool) -> u8 {
    match byte {
        b'T' => 0b_10000000,
        b'J' => {
            if is_part_two {
                0b_00000000
            } else {
                0b_10000001
            }
        }
        b'Q' => 0b_10000010,
        b'K' => 0b_10000100,
        b'A' => 0b_10001000,
        _ => *byte,
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let split_pos = input.chars().position(|c| c == ' ').unwrap();
    Some(
        input
            .lines()
            .map(|hand| Hand {
                kind: hand[0..split_pos]
                    .as_bytes()
                    .iter()
                    .sorted()
                    .dedup_with_count()
                    .map(|(count, _)| count)
                    .sorted()
                    .rev()
                    .join(""),
                cards: hand[0..split_pos]
                    .as_bytes()
                    .iter()
                    .map(|byte| byte_to_card_value(byte, false))
                    .collect(),
                bet: hand[split_pos + 1..].parse().unwrap(),
            })
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank as i32 + 1) * hand.bet)
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let split_pos = input.chars().position(|c| c == ' ').unwrap();
    Some(
        input
            .lines()
            .map(|hand| {
                let mut constellation = hand[0..split_pos]
                    .as_bytes()
                    .iter()
                    .sorted()
                    .dedup_with_count()
                    .sorted_by(|(a, _), (b, _)| b.cmp(a))
                    .collect_vec();
                let amount_joker = if let Some((amount_joker, _)) =
                    constellation.iter().find(|(_, kind)| kind == &&b'J')
                {
                    *amount_joker
                } else {
                    0_usize
                };

                if constellation[0].1 == &b'J' {
                    if constellation.len() > 1 {
                        constellation[1].0 += amount_joker;
                        constellation.retain(|(_, kind)| kind != &&b'J');
                    }
                } else {
                    constellation[0].0 += amount_joker;
                    constellation.retain(|(_, kind)| kind != &&b'J');
                }

                Hand {
                    kind: constellation
                        .iter()
                        .map(|(count, _)| count)
                        .sorted()
                        .rev()
                        .join(""),
                    cards: hand[0..split_pos]
                        .as_bytes()
                        .iter()
                        .map(|byte| byte_to_card_value(byte, true))
                        .collect(),
                    bet: hand[split_pos + 1..].parse().unwrap(),
                }
            })
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank as i32 + 1) * hand.bet)
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
