use std::{collections::VecDeque, time::Instant};

mod explode;
mod magnitude;
mod pair;
mod parse;
mod split;
mod adding;

use adding::add;
use magnitude::magnitude;
use parse::parse_pair;

fn main() {
    let start = Instant::now();
    println!(
        "Answer one: {} ({:?})",
        part1("input.txt"),
        Instant::now() - start
    );

    let start = Instant::now();
    println!(
        "Answer two: {} ({:?})",
        part2("input.txt"),
        Instant::now() - start
    );
}

fn part1(file_name: &str) -> usize {
    let mut pairs = helpers::read_lines_panicky(file_name)
        .map(|l| parse_pair(&l))
        .collect::<VecDeque<_>>();

    let mut current = pairs.pop_front().unwrap();
    while let Some(pair) = pairs.pop_front() {
        current = add(current, pair);
    }

    magnitude(&current)
}

fn part2(file_name: &str) -> usize {
    let pairs = helpers::read_lines_panicky(file_name)
        .map(|l| parse_pair(&l))
        .collect::<Vec<_>>();

    let mut max = 0;
    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if j != i {
                let first = &pairs[i];
                let second = &pairs[j];
                let added = add(first.clone(), second.clone());
                max = std::cmp::max(max, magnitude(&added));
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(4140, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(4235, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3993, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(4659, part2("input.txt"));
    }
}
