advent_of_code::solution!(15);

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, &curr| ((acc + usize::from(curr)) * 17) % 256)
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim_end().split(',').map(hash).sum::<usize>())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: [Vec<(&str, usize)>; 256] = std::array::from_fn(|_| Vec::new());
    input.trim_end().split(',').for_each(|input| {
        if input.contains('=') {
            let (label, focal_length) = input.split_once('=').unwrap();
            let focal_length: usize = focal_length.parse().unwrap();
            let hashed_label = hash(label);
            let position = boxes[hashed_label]
                .iter()
                .position(|(boxed_label, _)| boxed_label == &label);
            boxes[hashed_label].push((label, focal_length));
            if let Some(position) = position {
                boxes[hashed_label].swap_remove(position);
            }
        } else {
            let label = &input[..input.len() - 1];
            let hashed_label = hash(label);
            boxes[hashed_label].retain(|(boxed_label, _)| boxed_label != &label);
        }
    });
    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(box_index, box_contents)| {
                box_contents
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (slot, (_, focal_length))| {
                        acc + (1 + box_index) * (1 + slot) * focal_length
                    })
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
