use crate::pair::*;
use crate::explode::explode;
use crate::split::split;

pub fn add(left: Pair, right: Pair) -> Pair {
    reduce(Pair(
        Element::Pair(Box::new(left)),
        Element::Pair(Box::new(right)),
    ))
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
    use crate::parse::parse_pair;

    #[test]
    fn test_reduce() {
        let pair = parse_pair("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(expected, &format!("{}", reduce(pair)));
    }
}