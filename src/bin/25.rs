use std::collections::HashMap;

use itertools::Itertools;
use rand::seq::{IteratorRandom, SliceRandom};

advent_of_code::solution!(25);

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut components: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let from = line[0..3].to_string();
        let to = line[4..]
            .split_whitespace()
            .map(|v| v.to_string())
            .collect_vec();
        if let Some(links) = components.get_mut(&from) {
            links.append(&mut to.clone());
        } else {
            components.insert(from.clone(), to.clone());
        }
        for component in to {
            if let Some(links) = components.get_mut(&component) {
                links.push(from.clone());
            } else {
                components.insert(component.clone(), vec![from.clone()]);
            }
        }
    });
    components
}

pub fn part_one(input: &str) -> Option<usize> {
    let components = parse(input);
    let components: HashMap<String, (Vec<String>, usize)> = components
        .iter()
        .map(|(key, value)| (key.clone(), (value.clone(), 1)))
        .collect();

    let mut rng = rand::thread_rng();
    loop {
        let mut components_clone = components.clone();
        while components_clone.len() != 2 {
            let u_key = components_clone.keys().cloned().choose(&mut rng).unwrap();
            let (mut u_value, u_amount) = components_clone.get(&u_key).unwrap().clone();
            let v_key = u_value.choose(&mut rng).unwrap().clone();
            let (mut v_value, v_amount) = components_clone.get(&v_key).unwrap().clone();
            components_clone.remove(&v_key);
            components_clone.remove(&u_key);
            u_value.append(&mut v_value);
            u_value.retain(|component| component != &u_key && component != &v_key);
            let mut new_key = u_key.clone();
            new_key.push_str(&v_key.clone());
            for component in u_value.clone() {
                let (list, amount) = components_clone.get(&component).unwrap().clone();
                let new_list = list
                    .iter()
                    .map(|val| match val {
                        _ if val == &u_key => new_key.clone(),
                        _ if val == &v_key => new_key.clone(),
                        val => val.clone(),
                    })
                    .collect_vec();
                components_clone.insert(component, (new_list, amount));
            }
            components_clone.insert(new_key, (u_value, u_amount + v_amount));
        }
        println!("{components_clone:?}");
        let mut components_clone = components_clone.values();
        let (a_edges, a) = components_clone.next().unwrap();
        let (b_edges, b) = components_clone.next().unwrap();
        if a_edges.len() == 3 && b_edges.len() == 3 {
            return Some(a * b);
        }
    }
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
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
