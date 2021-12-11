const INPUT: &str = include_str!("../inputs/7.txt");

pub fn solve() {
    let solution_a = align(INPUT);
    println!("dec7, solution A {}", solution_a);

    let solution_b = align_increasing_costs(INPUT);
    println!("dec7, solution B {}", solution_b);
}

fn align(input: &str) -> usize {
    let positions: Vec<usize> = input.split(",").map(|f| f.parse().unwrap()).collect();
    let max = *positions.iter().max().unwrap();
    let min = *positions.iter().min().unwrap();

    let mut costs = vec![];
    for (i, pos) in positions.iter().enumerate() {
        let distances: Vec<usize> = (min..=max)
            .map(|p| (*pos as isize - p as isize).abs() as usize)
            .collect();
        costs.insert(i, distances);
    }

    let mut fuel_costs: Vec<usize> = vec![];
    for i in min..=max {
        let mut position_cost = vec![];
        for cost in costs.iter() {
            position_cost.push(cost[i])
        }

        fuel_costs.push(position_cost.iter().sum());
    }

    *fuel_costs.iter().min().unwrap()
}

fn align_increasing_costs(input: &str) -> usize {
    let positions: Vec<usize> = input.split(",").map(|f| f.parse().unwrap()).collect();
    let max = *positions.iter().max().unwrap();
    let min = *positions.iter().min().unwrap();

    let mut costs = vec![];
    for (i, pos) in positions.iter().enumerate() {
        let from_to: Vec<usize> = (min..=max).map(|to| fuel_costs(*pos, to)).collect();
        costs.insert(i, from_to);
    }

    let mut fuel_costs: Vec<usize> = vec![];
    for i in min..=max {
        let mut position_cost = vec![];
        for cost in costs.iter() {
            position_cost.push(cost[i])
        }

        fuel_costs.push(position_cost.iter().sum());
    }

    *fuel_costs.iter().min().unwrap()
}

fn sum_consecutive(from: usize, to: usize) -> usize {
    let (from, to) = if from < to { (from, to) } else { (to, from) };

    let n = (to - from + 1) as f32;

    ((n / 2.0) * (from + to) as f32) as usize
}

fn fuel_costs(from: usize, to: usize) -> usize {
    if from == to {
        return 0;
    }

    let n = (to as isize - from as isize).abs();

    sum_consecutive(1, n as usize)
}

#[cfg(test)]
mod tests {
    use crate::dec7::{align, align_increasing_costs, sum_consecutive};

    const INPUT_SAMPLE: &str = include_str!("../inputs/7_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(align(INPUT_SAMPLE), 37);
    }

    #[test]
    fn sum() {
        assert_eq!(sum_consecutive(1, 2), 3);
        assert_eq!(sum_consecutive(4, 8), 30);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(align_increasing_costs(INPUT_SAMPLE), 168);
    }
}
