use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("../inputs/8.txt");

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Digit {
    segments: u8,
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = 0b000_0000;

        for segment in s.chars() {
            match segment {
                'a' => segments |= 0b100_0000,
                'b' => segments |= 0b010_0000,
                'c' => segments |= 0b001_0000,
                'd' => segments |= 0b000_1000,
                'e' => segments |= 0b000_0100,
                'f' => segments |= 0b000_0010,
                'g' => segments |= 0b000_0001,
                _ => panic!("unknown segment id {}", segment),
            }
        }

        Ok(Self { segments })
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:07b}", self.segments)
    }
}

pub fn solve() {
    let solution_a = unique_digits(INPUT);
    println!("dec8, solution A {}", solution_a);

    let solution_b = sum_outputs(INPUT);
    println!("dec8, solution B {}", solution_b);
}

fn unique_digits(input: &str) -> usize {
    let entries: Vec<(&str, &str)> = input
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .collect();

    entries
        .iter()
        .map(|(_signals, output)| output.split(" ").filter(|v| check_unique(v)).count())
        .sum()
}

fn sum_outputs(input: &str) -> usize {
    let entries: Vec<(&str, &str)> = input
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .collect();

    let mut sum = 0;
    for (signal_seq, output_seq) in entries.iter() {
        let signals: Vec<&str> = signal_seq.split(" ").collect();
        let signal_to_num = decode_wiring(signals);

        let decoded: Vec<&str> = output_seq
            .split(" ")
            .map(|s| *signal_to_num.get(&s.parse::<Digit>().unwrap()).unwrap())
            .collect();
        sum += decoded.join("").parse::<usize>().unwrap();
    }

    sum
}

fn check_unique(combination: &str) -> bool {
    match combination.len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

fn decode_wiring(mut signals: Vec<&str>) -> HashMap<Digit, &str> {
    let mut signal_to_num: HashMap<Digit, &str> = HashMap::new();
    let mut num_to_signal: HashMap<usize, Digit> = HashMap::new();

    signals.sort_by(|a, b| a.len().cmp(&b.len()));

    let one: Digit = signals[0].parse().unwrap();
    num_to_signal.insert(1, one);
    signal_to_num.insert(one, "1");

    let mut c: Option<u8> = None;
    let mut d: Option<u8> = None;

    while signal_to_num.len() < 10 {
        for signal in signals.iter() {
            let digit: Digit = signal.parse().unwrap();
            let number = match signal.len() {
                3 => Some("7"),
                4 => Some("4"),
                5 => two_three_five(digit, one, c),
                6 => six_nine_zero(digit, one, num_to_signal.get(&6), d),
                7 => Some("8"),
                _ => None,
            };

            if let Some(num) = number {
                if !signal_to_num.contains_key(&digit) {
                    signal_to_num.insert(digit, num);
                    num_to_signal.insert(num.parse::<usize>().unwrap(), digit);
                }

                if num == "3" && d.is_none() {
                    if let Some(four) = num_to_signal.get(&4) {
                        d = Some((one.segments ^ four.segments) & digit.segments);
                    }
                }

                if num == "6" && c.is_none() {
                    c = Some(one.segments & digit.segments);
                }
            }
        }
    }

    signal_to_num
}

fn two_three_five<'a>(digit: Digit, one: Digit, c: Option<u8>) -> Option<&'a str> {
    if let Some(c) = c {
        return if digit.segments & c == 0 {
            Some("2")
        } else {
            Some("5")
        };
    }

    if one.segments & digit.segments == one.segments {
        return Some("3");
    }

    None
}

fn six_nine_zero<'a>(
    digit: Digit,
    one: Digit,
    six: Option<&Digit>,
    d: Option<u8>,
) -> Option<&'a str> {
    if six.is_some() {
        if let Some(d) = d {
            return if digit.segments ^ d == 0b111_1111 {
                Some("0")
            } else {
                Some("9")
            };
        }
    }

    if !(one.segments & digit.segments == one.segments) {
        return Some("6");
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::dec8::{sum_outputs, unique_digits};

    const INPUT_SAMPLE: &str = include_str!("../inputs/8_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(unique_digits(INPUT_SAMPLE), 26);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(sum_outputs(INPUT_SAMPLE), 61229);
    }
}
