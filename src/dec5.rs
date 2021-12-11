const INPUT: &str = include_str!("../inputs/5.txt");

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Line {
    from: (usize, usize),
    to: (usize, usize),
    points: Vec<(usize, usize)>,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        is_vertical(self.from, self.to)
    }

    pub fn is_horizontal(&self) -> bool {
        is_horizontal(self.from, self.to)
    }

    pub fn is_diagonal(&self) -> bool {
        if self.is_horizontal() || self.is_vertical() {
            return false;
        }

        is_diagonal(self.from, self.to)
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let from = parts.next().unwrap();
        let _ = parts.next();
        let to = parts.next().unwrap();

        let (from_x, from_y) = from.split_once(",").unwrap();
        let (to_x, to_y) = to.split_once(",").unwrap();

        let from = (
            from_x.parse::<usize>().unwrap(),
            from_y.parse::<usize>().unwrap(),
        );

        let to = (
            to_x.parse::<usize>().unwrap(),
            to_y.parse::<usize>().unwrap(),
        );

        let mut points = vec![];

        if is_horizontal(from, to) || is_vertical(from, to) || is_diagonal(from, to) {
            if is_horizontal(from, to) {
                let (x1, x2) = if from.0 < to.0 {
                    (from.0, to.0)
                } else {
                    (to.0, from.0)
                };

                for x in x1..=x2 {
                    points.push((x, from.1));
                }
            } else if is_vertical(from, to) {
                let (y1, y2) = if from.1 < to.1 {
                    (from.1, to.1)
                } else {
                    (to.1, from.1)
                };

                for y in y1..=y2 {
                    points.push((from.0, y));
                }
            } else if is_diagonal(from, to) {
                points.push(from);

                let x_ascending = from.0 < to.0;
                let y_ascending = from.1 < to.1;

                if x_ascending {
                    let mut y = from.1 as isize;
                    for x in (from.0 + 1)..to.0 {
                        let dy: isize = if y_ascending { 1 } else { -1 };
                        y += dy;
                        points.push((x, y as usize));
                    }
                } else if !x_ascending && !y_ascending {
                    let mut y = to.1 as isize;
                    let mut temp = vec![];
                    for x in (to.0 + 1)..from.0 {
                        y += 1;
                        temp.push((x, y as usize));
                    }
                    temp.reverse();
                    points.extend(temp.iter());
                } else {
                    let mut x = from.0 as isize;
                    for y in (from.1 + 1)..to.1 {
                        let dx: isize = if x_ascending { 1 } else { -1 };
                        x += dx;
                        points.push((x as usize, y));
                    }
                }

                points.push(to);
            }
        }

        Ok(Self { from, to, points })
    }
}

fn is_vertical(from: (usize, usize), to: (usize, usize)) -> bool {
    from.0 == to.0
}

fn is_horizontal(from: (usize, usize), to: (usize, usize)) -> bool {
    from.1 == to.1
}

fn is_diagonal(from: (usize, usize), to: (usize, usize)) -> bool {
    (from.0 as isize - to.0 as isize).abs() == (from.1 as isize - to.1 as isize).abs()
}

pub fn solve() {
    let solution_a = lines(INPUT);
    println!("dec5, solution A {}", solution_a);

    let solution_b = lines_diagonal(INPUT);
    println!("dec5, solution B {}", solution_b);
}

fn lines(input: &str) -> usize {
    let lines: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut points: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines.iter() {
        if line.is_horizontal() || line.is_vertical() {
            for point in line.points.iter() {
                let count = if let Some(count) = points.get(point) {
                    *count
                } else {
                    0
                };

                points.insert(*point, count + 1);
            }
        }
    }

    points.values().filter(|count| **count > 1 as usize).count()
}

fn lines_diagonal(input: &str) -> usize {
    let lines: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut points: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines.iter() {
        if line.is_horizontal() || line.is_vertical() || line.is_diagonal() {
            for point in line.points.iter() {
                let count = if let Some(count) = points.get(point) {
                    *count
                } else {
                    0
                };

                points.insert(*point, count + 1);
            }
        }
    }

    points.values().filter(|count| **count > 1 as usize).count()
}

#[cfg(test)]
mod tests {
    use crate::dec5::{lines, lines_diagonal};

    const INPUT_SAMPLE: &str = include_str!("../inputs/5_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(lines(INPUT_SAMPLE), 5);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(lines_diagonal(INPUT_SAMPLE), 12);
    }
}
