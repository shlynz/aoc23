use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|str_nr| str_nr.parse::<f32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let times = input[0].iter();
    let records = input[1].iter();
    let result = times
        .zip(records)
        .map(|(p, q)| (-p / 2_f32, (((p / 2_f32).powi(2) - q) as f32).sqrt()))
        .map(|(first, second)| (first + second).ceil() as i32 - (first - second).floor() as i32 - 1)
        .fold(1, |acc, res| acc * res as u32);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .join("")
                .split(":")
                .nth(1)
                .unwrap()
                .parse::<f64>()
                .unwrap()
        })
        .collect_vec();

    let time = input[0];
    let record = input[1];

    let first = -time / 2_f64;
    let second = (((time / 2_f64).powi(2) - record) as f64).sqrt();

    Some(((first + second).ceil() as i32 - (first - second).floor() as i32 - 1) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
