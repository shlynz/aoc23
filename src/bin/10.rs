use std::{collections::HashMap, usize};

use itertools::Itertools;
advent_of_code::solution!(10);

fn get_dir(char: &u8) -> Vec<(i8, i8)> {
    match char {
        b'|' => vec![(0, -1), (0, 1)],
        b'-' => vec![(-1, 0), (1, 0)],
        b'L' => vec![(0, -1), (1, 0)],
        b'J' => vec![(0, -1), (-1, 0)],
        b'7' => vec![(-1, 0), (0, 1)],
        b'F' => vec![(0, 1), (1, 0)],
        b'S' => vec![(0, 0), (0, 0)],
        _ => unreachable!(),
    }
}

fn apply_dir(curr: &(usize, usize), dir: &(i8, i8)) -> (usize, usize) {
    (
        curr.0.wrapping_add_signed(dir.0 as isize),
        curr.1.wrapping_add_signed(dir.1 as isize),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let segments = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, char)| ((x, y), char))
        })
        .filter(|(_, char)| char != &b'.')
        .collect::<HashMap<(usize, usize), u8>>();
    let mut curr_coords = segments.iter().find(|(_, char)| char == &&b'S').unwrap().0;
    let mut curr_char = b' ';
    let curr_dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut curr_dir = *curr_dir
        .iter()
        .find(
            |dir| match segments.get_key_value(&apply_dir(curr_coords, dir)) {
                Some((_, next_char)) => get_dir(next_char)
                    .iter()
                    .map(|(x, y)| (-x, -y))
                    .find(|(x, y)| &dir.0 == x && &dir.1 == y)
                    .is_some(),
                None => false,
            },
        )
        .unwrap();
    let mut length = 0;
    while curr_char != b'S' {
        length += 1;
        let (next_coords, next_char) = segments
            .get_key_value(&apply_dir(curr_coords, &curr_dir))
            .unwrap();
        let next_dir = get_dir(&next_char);
        let next_dir = next_dir
            .iter()
            .find(|(x, y)| !(curr_dir.0 == -x && curr_dir.1 == -y))
            .unwrap();
        curr_dir = *next_dir;
        curr_coords = next_coords;
        curr_char = *next_char;
    }
    Some(length / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let segments = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, char)| ((x, y), char))
        })
        .filter(|(_, char)| char != &b'.')
        .collect::<HashMap<(usize, usize), u8>>();
    let mut curr_coords = segments.iter().find(|(_, char)| char == &&b'S').unwrap().0;
    let mut curr_char = b' ';
    let curr_dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut curr_dir = *curr_dir
        .iter()
        .find(
            |dir| match segments.get_key_value(&apply_dir(curr_coords, dir)) {
                Some((_, next_char)) => get_dir(next_char)
                    .iter()
                    .map(|(x, y)| (-x, -y))
                    .find(|(x, y)| &dir.0 == x && &dir.1 == y)
                    .is_some(),
                None => false,
            },
        )
        .unwrap();
    let mut main_pipe = Vec::new();
    while curr_char != b'S' {
        main_pipe.push(curr_coords);
        let (next_coords, next_char) = segments
            .get_key_value(&apply_dir(curr_coords, &curr_dir))
            .unwrap();
        let next_dir = get_dir(&next_char);
        let next_dir = next_dir
            .iter()
            .find(|(x, y)| !(curr_dir.0 == -x && curr_dir.1 == -y))
            .unwrap();
        curr_dir = *next_dir;
        curr_coords = next_coords;
        curr_char = *next_char;
    }
    let to_ignore = vec![b'-', b'7', b'F'];
    let i_dont_even_anymore = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if main_pipe.contains(&&(x, y)) {
                        if !to_ignore.contains(segments.get(&(x, y)).unwrap()) {
                            'X'
                        } else {
                            'x'
                        }
                    } else {
                        char
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    Some(
        i_dont_even_anymore
            .iter()
            .map(|line| {
                line.iter()
                    .fold((0, false), |(amount, curr_inside), curr| match *curr {
                        'X' => (amount, !curr_inside),
                        'x' => (amount, curr_inside),
                        _ => {
                            if curr_inside {
                                (amount + 1, curr_inside)
                            } else {
                                (amount, curr_inside)
                            }
                        }
                    })
            })
            .map(|(amount, _)| amount as u32)
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
