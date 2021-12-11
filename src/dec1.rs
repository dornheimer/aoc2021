use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT: &str = include_str!("../inputs/1.txt");

pub fn solve() {
    let solution_a = sonar_scan(INPUT);
    println!("dec1, solution A {}", solution_a);

    let solution_b = sonar_scan_sliding(INPUT);
    println!("dec1, solution B {}", solution_b);
}

pub fn solution_a() -> usize {
    let mut increases = 0;
    let mut previous = None;
    if let Ok(lines) = read_lines("./inputs/1.txt") {
        for line in lines {
            if let Ok(value) = line {
                let depth = value.parse::<u32>().unwrap();
                if let Some(prev) = previous {
                    if depth > prev {
                        increases += 1;
                    }
                }

                previous = Some(depth);
            }
        }
    }

    increases
}

pub fn solution_b() -> usize {
    let mut increases = 0;
    let mut window_a = Vec::<u32>::with_capacity(3);
    let mut window_b = Vec::<u32>::with_capacity(3);
    let mut temp = Vec::<u32>::with_capacity(3);
    if let Ok(lines) = read_lines("./inputs/1.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(value) = line {
                let depth = value.parse::<u32>().unwrap();
                window_a.insert(0, depth);

                if i > 0 {
                    window_b.insert(0, depth);
                }

                if window_a.len() == 3 {
                    if temp.len() == 3 {
                        if window_a.iter().sum::<u32>() > temp.iter().sum::<u32>() {
                            increases += 1;
                        }
                    }

                    temp = window_a.clone();
                    window_a = vec![depth];
                }

                if window_b.len() == 3 {
                    if temp.len() == 3 {
                        if window_b.iter().sum::<u32>() > temp.iter().sum::<u32>() {
                            increases += 1;
                        }
                    }

                    temp = window_b.clone();
                    window_b = vec![depth];
                }
            }
        }
    }

    increases
}

fn sonar_scan(input: &str) -> usize {
    let measurements: Vec<u16> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    measurements
        .array_windows()
        .filter(|[m1, m2]| m2 > m1)
        .count()
}

fn sonar_scan_sliding(input: &str) -> usize {
    let measurements: Vec<u16> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    let sums: Vec<u16> = measurements
        .array_windows()
        .map(|[m1, m2, m3]| m1 + m2 + m3)
        .collect();

    sums.array_windows().filter(|[m1, m2]| m2 > m1).count()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

#[cfg(test)]
mod tests {
    use crate::dec1::{sonar_scan, sonar_scan_sliding};

    const INPUT_SAMPLE: &str = include_str!("../inputs/1_sample.txt");

    #[test]
    fn sample_a() {
        assert_eq!(sonar_scan(INPUT_SAMPLE), 7);
    }

    #[test]
    fn sample_b() {
        assert_eq!(sonar_scan_sliding(INPUT_SAMPLE), 5);
    }
}
