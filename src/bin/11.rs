use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let line_mods = input
        .lines()
        .map(|line| !line.bytes().any(|byte| byte == b'#'))
        .map(|needs_expansion| if needs_expansion { 2 } else { 1 })
        .collect_vec();

    let line_length = input.chars().position(|c| c == '\n').unwrap();
    let col_mods = input
        .lines()
        .fold(vec![true; line_length], |mut mods, line| {
            line.bytes().enumerate().for_each(|(pos, byte)| {
                if byte == b'#' {
                    mods[pos] = false;
                };
            });
            mods
        })
        .iter()
        .map(|&needs_expansion| if needs_expansion { 2 } else { 1 })
        .collect_vec();

    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .fold(Vec::new(), |mut acc, (x, curr)| {
                    if curr == '#' {
                        acc.push((x as usize, y as usize));
                        acc
                    } else {
                        acc
                    }
                })
        })
        .collect_vec();
    Some(
        galaxies
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|((x1, y1), (x2, y2))| {
                (
                    if x1 <= x2 { *x1..*x2 } else { *x2..*x1 },
                    if y1 <= y2 { *y1..*y2 } else { *y2..*y1 },
                )
            })
            .map(|(x, y)| col_mods[x].iter().sum::<i32>() + line_mods[y].iter().sum::<i32>())
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let size = 1_000_000;
    let line_mods = input
        .lines()
        .map(|line| !line.bytes().any(|byte| byte == b'#'))
        .map(|needs_expansion| if needs_expansion { size } else { 1 })
        .collect_vec();

    let line_length = input.chars().position(|c| c == '\n').unwrap();
    let col_mods = input
        .lines()
        .fold(vec![true; line_length], |mut mods, line| {
            line.bytes().enumerate().for_each(|(pos, byte)| {
                if byte == b'#' {
                    mods[pos] = false;
                };
            });
            mods
        })
        .iter()
        .map(|&needs_expansion| if needs_expansion { size } else { 1 })
        .collect_vec();

    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .fold(Vec::new(), |mut acc, (x, curr)| {
                    if curr == '#' {
                        acc.push((x as usize, y as usize));
                        acc
                    } else {
                        acc
                    }
                })
        })
        .collect_vec();
    Some(
        galaxies
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|((x1, y1), (x2, y2))| {
                (
                    if x1 <= x2 { *x1..*x2 } else { *x2..*x1 },
                    if y1 <= y2 { *y1..*y2 } else { *y2..*y1 },
                )
            })
            .map(|(x, y)| col_mods[x].iter().sum::<i64>() + line_mods[y].iter().sum::<i64>())
            .sum::<i64>() as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
