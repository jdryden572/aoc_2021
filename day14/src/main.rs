use std::{collections::HashMap, time::Instant};

type Pair = (char, char);

fn main() {
    let lines = helpers::read_lines_panicky("input.txt").collect();

    let start = Instant::now();
    println!("Answer one: {} ({:?})", both_parts(&lines, 10), Instant::now() - start);

    let start = Instant::now();
    println!("Answer two: {} ({:?})", both_parts(&lines, 40), Instant::now() - start);
}

fn both_parts(lines: &Vec<String>, step_count: usize) -> usize {
    let pair_mappings = parse_pair_mappings(lines);

    let mut pair_counts = parse_initial_pairs(lines);
    let mut temp_pair_counts = HashMap::new();

    for _ in 0..step_count {
        // for each pair, create two new pairs with the same count
        for (pair, count) in pair_counts.iter() {
            for new_pair in get_new_pairs(pair, &pair_mappings) {
                *temp_pair_counts.entry(new_pair).or_insert(0) += count;
            }
        }

        // clear the old pairings, the temp collection is now our main pairing counts
        pair_counts.clear();
        std::mem::swap(&mut pair_counts, &mut temp_pair_counts);
    }

    // Count the occurrances of each character by counting the first char in each pair.
    let mut char_counts = HashMap::new();
    for ((c, _), count) in pair_counts {
        *char_counts.entry(c).or_insert(0) += count;
    }

    // Since we counted chars using the first char in each pair, we are missing
    // one occurrance of the last char in the original string. Add it manually.
    let last_char = parse_last_char(lines);
    *char_counts.get_mut(&last_char).unwrap() += 1;

    let max = char_counts.values().max().unwrap();
    let min = char_counts.values().min().unwrap();
    max - min
}

fn get_new_pairs(pair: &Pair, pair_mappings: &HashMap<Pair, char>) -> [Pair; 2] {
    let &splitter = pair_mappings.get(pair).unwrap();
    [
        (pair.0, splitter),
        (splitter, pair.1)
    ]
}

fn parse_initial_pairs(lines: &Vec<String>) -> HashMap<Pair, usize> {
    let line = lines[0].chars().collect::<Vec<_>>();
    let mut pair_counts = HashMap::new();
    for pair in line.windows(2) {
        let pair = (pair[0], pair[1]);
        *pair_counts.entry(pair).or_insert(0) += 1;
    }
    pair_counts
}

fn parse_pair_mappings(lines: &Vec<String>) -> HashMap<Pair, char> {
    let mut mappings = HashMap::new();
    for line in lines.iter().skip(2) {
        let mut chars = line.chars();
        let pair = (chars.next().unwrap(), chars.next().unwrap());
        let value = chars.skip(4).next().unwrap();
        mappings.insert(pair, value);
    }
    mappings
}

fn parse_last_char(lines: &Vec<String>) -> char {
    lines[0].chars().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_lines_panicky("test_input.txt").collect();
        assert_eq!(1588, both_parts(&lines, 10));
    }

    #[test]
    fn final_part1() {
        let lines = helpers::read_lines_panicky("input.txt").collect();
        assert_eq!(3831, both_parts(&lines, 10));
    }

    #[test]
    fn test_part2() {
        let lines = helpers::read_lines_panicky("test_input.txt").collect();
        assert_eq!(2188189693529, both_parts(&lines, 40));
    }

    #[test]
    fn final_part2() {
        let lines = helpers::read_lines_panicky("input.txt").collect();
        assert_eq!(5725739914282, both_parts(&lines, 40));
    }
}