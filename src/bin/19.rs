use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(19);

#[derive(Debug, Clone, Copy)]
enum RuleResult<'a> {
    NextRule(&'a str),
    FinalStatus(bool),
}

impl<'a> RuleResult<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => RuleResult::FinalStatus(true),
            "R" => RuleResult::FinalStatus(false),
            _ => RuleResult::NextRule(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    category: char,
    comparison_type: std::cmp::Ordering,
    comparison: usize,
    destination: RuleResult<'a>,
}

impl<'a> Rule<'a> {
    fn apply(&self, part: &Part) -> bool {
        match self.category {
            'x' => part.x.cmp(&self.comparison) == self.comparison_type,
            'm' => part.m.cmp(&self.comparison) == self.comparison_type,
            'a' => part.a.cmp(&self.comparison) == self.comparison_type,
            's' => part.s.cmp(&self.comparison) == self.comparison_type,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct RuleCategory<'a> {
    rules: Vec<Rule<'a>>,
    fallthrough: RuleResult<'a>,
}

impl<'a> RuleCategory<'a> {
    fn apply(&self, part: &Part) -> RuleResult {
        if let Some(result) = self.rules.iter().find(|rule| rule.apply(&part)) {
            result.destination
        } else {
            self.fallthrough
        }
    }
}

#[derive(Debug)]
struct Rules<'a> {
    rules: HashMap<&'a str, RuleCategory<'a>>,
}

impl<'a> Rules<'a> {
    fn apply(&self, starting_rule_name: &'a str, part: &Part) -> bool {
        let starting_rule = self.rules.get(starting_rule_name).unwrap();
        let mut next_rule = starting_rule.apply(&part);
        while let RuleResult::NextRule(rule_name) = next_rule {
            let rule = self.rules.get(rule_name).unwrap();
            next_rule = rule.apply(&part);
        }

        if let RuleResult::FinalStatus(result) = next_rule {
            result
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn score(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse(input: &str) -> (Rules, Vec<Part>) {
    let (rules, parts) = input.split_once("\n\n").unwrap();

    let rules = Rules {
        rules: rules
            .lines()
            .map(|line| {
                let name_delimiter = line.chars().position(|c| c == '{').unwrap();
                let fallthrough_delimiter =
                    line.len() - line.chars().rev().position(|c| c == ',').unwrap();

                let name = &line[0..name_delimiter];
                let fallthrough = RuleResult::from(&line[fallthrough_delimiter..line.len() - 1]);
                let rules = &line[name_delimiter + 1..fallthrough_delimiter];

                let mut rules_cut = &rules[..];
                let mut rules_vec = Vec::new();
                while let Some(pos) = rules_cut.chars().position(|c| c == ',') {
                    let category = rules_cut.chars().nth(0).unwrap();
                    let comparison_type = match rules_cut.chars().nth(1).unwrap() {
                        '<' => std::cmp::Ordering::Less,
                        '>' => std::cmp::Ordering::Greater,
                        _ => unreachable!(),
                    };
                    let destination_delimiter = rules_cut.chars().position(|c| c == ':').unwrap();
                    let comparison = (&rules_cut[2..destination_delimiter])
                        .parse::<usize>()
                        .unwrap();
                    let destination = RuleResult::from(&rules_cut[destination_delimiter + 1..pos]);
                    rules_vec.push(Rule {
                        category,
                        comparison,
                        comparison_type,
                        destination,
                    });
                    rules_cut = &rules_cut[pos + 1..];
                }

                (
                    name,
                    RuleCategory {
                        rules: rules_vec,
                        fallthrough,
                    },
                )
            })
            .collect(),
    };

    let parts = parts
        .lines()
        .map(|line| {
            let categories = line[..line.len() - 1]
                .split(',')
                .map(|split| split.split_once('=').unwrap().1)
                .map(|str_nr| str_nr.parse::<usize>().unwrap())
                .collect_vec();
            Part {
                x: categories[0],
                m: categories[1],
                a: categories[2],
                s: categories[3],
            }
        })
        .collect_vec();

    (rules, parts)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, parts) = parse(input);
    rules.rules.iter().for_each(|rule| println!("{rule:?}"));
    println!("----------");
    parts.iter().for_each(|part| println!("{part:?}"));

    Some(
        parts
            .iter()
            .filter(|part| rules.apply(&"in", &part))
            .map(|part| part.score())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, _) = parse(input);
    rules.rules.iter().for_each(|rule| println!("{rule:?}"));
    println!("----------");
    Some(
        (0..4)
            .map(|_| 0..=4000)
            .multi_cartesian_product()
            .map(|vals| Part {
                x: vals[0],
                m: vals[1],
                a: vals[2],
                s: vals[3],
            })
            .filter(|part| rules.apply(&"in", &part))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}