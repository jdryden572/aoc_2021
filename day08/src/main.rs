mod decode;
use decode::Decoder;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
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

fn part2(file_name: &str) -> usize {
    let mut num = 0;
    for line in helpers::read_lines_panicky(file_name) {
        let mut split = line.split("|");
        let inputs = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        let readings = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();

        let decoder = Decoder::build(inputs);
        for i in 0..4 {
            let decoded = decoder.decode(&readings[i]);
            let factor = 1000 / 10usize.pow(i as u32);
            num += decoded * factor;
        }
    }

    num
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

    #[test]
    fn test_part2() {
        assert_eq!(61229, part2("test_input.txt"));
    }
}
