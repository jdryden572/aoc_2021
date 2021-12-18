use crate::pair::*;
use Status::*;

pub fn explode(pair: &mut Pair) -> bool {
    let status = explode_pair_recurs(pair, 0);

    matches!(status, Exploded(..))
}

enum Status {
    Exploded(Option<usize>, Option<usize>),
    NotExploded,
}

fn explode_pair_recurs(pair: &mut Pair, depth: usize) -> Status {
    match explode_element_recurs(&mut pair.0, depth) {
        Exploded(left_0, left_1) => {
            if let Some(add) = left_1 {
                add_number_to_left(&mut pair.1, add);
            }

            return Exploded(left_0, None);
        }
        NotExploded => {}
    }

    match explode_element_recurs(&mut pair.1, depth) {
        Exploded(right_0, right_1) => {
            if let Some(add) = right_0 {
                add_number_to_right(&mut pair.0, add);
            }

            Exploded(None, right_1)
        }
        NotExploded => NotExploded,
    }
}

fn explode_element_recurs(element: &mut Element, depth: usize) -> Status {
    match element {
        Element::Number(_) => NotExploded,
        Element::Pair(pair) => {
            if depth == 3 {
                let (left, right) = (pair.0.number(), pair.1.number());
                *element = Element::Number(0);
                Exploded(left, right)
            } else {
                explode_pair_recurs(pair, depth + 1)
            }
        }
    }
}

fn add_number_to_left(element: &mut Element, add: usize) -> bool {
    match element {
        Element::Number(num) => {
            *num += add;
            return true;
        }
        Element::Pair(ref mut pair) => {
            if add_number_to_left(&mut pair.0, add) {
                return true;
            } else {
                add_number_to_left(&mut pair.1, add);
                return true;
            }
        }
    }
}

fn add_number_to_right(element: &mut Element, add: usize) -> bool {
    match element {
        Element::Number(num) => {
            *num += add;
            return true;
        }
        Element::Pair(ref mut pair) => {
            if add_number_to_right(&mut pair.1, add) {
                return true;
            } else {
                add_number_to_right(&mut pair.0, add);
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_pair;

    #[test]
    fn test_explode() {
        let cases = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];
        for (input, expected) in cases {
            let mut pair = parse_pair(input);
            assert!(explode(&mut pair));
            assert_eq!(expected, &format!("{}", pair));
        }
    }

    #[test]
    fn test_not_exploded() {
        let cases = vec!["[1,2]", "[[1,2],3]", "[9,[8,7]]", "[[1,9],[8,5]]"];
        for input in cases {
            let mut pair = parse_pair(input);
            assert!(!explode(&mut pair));
            assert_eq!(input, &format!("{}", pair));
        }
    }
}
