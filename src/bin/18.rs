use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Debug)]
struct Input {
    dir: Direction,
    amount: isize,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let parts = value.split_whitespace().collect_vec();
        Input {
            dir: Direction::from(parts[0]),
            amount: parts[1].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn add(&self, origin: (isize, isize), amount: isize) -> (isize, isize) {
        let (x, y) = origin;
        match self {
            Direction::Up => (x, y - amount),
            Direction::Right => (x + amount, y),
            Direction::Down => (x, y + amount),
            Direction::Left => (x - amount, y),
        }
    }
}

fn shoelace_formula(input: Vec<Input>) -> isize {
    let mut last_pos = (0_isize, 0_isize);
    let input = input
        .iter()
        .map(|curr| {
            let curr_pos = curr.dir.add(last_pos, curr.amount);
            last_pos = curr_pos;
            curr_pos
        })
        .collect_vec();
    let inner_area = isize::abs(
        input
            .iter()
            .circular_tuple_windows()
            .map(|(curr, next)| (next.0 + curr.0, next.1 - curr.1))
            .map(|(x, y)| x * y)
            .sum::<isize>(),
    );
    let border = input
        .iter()
        .circular_tuple_windows()
        .map(|(curr, next)| isize::abs(curr.0 - next.0) + isize::abs(curr.1 - next.1))
        .sum::<isize>();
    (inner_area.abs() + border) / 2 + 1
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(shoelace_formula(
        input.lines().map(|line| Input::from(line)).collect_vec(),
    ))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(shoelace_formula(
        input
            .lines()
            .map(|line| {
                let parts = line.split_whitespace().collect_vec();
                let hex = parts[2];
                let hex = &hex[2..(hex.len() - 1)];
                let dir = match u8::from_str_radix(&hex[(hex.len() - 1)..], 16).unwrap() {
                    0 => Direction::Right,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Up,
                    _ => unreachable!(),
                };
                let amount = isize::from_str_radix(&hex[0..(hex.len() - 1)], 16).unwrap();
                Input { dir, amount }
            })
            .collect_vec(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
