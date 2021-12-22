use itertools::Itertools;
use std::{collections::HashSet, ops::RangeInclusive, time::Instant, cmp::{max, min}};

fn main() {
    let start = Instant::now();
    println!(
        "Answer one: {} ({:?})",
        part1("input.txt"),
        Instant::now() - start
    );

    let start = Instant::now();
    println!(
        "Answer two: {} ({:?})",
        part2("input.txt"),
        Instant::now() - start
    );
}

fn part1(file_name: &str) -> usize {
    let commands = parse_inputs(file_name);

    let mut cubes_on = HashSet::new();
    for command in commands {
        if command.cube().is_small() {
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

fn part2(file_name: &str) -> usize {
    let commands = parse_inputs(file_name);
    reboot_sequence(&commands)
}

fn reboot_sequence(commands: &[Command]) -> usize { 
    let mut changed = 0i64;
    for i in 0..commands.len() {
        changed += cubes_changed_by(i, commands);
    }
    changed as usize
}

fn cubes_changed_by(i: usize, commands: &[Command]) -> i64 {
    let mut changed = 0;
    let command = &commands[i];
    match command {
        Command::On(cuboid) => {
            changed += cuboid.num_points() as i64;
            let overlaps = find_overlaps(cuboid, &commands[0..i]);
            for j in 0..overlaps.len() {
                changed -= cubes_changed_by(j, &overlaps);
            }
        }
        Command::Off(cuboid) => {
            let overlaps = find_overlaps(cuboid, &commands[0..i]);
            for j in 0..overlaps.len() {
                changed -= cubes_changed_by(j, &overlaps);
            }
        }
    }

    changed
}

fn find_overlaps(cuboid: &Cuboid, commands: &[Command]) -> Vec<Command> {
    commands.iter().cloned().filter_map(|p| cuboid.overlaps(p.cube()).and_then(|c| Some(match p {
        Command::On(_) => Command::On(c),
        Command::Off(_) => Command::Off(c),
    }))).collect()
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

fn parse_ranges(input: &str) -> Cuboid {
    let mut split = input.split(",");
    Cuboid([
        parse_range(split.next().unwrap()),
        parse_range(split.next().unwrap()),
        parse_range(split.next().unwrap()),
    ])
}

fn parse_range(input: &str) -> RangeInclusive<i32> {
    let input = &input[2..];
    let (start, end) = input.split_once("..").unwrap();
    let start = start.parse().unwrap();
    let end = end.parse().unwrap();
    start..=end
}

#[derive(Debug, Clone)]
enum Command {
    On(Cuboid),
    Off(Cuboid),
}

impl Command {
    fn cube(&self) -> &Cuboid {
        match self {
            Command::On(cube) => cube,
            Command::Off(cube) => cube,
        }
    }
}

#[derive(Debug, Clone)]
struct Cuboid([RangeInclusive<i32>; 3]);

impl Cuboid {
    fn iter(&self) -> impl Iterator<Item = &RangeInclusive<i32>> {
        self.0.iter()
    }

    fn is_small(&self) -> bool {
        self.iter().all(|r| r.start() >= &-50 && r.end() <= &50)
    }

    fn num_points(&self) -> usize {
        let c = &self.0;
        let width = (c[0].end() - c[0].start() + 1) as usize;
        let height = (c[1].end() - c[1].start() + 1) as usize;
        let depth = (c[2].end() - c[2].start() + 1) as usize;
        width * height * depth
    }

    fn overlaps(&self, other: &Cuboid) -> Option<Cuboid> {
        let c = &self.0;
        let o = &other.0;
        let x = range_overlaps(&c[0], &o[0])?;
        let y = range_overlaps(&c[1], &o[1])?;
        let z = range_overlaps(&c[2], &o[2])?;
        Some(Cuboid([x, y, z]))
    }
}

fn range_overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    if a.start() > b.end() || a.end() < b.start() {
        None
    } else {
        let &start = max(a.start(), b.start());
        let &end = min(a.end(), b.end());
        Some(start..=end)
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

    #[test]
    fn test_part2() {
        assert_eq!(2758514936282235, part2("test_input_full.txt"));
    }

    #[test]
    fn test_part2_small() {
        let commands = parse_inputs(r"D:\rust\aoc_2021\day22\test_input_small.txt").into_iter().filter(|c| c.cube().is_small()).collect::<Vec<_>>();
        assert_eq!(39, reboot_sequence(&commands));
    }

    #[test]
    fn test_part2_large() {
        let commands = parse_inputs("test_input_large.txt").into_iter().filter(|c| c.cube().is_small()).collect::<Vec<_>>();
        assert_eq!(590784, reboot_sequence(&commands));
    }

    #[test]
    fn final_part2() {
        assert_eq!(1160303042684776, part2("input.txt"));
    }
}
