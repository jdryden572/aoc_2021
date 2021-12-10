fn main() {
    println!("Answer one: {}", part1("input.txt"));
}

fn part1(file_name: &str) -> u32 {
    helpers::read_lines_panicky(file_name)
        .map(get_line_score)
        .sum()
}

fn get_line_score(line: String) -> u32 {
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
        _ => panic!("Unrecognized char for score"),
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
}
