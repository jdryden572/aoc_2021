use crate::pair::*;

pub fn split(pair: &mut Pair) -> bool {
    split_element(&mut pair.0) || split_element(&mut pair.1)
}

fn split_element(element: &mut Element) -> bool {
    match element {
        Element::Number(num) => {
            if *num > 9 {
                let split = Pair(
                    Element::Number(*num / 2),
                    Element::Number(*num / 2 + *num % 2),
                );
                *element = Element::Pair(Box::new(split));
                true
            } else {
                false
            }
        }
        Element::Pair(pair) => split_element(&mut pair.0) || split_element(&mut pair.1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_pair;

    #[test]
    fn test_split() {
        let cases = vec![
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];
        for (input, expected) in cases {
            let mut pair = parse_pair(input);
            assert!(split(&mut pair));
            assert_eq!(expected, format!("{}", pair));
        }
    }

    #[test]
    fn test_not_split() {
        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let mut pair = parse_pair(input);
        assert!(!split(&mut pair));
        assert_eq!(input, format!("{}", pair));
    }
}
