use itertools::Itertools;
use std::{collections::HashSet, ops::RangeInclusive, time::Instant};

type Cube = [RangeInclusive<i32>; 3];

fn main() {
    let start = Instant::now();
    println!(
        "Answer one: {} ({:?})",
        part1("input.txt"),
        Instant::now() - start
    );
}

fn part1(file_name: &str) -> usize {
    let commands = parse_inputs(file_name);

    let mut cubes_on = HashSet::new();
    for command in commands {
        if small_cube(command.cube()) {
            match command {
                Command::On(cube) => {
                    for point in cube.iter().cloned().multi_cartesian_product() {
                        cubes_on.insert((point[0], point[1], point[2]));
                    }
                }
                Command::Off(cube) => {
                    for point in cube.iter().cloned().multi_cartesian_product() {
                        cubes_on.remove(&(point[0], point[1], point[2]));
                    }
                }
            }
        }
    }
    cubes_on.len()
}

fn small_cube(cube: &Cube) -> bool {
    cube.iter().all(|r| r.start() >= &-50 && r.end() <= &50)
}

fn parse_inputs(file_name: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    for line in helpers::read_lines_panicky(file_name) {
        if line.starts_with("on") {
            let command = Command::On(parse_ranges(&line[3..]));
            commands.push(command);
        } else {
            let command = Command::Off(parse_ranges(&line[4..]));
            commands.push(command);
        }
    }
    commands
}

fn parse_ranges(input: &str) -> Cube {
    let mut split = input.split(",");
    [
        parse_range(split.next().unwrap()),
        parse_range(split.next().unwrap()),
        parse_range(split.next().unwrap()),
    ]
}

fn parse_range(input: &str) -> RangeInclusive<i32> {
    let input = &input[2..];
    let (start, end) = input.split_once("..").unwrap();
    let start = start.parse().unwrap();
    let end = end.parse().unwrap();
    start..=end
}

#[derive(Debug)]
enum Command {
    On(Cube),
    Off(Cube),
}

impl Command {
    fn cube(&self) -> &Cube {
        match self {
            Command::On(cube) => cube,
            Command::Off(cube) => cube,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        assert_eq!(39, part1("test_input_small.txt"));
    }

    #[test]
    fn test_part1_large() {
        assert_eq!(590784, part1("test_input_large.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(542711, part1("input.txt"));
    }
}
