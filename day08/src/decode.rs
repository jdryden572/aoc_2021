use std::collections::{HashMap, HashSet};

/*
 AAAA
B    C
B    C
 DDDD
E    F
E    F
 GGGG
*/

static DIGITS: &[&str] = &[
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

pub struct Decoder {
    mappings: HashMap<char, char>,
}

impl Decoder {
    pub fn build(inputs: Vec<String>) -> Self {
        let mut mappings = HashMap::new();
        let mut inputs: Vec<Vec<char>> = inputs.into_iter().map(|i| i.chars().collect()).collect();
        inputs.sort_by(|x, y| x.len().cmp(&y.len()));

        let one = set(&inputs[0]);
        let seven = set(&inputs[1]);
        let four = set(&inputs[2]);
        let eight = set(&inputs[9]);

        let two_three_five = vec![set(&inputs[3]), set(&inputs[4]), set(&inputs[5])];
        let zero_six_nine = vec![set(&inputs[6]), set(&inputs[7]), set(&inputs[8])];

        // Top segment is in 7, but not in 1
        let &seg_a = seven.difference(&one).next().unwrap();
        mappings.insert(seg_a, 'a');

        // Bottom segment is the only one (other than top) that is in all of 0+2+3+5+6+9
        let combined = || two_three_five.iter().chain(zero_six_nine.iter());
        let &seg_g = eight
            .iter()
            .filter(|&c| c != &seg_a && combined().filter(|d| d.contains(c)).count() == 6)
            .next()
            .unwrap();
        mappings.insert(seg_g, 'g');

        // Segment F is the segment from 1 that is also in all three of 0+6+9
        let in_zero_six = set(zero_six_nine[0].intersection(&zero_six_nine[1]));
        let in_all_three = set(in_zero_six.intersection(&zero_six_nine[2]));
        let &seg_f = one.intersection(&in_all_three).next().unwrap();
        mappings.insert(seg_f, 'f');

        // Segment C is the other segment from 1
        let &seg_c = one.difference(&set(&[seg_f])).next().unwrap();
        mappings.insert(seg_c, 'c');

        // Segment D is the only one (other than C) that is from 4 but only two of 0+6+9
        let &seg_d = four
            .difference(&set(&[seg_c]))
            .filter(|&c| zero_six_nine.iter().filter(|d| d.contains(c)).count() == 2)
            .next()
            .unwrap();
        mappings.insert(seg_d, 'd');

        // Segment B is the remaining unknown from 4
        let &seg_b = four
            .difference(&set(&[seg_c, seg_d, seg_f]))
            .next()
            .unwrap();
        mappings.insert(seg_b, 'b');

        // Segment E is the last one, just find the one we don't have as a key yet
        let &seg_e = eight.difference(&set(mappings.keys())).next().unwrap();
        mappings.insert(seg_e, 'e');

        Self { mappings }
    }

    pub fn decode(&self, input: &str) -> usize {
        let mut mapped = input
            .chars()
            .map(|c| self.mappings.get(&c).copied().unwrap())
            .collect::<Vec<char>>();
        mapped.sort();
        let mapped = mapped.into_iter().collect::<String>();

        DIGITS.iter().position(|&digit| digit == mapped).unwrap()
    }
}

fn set<'a, I: IntoIterator<Item = &'a char>>(slice: I) -> HashSet<char> {
    slice.into_iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let inputs: Vec<String> = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let mut mappings = HashMap::new();
        mappings.insert('a', 'c');
        mappings.insert('b', 'f');
        mappings.insert('c', 'g');
        mappings.insert('d', 'a');
        mappings.insert('e', 'b');
        mappings.insert('f', 'd');
        mappings.insert('g', 'e');
        assert_eq!(mappings, Decoder::build(inputs).mappings);
    }

    #[test]
    fn test_decode() {
        let mut mappings = HashMap::new();
        mappings.insert('a', 'c');
        mappings.insert('b', 'f');
        mappings.insert('c', 'g');
        mappings.insert('d', 'a');
        mappings.insert('e', 'b');
        mappings.insert('f', 'd');
        mappings.insert('g', 'e');

        let decoder = Decoder { mappings };
        assert_eq!(5, decoder.decode("cdfeb"));
        assert_eq!(3, decoder.decode("fcadb"));
        assert_eq!(3, decoder.decode("cdbaf"));
    }
}
