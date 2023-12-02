advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|val| {
                let mut digits = val.matches(char::is_numeric).into_iter();
                match (digits.next(), digits.last()) {
                    (Some(first), Some(last)) => first.to_owned() + last,
                    (Some(first), None) => first.to_owned() + first,
                    (_, _) => "0".to_owned(),
                }
                .parse::<u32>()
                .unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|line| {
                let val = line
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine");
                let mut digits = val.matches(char::is_numeric).into_iter();
                match (digits.next(), digits.last()) {
                    (Some(first), Some(last)) => first.to_owned() + last,
                    (Some(first), None) => first.to_owned() + first,
                    (_, _) => "0".to_owned(),
                }
                .parse::<u32>()
                .unwrap()
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
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
