use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy)]
struct Hailstorm {
    px: usize,
    py: usize,
    pz: usize,
    vx: isize,
    vy: isize,
    vz: isize,
}

impl Hailstorm {
    fn intersect(&self, other: Hailstorm) -> bool {
        let min = 200000000000000.0;
        let max = 400000000000000.0;
        let hail_one = SlopeIntercept::from(*self);
        let hail_two = SlopeIntercept::from(other);
        if hail_one.m == hail_two.m {
            false
        } else {
            let x = (hail_two.b - hail_one.b) as f64 / (hail_one.m - hail_two.m) as f64;
            let y = hail_one.m * x + hail_one.b;

            if x < min || x > max || y < min || y > max {
                false
            } else {
                let in_past = |hail: Hailstorm| {
                    ((x - hail.px as f64) / hail.vx as f64) < 0.0
                        && ((y - hail.py as f64) / hail.vy as f64) < 0.0
                };

                !in_past(*self) && !in_past(other)
            }
        }
    }
}

#[derive(Debug)]
struct SlopeIntercept {
    m: f64,
    b: f64,
}

impl From<Hailstorm> for SlopeIntercept {
    fn from(value: Hailstorm) -> Self {
        let m = value.vy as f64 / value.vx as f64;
        let b = value.py as f64 - (m * value.px as f64);

        SlopeIntercept { m, b }
    }
}

fn parse(input: &str) -> Vec<Hailstorm> {
    let parser = Regex::new(
        r"(?<px>\d+), (?<py>\d+), (?<pz>\d+) @ +(?<vx>-*\d+), +(?<vy>-*\d+), +(?<vz>-*\d+)",
    )
    .unwrap();
    input
        .lines()
        .filter_map(|line| {
            if let Some(captures) = parser.captures(line) {
                let (_, [px, py, pz, vx, vy, vz]) = captures.extract();
                let px = px.parse::<usize>().unwrap();
                let py = py.parse::<usize>().unwrap();
                let pz = pz.parse::<usize>().unwrap();
                let vx = vx.parse::<isize>().unwrap();
                let vy = vy.parse::<isize>().unwrap();
                let vz = vz.parse::<isize>().unwrap();
                Some(Hailstorm {
                    px,
                    py,
                    pz,
                    vx,
                    vy,
                    vz,
                })
            } else {
                None
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let hailstorms = parse(input);
    Some(
        hailstorms
            .iter()
            .tuple_combinations()
            .filter(|(first, &second)| first.intersect(second))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let hailstorms = parse(input);
    let hailstorms = hailstorms.iter().take(3).collect_vec();
    println!("{:?}", hailstorms);
    println!("(79 u v + 276481733510955 u - 359781776524153 v)/(u - v) = 420851642592931");
    println!("(5 (u (29 v + 54173413157932) - 62541132055815 v))/(u - v) = 273305746686315");
    println!("(3 u (5 v + 91256287537271) - 236728636905923 v)/(u - v) = 176221626745613");
    println!("(-44 u - 35 v + 83300043013198)/(u - v) = -261");
    println!("-(5 (25 u + 4 v - 8367718897883))/(u - v) = 15");
    println!("(18 u - 33 v - 37040225705890)/(u - v) = 233");
    println!("With t=637228617556,v=487736179331,u=281427954234");
    println!("{}", "https://matrixcalc.org/slu.html#solve-using-Gaussian-elimination(%7B%7B1,0,0,u,0,0,0,0,0,-44*u+359781776524153%7D,%7B0,1,0,0,u,0,0,0,0,-125*u+312705660279075%7D,%7B0,0,1,0,0,u,0,0,0,18*u+236728636905923%7D,%7B1,0,0,v,0,0,0,0,0,35*v+276481733510955%7D,%7B0,1,0,0,v,0,0,0,0,20*v+270867065789660%7D,%7B0,0,1,0,0,v,0,0,0,33*v+273768862611813%7D,%7B1,0,0,t,0,0,0,0,0,102*t+189537654420103%7D,%7B0,1,0,0,t,0,0,0,0,-15*t+292422605212995%7D,%7B0,0,1,0,0,t,0,0,0,-14*t+333617095281945%7D%7D)");
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
