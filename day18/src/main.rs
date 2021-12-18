use std::fmt::Display;

mod pair;
mod parse;
use pair::{Pair, Element};
use parse::parse_pair;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
}

fn part1(file_name: &str) -> usize {
    todo!()
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

fn explode(pair: &mut Pair) -> bool {
    todo!()
}

fn split(pair: &mut Pair) -> bool {
    todo!()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(4140, part1("test_input.txt"));
    }

    #[test]
    fn test_reduce() {
        let pair = parse_pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(expected, &format!("{}", reduce(pair)));
    }
}