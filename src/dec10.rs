use std::fmt;

const INPUT: &str = include_str!("../inputs/10.txt");

const OPENING: [char; 4] = ['(', '[', '{', '<'];
const CLOSING: [char; 4] = [')', ']', '}', '>'];

#[derive(Debug)]
struct ParseError {
    expected: char,
    actual: char,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Expected {}, but found {} instead",
            self.expected, self.actual
        )
    }
}

pub fn solve() {
    let solution_a = error_lines(INPUT);
    println!("dec10, solution A {}", solution_a);

    let solution_b = completion(INPUT);
    println!("dec10, solution B {}", solution_b);
}

fn error_lines(input: &str) -> u32 {
    let mut error_scores = vec![];
    for line in input.lines() {
        let result = validate(line);
        if let Err(e) = result {
            let score = match e.actual {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("unexpected token {}", e.actual),
            };
            error_scores.push(score);
        }
    }

    error_scores.iter().sum()
}

fn completion(input: &str) -> usize {
    let mut completion_scores = vec![];
    for line in input.lines() {
        let result = validate(line);
        if let Err(e) = result {
        } else {
            let to_complete = result.unwrap();
            let mut completion_score = 0;
            for char in to_complete.iter().rev() {
                let score = match char {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("unexpected token {}", char),
                };

                completion_score *= 5;
                completion_score += score;
            }

            completion_scores.push(completion_score);
        }
    }

    completion_scores.sort();

    completion_scores[completion_scores.len() / 2]
}

fn validate(line: &str) -> Result<Vec<char>, ParseError> {
    let mut stack = Vec::new();
    for char in line.chars() {
        if OPENING.contains(&char) {
            stack.push(char);
        } else {
            let opening = stack.pop().unwrap();
            let expected = CLOSING[get_pair_index(opening)];
            if expected != char {
                return Err(ParseError {
                    expected,
                    actual: char,
                });
            }
        }
    }

    Ok(stack
        .into_iter()
        .map(|c| CLOSING[get_pair_index(c)])
        .collect())
}

fn get_pair_index(c: char) -> usize {
    OPENING.iter().position(|&char| char == c).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::dec10::{completion, error_lines, validate};

    const INPUT_SAMPLE: &str = include_str!("../inputs/10_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(error_lines(INPUT_SAMPLE), 26397);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(completion(INPUT_SAMPLE), 288957);
    }
}
