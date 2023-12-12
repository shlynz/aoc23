use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

fn reject_rust_return_to_haskell(
    springs: &[u8],
    groups: &[usize],
    collected: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if springs.is_empty() {
        return match (collected, groups.len()) {
            (0, 0) => 1,
            (n, 1) if groups[0] == n => 1,
            _ => 0,
        };
    }

    if collected != 0 && groups.is_empty() {
        return 0;
    }

    let memo_key = (springs.len(), groups.len(), collected);
    if let Some(&result) = cache.get(&memo_key) {
        return result;
    }

    let result = match (springs[0], collected) {
        (b'.', 0) => reject_rust_return_to_haskell(&springs[1..], groups, 0, cache),
        (b'.', n) if n != groups[0] => 0,
        (b'.', _) => reject_rust_return_to_haskell(&springs[1..], &groups[1..], 0, cache),
        (b'#', 0) => reject_rust_return_to_haskell(&springs[1..], groups, 1, cache),
        (b'#', n) => reject_rust_return_to_haskell(&springs[1..], groups, n + 1, cache),
        (b'?', 0) => {
            reject_rust_return_to_haskell(&springs[1..], groups, 0, cache)
                + reject_rust_return_to_haskell(&springs[1..], groups, 1, cache)
        }
        (b'?', n) if groups.len() != 0 && n == groups[0] => {
            reject_rust_return_to_haskell(&springs[1..], &groups[1..], 0, cache)
        }
        (b'?', n) => reject_rust_return_to_haskell(&springs[1..], groups, n + 1, cache),
        _ => panic!("how did we end up here?"),
    };

    cache.insert(memo_key, result);
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut memoization_cache = HashMap::new();
                let (springs, groups) = line.split_once(' ').unwrap();
                let groups = groups
                    .split(',')
                    .map(|nr_str| nr_str.parse::<usize>().unwrap())
                    .collect_vec();
                reject_rust_return_to_haskell(
                    springs.as_bytes(),
                    &groups,
                    0,
                    &mut memoization_cache,
                )
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut memoization_cache = HashMap::new();
                let (springs, groups) = line.split_once(' ').unwrap();
                let groups = groups
                    .split(',')
                    .map(|nr_str| nr_str.parse::<usize>().unwrap())
                    .collect_vec();
                let springs = vec![springs; 5].join("?");
                let groups = vec![groups; 5].concat();
                reject_rust_return_to_haskell(
                    springs.as_bytes(),
                    &groups,
                    0,
                    &mut memoization_cache,
                )
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
