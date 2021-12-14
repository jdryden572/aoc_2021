use std::collections::HashMap;

type Pair = (char, char);

fn main() {
    println!("Answer one: {}", both_parts("input.txt", 10));
    println!("Answer two: {}", both_parts("input.txt", 40));
}

fn both_parts(file_name: &str, step_count: usize) -> usize {
    let pair_mappings = parse_pair_mappings(file_name);

    let mut pair_counts = parse_initial_pairs(file_name);
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

    // Count the occurrances of each character (since each pair has two chars)
    let mut char_counts = HashMap::new();
    for (c, count) in pair_counts.into_iter().flat_map(|((a, b), count)| [(a, count), (b, count)]) {
        *char_counts.entry(c).or_insert(0) += count;
    }

    // Not counting the first and last chars, each char is a part of two pairs.
    // Currently each char is counted once for each pair, so we are off by a 
    // factor of two.
    for count in char_counts.values_mut() {
        *count = *count / 2;
    }

    // This is the weird part. The first and last chars are only part of one
    // pair, so we over-corrected for those chars. Add one extra count for them.
    for c in parse_first_last_chars(file_name) {
        *char_counts.get_mut(&c).unwrap() += 1;
    }

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

fn parse_initial_pairs(file_name: &str) -> HashMap<Pair, usize> {
    let line = helpers::read_lines_panicky(file_name).next().unwrap().chars().collect::<Vec<_>>();
    let mut pair_counts = HashMap::new();
    for pair in line.windows(2) {
        let pair = (pair[0], pair[1]);
        *pair_counts.entry(pair).or_insert(0) += 1;
    }
    pair_counts
}

fn parse_pair_mappings(file_name: &str) -> HashMap<Pair, char> {
    let mut mappings = HashMap::new();
    for line in helpers::read_lines_panicky(file_name).skip(2) {
        let mut chars = line.chars();
        let pair = (chars.next().unwrap(), chars.next().unwrap());
        let value = chars.skip(4).next().unwrap();
        mappings.insert(pair, value);
    }
    mappings
}

fn parse_first_last_chars(file_name: &str) -> [char; 2] {
    let line = helpers::read_lines_panicky(file_name).next().unwrap();
    let mut chars = line.chars();
    let first = chars.next().unwrap();
    let last = chars.last().unwrap();
    [first, last]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1588, both_parts("test_input.txt", 10));
    }

    #[test]
    fn final_part1() {
        assert_eq!(3831, both_parts("input.txt", 10));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2188189693529, both_parts("test_input.txt", 40));
    }

    #[test]
    fn final_part2() {
        assert_eq!(5725739914282, both_parts("input.txt", 40));
    }
}