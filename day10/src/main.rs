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

    // the prompt promises that there will be an odd number of lines, so this will pick the middle score.
    scores.sort();
    scores[scores.len() / 2]
}

fn get_completion_string_score(line: String) -> usize {
    let mut opening_tags = Vec::new();
    for tag in line.chars().map(Tag::from) {
        match tag {
            Tag::Open(_) => opening_tags.push(tag),
            Tag::Close(_) => { opening_tags.pop().unwrap(); }
        }
    }

    opening_tags
        .iter()
        .rev()
        .map(Tag::get_matching_close)
        .map(completion_char_score)
        .fold(0, |total, score| total * 5 + score)
}

fn completion_char_score(tag: Tag) -> usize {
    [')', ']', '}', '>']
        .iter()
        .position(|&c| c == tag.into_inner())
        .unwrap()
        + 1
}

fn get_illegal_closing_score(line: &str) -> u32 {
    let mut stack = Vec::new();
    for tag in line.chars().map(Tag::from) {
        match tag {
            Tag::Open(_) => stack.push(tag),
            Tag::Close(_) => {
                if !stack.pop().unwrap().matches(&tag) {
                    return tag.illegal_close_score();
                }
            }
        }
    }

    0
}

#[derive(PartialEq, Eq)]
enum Tag {
    Open(char),
    Close(char),
}

impl Tag {
    fn matches(&self, other: &Tag) -> bool {
        match (self, other) {
            (Tag::Open(_), Tag::Close(_)) => other == &self.get_matching_close(),
            (Tag::Close(_), Tag::Open(_)) => self == &other.get_matching_close(),
            _ => false
        }
    }

    fn get_matching_close(&self) -> Tag {
        match self {
            Tag::Close(_) => panic!("Don't do that!"),
            Tag::Open(open) => match *open {
                '(' => ')'.into(),
                '[' => ']'.into(),
                '{' => '}'.into(),
                '<' => '>'.into(),
                _ => panic!("Unrecognized open char"),
            }
        }
    }

    fn illegal_close_score(&self) -> u32 {
        match self {
            Tag::Open(_) => 0,
            Tag::Close(close) => match *close {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        }
    }

    fn into_inner(&self) -> char {
        match self {
            Tag::Open(c) => *c,
            Tag::Close(c) => *c,
        }
    }
}

impl From<char> for Tag {
    fn from(c: char) -> Self {
        match c {
            '(' | '[' | '{' | '<' => Tag::Open(c),
            ')' | ']' | '}' | '>' => Tag::Close(c),
            _ => panic!("Unrecognized char"),
        }
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
