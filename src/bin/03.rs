use regex::Regex;
advent_of_code::solution!(3);

#[derive(Debug)]
struct ParsedInput {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Part {
    nr: i32,
    y: i32,
    x_start: i32,
    x_end: i32,
}

#[derive(Debug)]
struct Symbol {
    r#type: String,
    x: i32,
    y: i32,
}

fn parse(input: &str) -> ParsedInput {
    let line_parser = Regex::new(r"(?<nr>\d+)|(?<symbol>[^\d.]+)").unwrap();
    input.lines().enumerate().fold(
        ParsedInput {
            parts: Vec::new(),
            symbols: Vec::new(),
        },
        |mut acc, enumeration| {
            let (y, line) = enumeration;
            line_parser.captures_iter(line).for_each(|capture| {
                if let Some(capture) = capture.name("nr") {
                    acc.parts.push(Part {
                        nr: capture.as_str().parse::<i32>().unwrap(),
                        y: y as i32,
                        x_start: capture.start() as i32,
                        x_end: capture.end() as i32 - 1,
                    })
                } else if let Some(capture) = capture.name("symbol") {
                    acc.symbols.push(Symbol {
                        r#type: capture.as_str().to_owned(),
                        x: capture.start() as i32,
                        y: y as i32,
                    })
                }
            });
            acc
        },
    )
}
pub fn part_one(input: &str) -> Option<u32> {
    let ParsedInput { parts, symbols } = parse(input);
    Some(
        parts
            .iter()
            .filter(|part| {
                symbols.iter().any(|symbol| {
                    symbol.y <= part.y + 1
                        && symbol.y >= part.y - 1
                        && symbol.x <= part.x_end + 1
                        && symbol.x >= part.x_start - 1
                })
            })
            .fold(0, |acc, curr| acc + curr.nr as u32),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let ParsedInput { parts, symbols } = parse(input);
    Some(
        symbols
            .iter()
            .filter(|symbol| symbol.r#type == "*")
            .map(|symbol| {
                parts
                    .iter()
                    .filter(|part| {
                        symbol.y <= part.y + 1
                            && symbol.y >= part.y - 1
                            && symbol.x <= part.x_end + 1
                            && symbol.x >= part.x_start - 1
                    })
                    .collect::<Vec<&Part>>()
            })
            .filter(|parts| parts.len() == 2)
            .map(|parts| (parts[0].nr * parts[1].nr) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
