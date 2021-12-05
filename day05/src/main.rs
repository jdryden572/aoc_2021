use std::{collections::HashMap, str::FromStr};

mod line;
use line::Line;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> usize {
    let lines = parse_lines(file_name).filter(|l| l.is_horizontal() || l.is_vertical());

    count_most_dangerous_points(lines)
}

fn part2(file_name: &str) -> usize {
    let lines = parse_lines(file_name);
    count_most_dangerous_points(lines)
}

fn count_most_dangerous_points<I: Iterator<Item = Line>>(lines: I) -> usize {
    let all_points = lines.flat_map(|l| l.points_on_line());

    let mut point_counts = HashMap::new();
    for point in all_points {
        let count = point_counts.entry(point).or_insert(0);
        *count += 1;
    }

    point_counts.values().filter(|&c| *c > 1).count()
}

fn parse_lines(file_name: &str) -> impl Iterator<Item = Line> + '_ {
    helpers::read_lines_panicky(file_name).map(|l| Line::from_str(&l).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(7318, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(19939, part2("input.txt"));
    }
}
