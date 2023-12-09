use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|val| val.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .map(|nrs| {
                let mut hist = vec![nrs.clone()];

                let mut last_hist = hist.last().unwrap();
                while !last_hist.iter().all(|&val| val == 0) {
                    hist.push(
                        last_hist
                            .iter()
                            .tuple_windows()
                            .map(|(first, second)| second - first)
                            .collect_vec(),
                    );
                    last_hist = hist.last().unwrap();
                }
                hist.iter().fold(0, |acc, curr| acc + curr.last().unwrap())
            })
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|val| val.parse::<i32>().unwrap())
                    .rev()
                    .collect_vec()
            })
            .map(|nrs| {
                let mut hist = vec![nrs.clone()];

                let mut last_hist = hist.last().unwrap();
                while !last_hist.iter().all(|&val| val == 0) {
                    hist.push(
                        last_hist
                            .iter()
                            .tuple_windows()
                            .map(|(first, second)| second - first)
                            .collect_vec(),
                    );
                    last_hist = hist.last().unwrap();
                }
                hist.iter().fold(0, |acc, curr| acc + curr.last().unwrap())
            })
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
