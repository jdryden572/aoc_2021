mod rotate;
use std::{time::Instant, collections::{VecDeque, HashSet, HashMap}};

use rotate::Rotate;

type Point = (i32, i32, i32);

fn main() {
    let start = Instant::now();
    println!("Answer one: {} ({:?})", part1("input.txt"), Instant::now() - start);
}

fn part1(file_name: &str) -> usize {
    let mut scanners = parse_scanners(file_name);
    let scanner_zero = scanners.pop_front().unwrap();
    
    let mut beacons = HashSet::<_>::from_iter(scanner_zero.points);

    while let Some(scanner) = scanners.pop_front() {
        println!("Scanner {}", scanner.id);
        let mut found = false;
        for (i, points) in Rotate::new(&scanner.points).enumerate() {
            let mut distances: HashMap<Point, usize> = HashMap::new();
            for &point in points.iter() {
                for &orig in beacons.iter() {
                    *distances.entry(diff(point, orig)).or_default() += 1;
                }
            }

            if let Some((distance, _)) = distances.iter().find(|&(_, c)| c >= &12) {
                println!("Found after {} rotations", i);
                found = true;
                beacons.extend(points.into_iter().map(|p| diff(p, *distance)));
                break;
            }
        }
         if !found {
             scanners.push_back(scanner);
         }
    }

    beacons.len()
}

fn diff(left: Point, right: Point) -> Point {
    (
        left.0 - right.0,
        left.1 - right.1,
        left.2 - right.2,
    )
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
        let id = chunk.next().unwrap().split_whitespace().nth(2).unwrap().parse().unwrap();
        let points = chunk.map(|l| {
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
        assert_eq!(79, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(472, part1("input.txt"));
    }
}