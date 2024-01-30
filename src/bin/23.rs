use std::{
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

advent_of_code::solution!(23);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn all() -> Vec<Self> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

impl Into<Coordinates> for Direction {
    fn into(self) -> Coordinates {
        match self {
            Direction::Up => Coordinates { x: 0, y: -1 },
            Direction::Right => Coordinates { x: 1, y: 0 },
            Direction::Down => Coordinates { x: 0, y: 1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let y_cmp = self.y.cmp(&other.y);
        return Some(if y_cmp == std::cmp::Ordering::Equal {
            self.x.cmp(&other.x)
        } else {
            y_cmp
        });
    }
}

impl Ord for Coordinates {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Coordinates {
    fn new(x: isize, y: isize) -> Self {
        Coordinates { x, y }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {:2}, y: {:2}", self.x, self.y)
    }
}

impl std::ops::Add<Self> for Coordinates {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Grid<T> {
    grid: BTreeMap<Coordinates, T>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

impl<T: Clone> Grid<T> {
    fn new(grid: BTreeMap<Coordinates, T>) -> Self {
        let coordinates = grid.iter().map(|(coords, _)| coords);
        let x_coords = coordinates.clone().map(|coord| coord.x);
        let y_coords = coordinates.clone().map(|coord| coord.y);
        let min_x = x_coords.clone().min().unwrap();
        let max_x = x_coords.clone().max().unwrap();
        let min_y = y_coords.clone().min().unwrap();
        let max_y = y_coords.clone().max().unwrap();

        Grid {
            grid,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn get(&self, coords: &Coordinates) -> Option<T> {
        if let Some(item) = self.grid.get(coords) {
            Some(item.clone())
        } else {
            None
        }
    }
}

impl Grid<char> {
    fn get_adjacent(&self, coords: Coordinates, part2: bool) -> Vec<(Direction, Coordinates)> {
        Direction::all()
            .into_iter()
            .map(|dir| (dir, coords + dir.into()))
            .filter(|(dir, coords)| {
                if let Some(item) = self.get(coords) {
                    return if item == '#' {
                        false
                    } else if vec!['^', '>', 'v', '<'].contains(&item) {
                        match (dir, item) {
                            (Direction::Up, '^') => true,
                            (Direction::Right, '>') => true,
                            (Direction::Down, 'v') => true,
                            (Direction::Left, '<') => true,
                            (_, _) => part2,
                        }
                    } else {
                        true
                    };
                } else {
                    false
                }
            })
            .collect_vec()
    }

    fn dijkstra(
        &self,
        start: &Coordinates,
        end: &Coordinates,
        weight_per_step: isize,
        part2: bool,
    ) -> Option<isize> {
        let mut costs: HashMap<Coordinates, isize> = HashMap::new();
        let mut to_check: BinaryHeap<
            std::cmp::Reverse<(isize, Direction, Coordinates, Vec<Coordinates>)>,
        > = BinaryHeap::new();

        to_check.push(std::cmp::Reverse((0, Direction::Down, *start, Vec::new())));

        while let Some(std::cmp::Reverse((score, last_dir, coords, prev_visited))) = to_check.pop()
        {
            if prev_visited.contains(&coords) {
                continue;
            }
            if let Some(&prev_best) = costs.get(&coords).or(Some(&isize::MAX)) {
                if prev_best > score {
                    costs.insert(coords, score);
                }
            }
            let mut next_visited = prev_visited.clone();
            next_visited.push(coords);
            self.get_adjacent(coords, part2)
                .iter()
                .filter(|(dir, _)| dir != &last_dir.opposite())
                .for_each(|(dir, step)| {
                    to_check.push(std::cmp::Reverse((
                        score + weight_per_step,
                        *dir,
                        *step,
                        next_visited.clone(),
                    )))
                });
        }

        if let Some(&score) = costs.get(&end) {
            Some(score)
        } else {
            None
        }
    }
}

impl<T: Eq> Grid<T> {
    fn find_first(&self, to_find: T) -> Option<Coordinates> {
        if let Some((&coords, _)) = self.grid.iter().find(|(_, item)| item == &&to_find) {
            Some(coords)
        } else {
            None
        }
    }

    fn find_last(&self, to_find: T) -> Option<Coordinates> {
        if let Some((&coords, _)) = self.grid.iter().rev().find(|(_, item)| item == &&to_find) {
            Some(coords)
        } else {
            None
        }
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_y = self.min_y;
        let mut result = String::new();
        for (key, &value) in self.grid.iter() {
            if key.y != last_y {
                result.push('\n');
                last_y = key.y;
            }
            result.push(value);
        }
        write!(f, "{result}")
    }
}

impl<T: From<u8> + Clone> From<&str> for Grid<T> {
    fn from(value: &str) -> Self {
        Grid::new(
            value
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    let y: isize = y.try_into().unwrap();
                    line.bytes()
                        .enumerate()
                        .map(|(x, item)| (Coordinates::new(x.try_into().unwrap(), y), item.into()))
                        .collect_vec()
                })
                .collect(),
        )
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let grid: Grid<char> = Grid::from(input);
    println!("{grid}");

    if let (Some(start_coords), Some(end_coords)) = (grid.find_first('.'), grid.find_last('.')) {
        if let Some(result) = grid.dijkstra(&start_coords, &end_coords, -1, false) {
            return Some(result.abs());
        }
    };
    None
}

fn get_junction_coords(grid: &Grid<char>) -> Vec<Coordinates> {
    let mut junctions = Vec::new();
    for x in grid.min_x..=grid.max_x {
        for y in grid.min_y..=grid.max_y {
            let curr_coord = Coordinates { x, y };
            if grid.get(&curr_coord).unwrap_or('#') != '#'
                && grid.get_adjacent(curr_coord, true).len() > 2
            {
                junctions.push(curr_coord);
            }
        }
    }
    let end_coords = grid.find_last('.').unwrap();
    junctions.push(end_coords);

    junctions
}

struct Edge {
    from: Coordinates,
    to: Coordinates,
    weight: usize,
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t->\t{},\tcosts: {}", self.from, self.to, self.weight)
    }
}

struct Neighbour {
    coords: Coordinates,
    weight: usize,
}

fn get_simple_graph(grid: &Grid<char>) -> HashMap<Coordinates, Vec<Neighbour>> {
    let junctions = get_junction_coords(&grid);
    let mut edges = Vec::new();

    struct State {
        coords: Coordinates,
        steps: usize,
        last_junction: Coordinates,
    }
    let mut to_check = Vec::new();
    let mut seen = HashSet::new();
    let start_coords = grid.find_first('.').unwrap();
    to_check.push(State {
        coords: start_coords,
        steps: 1,
        last_junction: start_coords,
    });
    seen.insert(start_coords);
    while let Some(state) = to_check.pop() {
        for (_, coords) in grid.get_adjacent(state.coords, false) {
            let mut last_junction = state.last_junction;
            let mut steps = state.steps + 1;
            if junctions.contains(&coords) && steps > 2 {
                edges.push(Edge {
                    from: last_junction,
                    to: coords,
                    weight: steps,
                });
                last_junction = coords;
                steps = 0;
            }
            if seen.contains(&coords) {
                continue;
            }
            to_check.push(State {
                coords: coords.clone(),
                steps,
                last_junction,
            });
            seen.insert(coords);
        }
    }

    let mut nodes: HashMap<Coordinates, Vec<Neighbour>> = HashMap::new();
    edges.iter().for_each(|edge| {
        if let Some(neighbours) = nodes.get_mut(&edge.from) {
            neighbours.push(Neighbour {
                coords: edge.to,
                weight: edge.weight,
            });
        } else {
            nodes.insert(
                edge.from,
                vec![Neighbour {
                    coords: edge.to,
                    weight: edge.weight,
                }],
            );
        }
        if let Some(neighbours) = nodes.get_mut(&edge.to) {
            neighbours.push(Neighbour {
                coords: edge.from,
                weight: edge.weight,
            });
        } else {
            nodes.insert(
                edge.to,
                vec![Neighbour {
                    coords: edge.from,
                    weight: edge.weight,
                }],
            );
        }
    });

    nodes
}

fn search_simple_graph(
    current: Coordinates,
    visited: &mut Vec<Coordinates>,
    graph: &HashMap<Coordinates, Vec<Neighbour>>,
    steps: usize,
    end: Coordinates,
) -> usize {
    if current == end {
        steps
    } else {
        let mut visited_clone = visited.clone();
        visited_clone.push(current);
        let mut results = vec![0];
        for neighbour in graph
            .get(&current)
            .unwrap()
            .iter()
            .filter(|n| !visited.contains(&n.coords))
        {
            results.push(search_simple_graph(
                neighbour.coords,
                &mut visited_clone.clone(),
                graph,
                steps + neighbour.weight,
                end,
            ));
        }
        *results.iter().max().unwrap()
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::from(input);
    println!("{grid}");

    let graph = get_simple_graph(&grid);

    Some(
        search_simple_graph(
            grid.find_first('.').unwrap(),
            &mut Vec::new(),
            &graph,
            0,
            grid.find_last('.').unwrap(),
        ) - 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
