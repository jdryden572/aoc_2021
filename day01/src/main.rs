fn main() {
    let answer = part1("input.txt");
    println!("{}", answer);

    let answer_two = part2("input.txt");
    println!("{}", answer_two);
}

fn part1(file_name: &str) -> usize {
    parse_ints_from_file(file_name)
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}

fn part2(file_name: &str) -> usize {
    parse_ints_from_file(file_name)
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}

fn parse_ints_from_file(file_name: &str) -> Vec<i32> {
    helpers::read_lines_panicky(file_name)
        .map(|l| l.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = part1("test_input_1.txt");
        assert_eq!(3, answer);
    }

    #[test]
    fn part1_final() {
        let answer = part1("input.txt");
        assert_eq!(1521, answer);
    }

    #[test]
    fn test_part2() {
        let answer = part2("test_input_2.txt");
        assert_eq!(5, answer);
    }

    #[test]
    fn part2_final() {
        let answer = part2("input.txt");
        assert_eq!(1543, answer);
    }
}
