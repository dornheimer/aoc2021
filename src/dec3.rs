const INPUT: &str = include_str!("../inputs/3.txt");

pub fn solve() {
    let solution_a = power(INPUT);
    println!("dec3, solution A {}", solution_a);

    let solution_b = life_support(INPUT);
    println!("dec3, solution B {}", solution_b);
}

fn power(input: &str) -> usize {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    let num_digits = input.lines().nth(1).unwrap().len();
    for pos in 0..num_digits {
        let (most_common, least_common) =
            if count_at_position(input, pos + 1) > input.lines().count() / 2 {
                ("1", "0")
            } else {
                ("0", "1")
            };

        gamma.push_str(most_common);
        epsilon.push_str(least_common);
    }

    binary_to_decimal(&gamma) * binary_to_decimal(&epsilon)
}

fn life_support(input: &str) -> usize {
    let num_digits = input.lines().nth(0).unwrap().len();
    let mut numbers_oxygen: Vec<&str> = input.lines().collect();
    let mut numbers_co2: Vec<&str> = input.lines().collect();
    for pos in 0..num_digits {
        let most_common = if most_common_at_position(&numbers_oxygen, pos + 1) {
            "1"
        } else {
            "0"
        };

        if numbers_oxygen.len() > 1 {
            numbers_oxygen = numbers_oxygen
                .into_iter()
                .filter(|num| is_char_at_position(num, most_common, pos + 1))
                .collect();
        }

        let least_common = if !most_common_at_position(&numbers_co2, pos + 1) {
            "1"
        } else {
            "0"
        };

        if numbers_co2.len() > 1 {
            numbers_co2 = numbers_co2
                .into_iter()
                .filter(|num| is_char_at_position(num, least_common, pos + 1))
                .collect();
        }
    }

    let oxygen = numbers_oxygen[0];
    let co2 = numbers_co2[0];

    binary_to_decimal(oxygen) * binary_to_decimal(co2)
}

fn count_at_position(input: &str, position: usize) -> usize {
    input
        .lines()
        .filter(|num| is_char_at_position(num, "1", position))
        .count()
}

fn count_at_position_vec(input: &Vec<&str>, position: usize) -> usize {
    input
        .iter()
        .filter(|num| is_char_at_position(num, "1", position))
        .count()
}

fn most_common_at_position(input: &Vec<&str>, position: usize) -> bool {
    count_at_position_vec(input, position) as f32 >= input.len() as f32 / 2.0
}

fn is_char_at_position(string: &str, char: &str, position: usize) -> bool {
    string.split("").nth(position).unwrap() == char
}

fn binary_to_decimal(string: &str) -> usize {
    usize::from_str_radix(string, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::dec3::{life_support, power};

    const INPUT_SAMPLE: &str = include_str!("../inputs/3_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(power(INPUT_SAMPLE), 198);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(life_support(INPUT_SAMPLE), 230);
    }
}
