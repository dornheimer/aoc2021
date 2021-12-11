use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/6.txt");
const SPAWN_TIMER_NEW: usize = 8;
const SPAWN_TIMER_RESET: usize = 6;

pub fn solve() {
    let solution_a = growth_cache(INPUT, 80);
    println!("dec6, solution A {}", solution_a);

    let solution_b = growth_cache(INPUT, 256);
    println!("dec6, solution B {}", solution_b);
}

// slow, non optimized solution
fn growth(input: &str, days: usize) -> usize {
    let mut fish: Vec<usize> = input.split(",").map(|f| f.parse().unwrap()).collect();
    for _day in 0..days {
        let new_count = fish.iter().filter(|f| **f == 0).count();
        fish = fish
            .iter()
            .map(|f| if *f == 0 { SPAWN_TIMER_RESET } else { f - 1 })
            .collect();
        for _new in 0..new_count {
            fish.push(SPAWN_TIMER_NEW);
        }
    }

    fish.len()
}

// recursive solution with cache
fn growth_cache(input: &str, days: usize) -> usize {
    let fish: Vec<usize> = input.split(",").map(|f| f.parse().unwrap()).collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    return fish.len()
        + fish
            .iter()
            .map(|f| spawn(days, *f, 0, &mut cache))
            .sum::<usize>();
}

fn spawn(
    remaining: usize,
    state: usize,
    counter: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if state > remaining {
        return counter;
    }

    if remaining > 0 {
        if state == 0 {
            let new = if !cache.contains_key(&(SPAWN_TIMER_NEW, remaining - 1)) {
                let new = spawn(remaining - 1, SPAWN_TIMER_NEW, 0, cache);
                cache.insert((SPAWN_TIMER_NEW, remaining - 1), new);
                new
            } else {
                *cache.get(&(SPAWN_TIMER_NEW, remaining - 1)).unwrap()
            };

            let reset = if !cache.contains_key(&(SPAWN_TIMER_RESET, remaining - 1)) {
                let reset = spawn(remaining - 1, SPAWN_TIMER_RESET, 1, cache);
                cache.insert((SPAWN_TIMER_RESET, remaining - 1), reset);
                reset
            } else {
                *cache.get(&(SPAWN_TIMER_RESET, remaining - 1)).unwrap()
            };

            return new + reset + counter;
        }

        return spawn(remaining - 1, state - 1, counter, cache);
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::dec6::{growth, growth_cache, spawn};
    use std::collections::HashMap;

    const INPUT_SAMPLE: &str = include_str!("../inputs/6_sample.txt");

    #[test]
    fn part1_sample() {
        assert_eq!(growth(INPUT_SAMPLE, 80), 5934);
    }

    #[test]
    fn part1_sample_cache() {
        assert_eq!(growth_cache(INPUT_SAMPLE, 18), 26);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(growth_cache(INPUT_SAMPLE, 256), 26984457539);
    }

    #[test]
    fn spawn_test() {
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

        assert_eq!(spawn(18, 3, 0, &mut cache), 4);
    }
}
