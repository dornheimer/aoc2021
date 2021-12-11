const INPUT: &str = include_str!("../inputs/4.txt");

struct Board {
    numbers: Vec<Option<usize>>,
    bingo: bool,
}

impl Board {
    const SIZE: usize = 5;

    fn new(s: &str) -> Self {
        let numbers: Vec<Option<usize>> = s
            .split_whitespace()
            .map(|num| Some(num.trim().parse().unwrap()))
            .collect();

        Self {
            numbers,
            bingo: false,
        }
    }

    pub fn check(&mut self, draw: usize) -> bool {
        for i in 0..self.numbers.len() {
            if let Some(n) = self.numbers[i] {
                if n == draw {
                    self.numbers[i] = None;
                }
            }
        }

        for y in 0..Self::SIZE {
            if self.get_row(y).iter().all(|num| num.is_none()) {
                self.bingo = true;
                return true;
            }
        }

        for x in 0..Self::SIZE {
            if self.get_column(x).iter().all(|num| num.is_none()) {
                self.bingo = true;
                return true;
            }
        }

        false
    }

    pub fn get_unmarked(&self) -> Vec<usize> {
        self.numbers
            .clone()
            .into_iter()
            .filter(|num| num.is_some())
            .map(|num| num.unwrap())
            .collect()
    }

    fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        self.numbers[y * Self::SIZE + x]
    }

    fn get_row(&self, y: usize) -> Vec<Option<usize>> {
        (0..Self::SIZE).map(|x| self.get_index(x, y)).collect()
    }

    fn get_column(&self, x: usize) -> Vec<Option<usize>> {
        (0..Self::SIZE).map(|y| self.get_index(x, y)).collect()
    }
}

pub fn solve() {
    let solution_a = bingo(INPUT);
    println!("dec4, solution A {}", solution_a);

    let solution_b = squid(INPUT);
    println!("dec4, solution B {}", solution_b);
}

fn bingo(input: &str) -> usize {
    let (draws, mut boards) = setup(input);

    for draw in draws.iter() {
        for board in boards.iter_mut() {
            if board.check(*draw) {
                return board.get_unmarked().iter().sum::<usize>() * draw;
            }
        }
    }

    panic!("no solution found");
}

fn squid(input: &str) -> usize {
    let (draws, mut boards) = setup(input);

    for draw in draws.iter() {
        let remaining_count = boards.iter().filter(|b| !b.bingo).count();
        for board in boards.iter_mut() {
            if !board.bingo && board.check(*draw) && remaining_count == 1 {
                return board.get_unmarked().iter().sum::<usize>() * draw;
            }
        }
    }

    panic!("no solution found");
}

fn setup(s: &str) -> (Vec<usize>, Vec<Board>) {
    let mut input_iterator = s.split("\n\n");
    let draws: Vec<usize> = input_iterator
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let boards: Vec<Board> = input_iterator.into_iter().map(|s| Board::new(s)).collect();

    (draws, boards)
}

#[cfg(test)]
mod tests {
    use crate::dec4::{bingo, squid};

    const INPUT_SAMPLE: &str = include_str!("../inputs/4_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(bingo(INPUT_SAMPLE), 4512);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(squid(INPUT_SAMPLE), 1924);
    }
}
