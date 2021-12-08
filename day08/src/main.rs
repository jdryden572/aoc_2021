fn main() {
    println!("Answer one: {}", part1("input.txt"));
}

fn part1(file_name: &str) -> usize {
    helpers::read_lines_panicky(file_name)
        .map(|l| l.split("|").skip(1).next().unwrap().to_string())
        .map(|group| {
            group
                .split_whitespace()
                .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(237, part1("input.txt"));
    }
}
