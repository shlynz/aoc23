use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug)]
enum ModuleType {
    PassThrough,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    module_type: ModuleType,
    targets: Vec<&'a str>,
}

impl<'a> From<&'a str> for Module<'a> {
    fn from(value: &'a str) -> Self {
        let (name, destinations) = value.split_once(" -> ").unwrap();
        let targets = destinations.split_terminator(", ").collect_vec();

        let module_type = match value {
            value if value.starts_with('%') => ModuleType::FlipFlop,
            value if value.starts_with('&') => ModuleType::Conjunction,
            _ => ModuleType::PassThrough,
        };

        let name = match module_type {
            ModuleType::PassThrough => name,
            _ => &name[1..],
        };

        Module {
            name,
            module_type,
            targets,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let modules = input
        .lines()
        .map(|line| Module::from(line))
        .map(|module| (module.name, module))
        .collect::<HashMap<&str, Module>>();

    let mut flip_flops = HashMap::new();
    let mut conjunctions = HashMap::new();
    modules.iter().for_each(|(_, origin_module)| {
        origin_module.targets.iter().for_each(|target| {
            if let Some(module) = modules.get(target) {
                match module.module_type {
                    ModuleType::FlipFlop => {
                        flip_flops.insert(target, false);
                    }
                    ModuleType::Conjunction => {
                        let input_states = conjunctions.entry(target).or_insert(HashMap::new());
                        input_states.insert(origin_module.name, false);
                    }
                    _ => return,
                }
            }
        });
    });

    let mut lows = 0;
    let mut highs = 0;

    for _ in 0..1000 {
        let mut to_check = VecDeque::new();
        modules
            .get("broadcaster")
            .unwrap()
            .targets
            .iter()
            .for_each(|target| to_check.push_back(("broadcaster", target, false)));
        lows += 1;
        //println!("");

        while let Some((source, target, edge)) = to_check.pop_front() {
            if edge {
                highs += 1;
            } else {
                lows += 1;
            }

            //println!("{source:?} -{edge}-> {target:?}");
            if !modules.keys().contains(target) {
                continue;
            }
            let next_module = modules.get(target).unwrap();
            let next_source = target;

            match next_module.module_type {
                ModuleType::FlipFlop => {
                    if !edge {
                        let prev_val = flip_flops.get_mut(target).unwrap();
                        *prev_val = !*prev_val;
                        next_module.targets.iter().for_each(|target| {
                            to_check.push_back((next_source, target, *prev_val))
                        });
                    }
                }
                ModuleType::Conjunction => {
                    let states = conjunctions.get_mut(target).unwrap();
                    let state = states.get_mut(source).unwrap();

                    *state = edge;

                    let any_lows = states.values().filter(|&&val| !val).count();
                    let any_lows = any_lows != 0;
                    next_module
                        .targets
                        .iter()
                        .for_each(|target| to_check.push_back((next_source, target, any_lows)));
                }
                ModuleType::PassThrough => unimplemented!(),
            }
        }
    }

    println!("{lows}, {highs}");
    Some(lows * highs)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4250 * 2750));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
