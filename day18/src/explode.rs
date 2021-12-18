use crate::pair::*;

pub fn explode(pair: &mut Pair) -> bool {
    explode_pair_recurs(pair, 0);

    true
}

fn explode_pair_recurs(pair: &mut Pair, depth: usize) -> (Option<usize>, Option<usize>) {
    let (left_0, left_1) = explode_element_recurs(&mut pair.0, depth);
    let mut escape = false;
    if let Some(add) = left_1 {
        escape = true;
        add_number_to_left(&mut pair.1, add);
    }

    if left_0.is_some() {
        return (left_0, None);
    }

    if escape {
        return (None, None);
    }

    let (right_0, right_1) = explode_element_recurs(&mut pair.1, depth);
    if let Some(add) = right_0 {
        add_number_to_right(&mut pair.0, add);
    }

    if right_1.is_some() {
        return (None, right_1);
    }

    (None, None)
}

fn explode_element_recurs(element: &mut Element, depth: usize) -> (Option<usize>, Option<usize>) {
    match element {
        Element::Number(_) => (None, None),
        Element::Pair(pair) => if depth == 3 {
            let result = (pair.0.number(), pair.1.number());
            *element = Element::Number(0);
            result
        } else {
            explode_pair_recurs(pair, depth + 1)
        },
    }
}

fn add_number_to_left(element: &mut Element, add: usize) -> bool {
    match element {
        Element::Number(num) => {
            *num  += add;
            return true;
        },
        Element::Pair(ref mut pair) => {
            if add_number_to_left(&mut pair.0, add) {
                return true;
            } else {
                add_number_to_left(&mut pair.1, add);
                return true;
            }
        },
    }
}

fn add_number_to_right(element: &mut Element, add: usize) -> bool {
    match element {
        Element::Number(num) => {
            *num  += add;
            return true;
        },
        Element::Pair(ref mut pair) => {
            if add_number_to_right(&mut pair.1, add) {
                return true;
            } else {
                add_number_to_right(&mut pair.0, add);
                return true;
            }
        },
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
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
        ];
        for (input, expected) in cases {
            let mut pair = parse_pair(input);
            assert!(explode(&mut pair));
            assert_eq!(expected, &format!("{}", pair));
        }
    }
}