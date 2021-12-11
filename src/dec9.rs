use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("../inputs/9.txt");

pub fn solve() {
    let solution_a = risk_levels(INPUT);
    println!("dec9, solution A {}", solution_a);

    let solution_b = map_basins(INPUT);
    println!("dec9, solution B {}", solution_b);
}

struct Map {
    width: usize,
    height: usize,
    points: Vec<u8>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let points: Vec<u8> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|n| n.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect();

        Ok(Self {
            width,
            height,
            points,
        })
    }
}

impl Map {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.points[xy_index(x, y, self.width)]
    }

    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push((x - 1, y));
        }

        if x < self.width - 1 {
            neighbours.push((x + 1, y));
        }

        if y > 0 {
            neighbours.push((x, y - 1));
        }

        if y < self.height - 1 {
            neighbours.push((x, y + 1));
        }

        neighbours
    }

    pub fn get_unvisited_neighbours(
        &self,
        x: usize,
        y: usize,
        visited: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        self.get_neighbours(x, y)
            .into_iter()
            .filter(|n| visited.contains(&(x, y)))
            .collect()
    }
}

fn risk_levels(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    let mut risk_levels = vec![];
    for y in 0..map.height {
        for x in 0..map.width {
            let point = map.get(x, y);
            if map
                .get_neighbours(x, y)
                .iter()
                .map(|(x, y)| map.get(*x, *y))
                .all(|n| point < n)
            {
                risk_levels.push((point + 1) as usize)
            }
        }
    }

    risk_levels.iter().sum()
}

fn map_basins(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    let mut low_points = vec![];
    for y in 0..map.height {
        for x in 0..map.width {
            let point = map.get(x, y);
            if map
                .get_neighbours(x, y)
                .iter()
                .map(|(x, y)| map.get(*x, *y))
                .all(|n| point < n)
            {
                low_points.push((x, y))
            }
        }
    }

    let mut basins = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (x, y) in low_points.iter() {
        basins.push(get_basin(*x, *y, &map, &mut visited));
    }

    basins.sort_by(|a, b| a.len().cmp(&b.len()));
    basins.iter().rev().take(3).fold(1, |acc, e| acc * e.len())
}

fn get_basin(
    x: usize,
    y: usize,
    map: &Map,
    visited: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut basin = vec![];
    basin.push((x, y));
    visited.insert((x, y));
    let neighbours = map.get_unvisited_neighbours(x, y, &visited);
    for (nx, ny) in neighbours.iter() {
        if visited.contains(&(*nx, *ny)) {
            continue;
        }

        visited.insert((*nx, *ny));
        if map.get(*nx, *ny) == 9 {
            continue;
        }

        if !basin.contains(&(*nx, *ny)) {
            basin.push((*nx, *ny));
        }

        for (ix, iy) in get_basin(*nx, *ny, map, visited) {
            if !basin.contains(&(ix, iy)) {
                basin.push((ix, iy));
            }
        }
    }

    basin
}

fn xy_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

#[cfg(test)]
mod tests {
    use crate::dec9::{map_basins, risk_levels};

    const INPUT_SAMPLE: &str = include_str!("../inputs/9_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(risk_levels(INPUT_SAMPLE), 15);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(map_basins(INPUT_SAMPLE), 1134);
    }
}
