use std::collections::VecDeque;

mod pair;
mod parse;
mod magnitude;
mod explode;
mod split;

use pair::{Pair, Element};
use parse::parse_pair;
use magnitude::magnitude;
use explode::explode;
use split::split;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
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
    fn test_reduce() {
        let pair = parse_pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(expected, &format!("{}", reduce(pair)));
    }
}