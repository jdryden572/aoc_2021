fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> u32 {
    helpers::read_lines_panicky(file_name)
        .map(|l| get_illegal_closing_score(&l))
        .sum()
}

fn part2(file_name: &str) -> usize {
    let mut scores = helpers::read_lines_panicky(file_name)
        .filter(|l| get_illegal_closing_score(&l) == 0)
        .map(get_completion_string_score)
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn get_completion_string_score(line: String) -> usize {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if !tag_pair_matches(stack.pop().unwrap(), c) {
                    panic!("This line shouldn't have illegal closings")
                }
            }
            _ => panic!("Unrecognized char"),
        }
    }

    let completion_seq = stack.into_iter().rev().map(get_matching_close);
    get_completion_score(completion_seq)
}

fn get_completion_score<I: Iterator<Item = char>>(completion: I) -> usize {
    completion.map(get_completion_char_score).fold(0, |acc,  score| acc * 5 + score)
}

fn get_completion_char_score(close: char) -> usize {
    [')', ']', '}', '>'].iter().position(|&c| c == close).unwrap() + 1
}

fn get_matching_close(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unrecognized open char"),
    }
}

fn get_illegal_closing_score(line: &str) -> u32 {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if !tag_pair_matches(stack.pop().unwrap(), c) {
                    return tag_score(c);
                }
            }
            _ => panic!("Unrecognized char"),
        }
    }

    0
}

fn tag_pair_matches(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn tag_score(close: char) -> u32 {
    match close {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26397, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(290691, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(2768166558, part2("input.txt"));
    }
}
