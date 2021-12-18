use std::collections::VecDeque;

mod explode;
mod magnitude;
mod pair;
mod parse;
mod split;

use explode::explode;
use magnitude::magnitude;
use pair::{Element, Pair};
use parse::parse_pair;
use split::split;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> usize {
    let mut pairs = helpers::read_lines_panicky(file_name)
        .map(|l| parse_pair(&l))
        .collect::<VecDeque<_>>();

    let mut current = pairs.pop_front().unwrap();
    while let Some(pair) = pairs.pop_front() {
        current = add(current, pair);
        current = reduce(current);
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
                let added = reduce(added);
                max = std::cmp::max(max, magnitude(&added));
            }
        }
    }

    max
}

fn add(left: Pair, right: Pair) -> Pair {
    Pair(
        Element::Pair(Box::new(left)),
        Element::Pair(Box::new(right)),
    )
}

fn reduce(mut pair: Pair) -> Pair {
    loop {
        if explode(&mut pair) {
            continue;
        }

        if split(&mut pair) {
            continue;
        }

        // didn't do either, we're done
        break;
    }
    pair
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

    #[test]
    fn test_reduce() {
        let pair = parse_pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(expected, &format!("{}", reduce(pair)));
    }
}
