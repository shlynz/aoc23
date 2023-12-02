use regex::Regex;
advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let game_id: Regex = Regex::new(r"(Game )(\d+)").unwrap();
    let red: Regex = Regex::new(r"(\d+)( red)").unwrap();
    let green: Regex = Regex::new(r"(\d+)( green)").unwrap();
    let blue: Regex = Regex::new(r"(\d+)( blue)").unwrap();

    Some(
        input
            .lines()
            .into_iter()
            .map(|line| Game {
                id: game_id
                    .captures(line)
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
                red: red
                    .captures_iter(line)
                    .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    .max()
                    .unwrap(),
                green: green
                    .captures_iter(line)
                    .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    .max()
                    .unwrap(),
                blue: blue
                    .captures_iter(line)
                    .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    .max()
                    .unwrap(),
            })
            .filter(|game| game.red <= MAX_RED && game.green <= MAX_GREEN && game.blue <= MAX_BLUE)
            .fold(0, |acc, game| acc + game.id),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let game_id: Regex = Regex::new(r"(Game )(\d+)").unwrap();
    let red: Regex = Regex::new(r"(\d+)( red)").unwrap();
    let green: Regex = Regex::new(r"(\d+)( green)").unwrap();
    let blue: Regex = Regex::new(r"(\d+)( blue)").unwrap();

    Some(
        input
            .lines()
            .into_iter()
            .map(|line| Game {
                id: game_id
                    .captures(line)
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
                red: red
                    .captures_iter(line)
                    .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    .max()
                    .unwrap(),
                green: green
                    .captures_iter(line)
                    .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    .max()
                    .unwrap(),
                blue: blue
                    .captures_iter(line)
                    .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
                    .max()
                    .unwrap(),
            })
            .map(|game| game.red * game.green * game.blue)
            .fold(0, |acc, game_power| acc + game_power),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
