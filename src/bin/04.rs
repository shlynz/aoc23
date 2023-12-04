advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // inspired by (read: stolen from) github.com/timvisee
    let winning_numbers_start = input.chars().position(|c| c == ':').unwrap();
    let scratch_numbers_start = input.chars().position(|c| c == '|').unwrap();
    Some(
        input
            .lines()
            .map(|card| {
                card[scratch_numbers_start + 1..]
                    .as_bytes()
                    .chunks_exact(3)
                    .filter(|chunk| {
                        card[winning_numbers_start + 1..scratch_numbers_start]
                            .as_bytes()
                            .chunks_exact(3)
                            .any(|winning_nr| &winning_nr == chunk)
                    })
                    .count() as u32
            })
            .filter(|wins| wins > &0)
            .map(|wins| 2_u32.pow(wins - 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // inspired by (read: stolen from) github.com/timvisee
    let winning_numbers_start = input.chars().position(|c| c == ':').unwrap();
    let scratch_numbers_start = input.chars().position(|c| c == '|').unwrap();
    let mut additional_wins = [1_u32; 256];
    Some(
        input
            .lines()
            .enumerate()
            .map(|(card_nr, card)| {
                let wins = card[scratch_numbers_start + 1..]
                    .as_bytes()
                    .chunks_exact(3)
                    .filter(|chunk| {
                        card[winning_numbers_start + 1..scratch_numbers_start]
                            .as_bytes()
                            .chunks_exact(3)
                            .any(|winning_nr| &winning_nr == chunk)
                    })
                    .count() as usize;
                for extra_ticket_won in 1..=wins {
                    additional_wins[extra_ticket_won + card_nr] += additional_wins[card_nr];
                }
                additional_wins[card_nr]
            })
            .sum::<u32>(),
    )
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
