use crate::pair::*;

pub fn parse_pair(input: &str) -> Pair {
    let input = trim_outer_braces(input);
    let (left, right) = split_at_comma(input);
    let a = parse_element(left);
    let b = parse_element(right);
    Pair(a, b)
}

fn parse_element(input: &str) -> Element {
    if input.contains("[") {
        let pair = parse_pair(input);
        Element::Pair(Box::new(pair))
    } else {
        let num = input.parse().unwrap();
        Element::Number(num)
    }
}

fn trim_outer_braces(input: &str) -> &str {
    assert_eq!("[", &input[0..1]);
    assert_eq!("]", &input[input.len()-1..]);
    &input[1..input.len()-1]
}

fn split_at_comma(input: &str) -> (&str, &str) {
    let mut depth = 0;
    for (i, c) in input.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {
                if depth == 0 {
                    let (a, b) = input.split_at(i);
                    return (a, &b[1..])
                }
            },
            _ => {},
        }
    }

    panic!("Never found comma!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pairs() {
        let inputs = vec![
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        ];
        for input in inputs {
            assert_eq!(input, &format!("{}", parse_pair(input)));
        }
    }
}