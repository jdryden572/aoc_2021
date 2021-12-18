use crate::pair::*;

pub fn magnitude(pair: &Pair) -> usize {
    let left = element_magnitude(&pair.0);
    let right = element_magnitude(&pair.1);
    3 * left + 2 * right
}

fn element_magnitude(element: &Element) -> usize {
    match element {
        Element::Number(num) => *num,
        Element::Pair(pair) => magnitude(pair),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_pair;

    #[test]
    fn test_magnitude() {
        assert_eq!(143, magnitude(&parse_pair("[[1,2],[[3,4],5]]")));
        assert_eq!(1384, magnitude(&parse_pair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")));
        assert_eq!(445, magnitude(&parse_pair("[[[[1,1],[2,2]],[3,3]],[4,4]]")));
        assert_eq!(791, magnitude(&parse_pair("[[[[3,0],[5,3]],[4,4]],[5,5]]")));
        assert_eq!(1137, magnitude(&parse_pair("[[[[5,0],[7,4]],[5,5]],[6,6]]")));
        assert_eq!(3488, magnitude(&parse_pair("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")));
    }
}