use std::str::FromStr;

const INPUT: &str = include_str!("../inputs/2.txt");

pub fn solve() {
    let solution_a = dive(INPUT);
    println!("dec2, solution A {}", solution_a);

    let solution_b = dive_aim(INPUT);
    println!("dec2, solution B {}", solution_b);
}

enum Instruction {
    UP(usize),
    DOWN(usize),
    FORWARD(usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, amount) = s.split_once(" ").unwrap();
        let amount = amount.parse().unwrap();

        match instruction {
            "up" => Ok(Instruction::UP(amount)),
            "down" => Ok(Instruction::DOWN(amount)),
            "forward" => Ok(Instruction::FORWARD(amount)),
            _ => panic!("invalid instruction"),
        }
    }
}

fn dive(input: &str) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;

    for instruction in input.lines() {
        let instruction: Instruction = instruction.parse().unwrap();

        match instruction {
            Instruction::UP(amount) => depth -= amount,
            Instruction::DOWN(amount) => depth += amount,
            Instruction::FORWARD(amount) => horizontal += amount,
        }
    }

    depth * horizontal
}

fn dive_aim(input: &str) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    let instructions: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    for i in instructions {
        let amount = i[1].parse::<usize>().unwrap();

        match i[0] {
            "up" => aim -= amount,
            "down" => aim += amount,
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            }
            _ => (),
        }
    }

    depth * horizontal
}

#[cfg(test)]
mod tests {
    use crate::dec2::{dive, dive_aim};

    const INPUT_SAMPLE: &str = include_str!("../inputs/2_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(dive(INPUT_SAMPLE), 150);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(dive_aim(INPUT_SAMPLE), 900);
    }
}
