mod rotate;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use rotate::Rotate;

type Point = (i32, i32, i32);

fn main() {
    let start = Instant::now();
    let (one, two) = both_parts("input.txt");
    let elapsed = Instant::now() - start;
    println!("Answer one: {} ({:?})", one, elapsed);
    //println!("Answer two: {} ({:?})", two, elapsed);
}

fn both_parts(file_name: &str) -> (usize, usize) {
    let mut scanners = parse_scanners(file_name);
    let scanner_zero = scanners.pop_front().unwrap();

    let mut beacons = HashSet::<_>::from_iter(scanner_zero.points);
    let mut scanner_distances = Vec::new();
    let mut rotations = HashSet::new();
    let mut calculations = 0usize;

    while let Some(scanner) = scanners.pop_front() {
        //println!("Scanner {}", scanner.id);
        let mut found = false;
        for (i, points) in Rotate::new(&scanner.points).enumerate() {
            let mut distances: HashMap<Point, usize> = HashMap::new();
            for &point in points.iter() {
                for &orig in beacons.iter() {
                    calculations += 1;
                    *distances.entry(diff(point, orig)).or_default() += 1;
                }
            }

            if let Some((distance, _)) = distances.iter().find(|&(_, c)| c >= &12) {
                scanner_distances.push(*distance);
                // println!(
                //     "Distance from scanner 0: {},{},{}",
                //     distance.0, distance.1, distance.2
                // );
                rotations.insert(i);
                //println!("Found after {} rotations", i);
                found = true;
                beacons.extend(points.into_iter().map(|p| diff(p, *distance)));
                break;
            }
        }
        if !found {
            scanners.push_back(scanner);
        }
    }

    println!("Unique rotations: {}", rotations.len());
    println!("Vector calculations: {}", calculations);

    let mut max = 0;
    for &distance in scanner_distances.iter() {
        for &other in scanner_distances.iter() {
            let manhattan = (distance.0 - other.0).abs()
                + (distance.1 - other.1).abs()
                + (distance.2 - other.2).abs();
            max = std::cmp::max(max, manhattan);
        }
    }

    (beacons.len(), max as usize)
}

fn diff(left: Point, right: Point) -> Point {
    (left.0 - right.0, left.1 - right.1, left.2 - right.2)
}

struct Scanner {
    id: i32,
    points: Vec<Point>,
}

fn parse_scanners(file_name: &str) -> VecDeque<Scanner> {
    let lines = helpers::read_lines_panicky(file_name).collect::<Vec<_>>();
    let mut scanners = VecDeque::new();
    for chunk in lines.split(|l| l.is_empty()) {
        let mut chunk = chunk.into_iter();
        let id = chunk
            .next()
            .unwrap()
            .split_whitespace()
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        let points = chunk
            .map(|l| {
                let mut coords = l.split(",");
                (
                    coords.next().unwrap().parse::<i32>().unwrap(),
                    coords.next().unwrap().parse::<i32>().unwrap(),
                    coords.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        scanners.push_back(Scanner { id, points });
    }
    scanners
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(79, both_parts("test_input.txt").0);
    }

    #[test]
    fn final_part1() {
        assert_eq!(472, both_parts("input.txt").0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(3621, both_parts("test_input.txt").1);
    }

    #[test]
    fn final_part2() {
        assert_eq!(12092, both_parts("input.txt").1);
    }
}
