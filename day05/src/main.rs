use std::{str::FromStr, num::ParseIntError, cmp::{max, min}, collections::HashMap};

fn main() {
    println!("Answer one: {}", part1("input.txt"));
}

fn part1(file_name: &str) -> usize {
    let lines = parse_lines(file_name)
        .filter(|l| l.is_horizontal() || l.is_vertical());

    let all_points = lines.map(|l| l.points_on_line()).flatten();

    let mut point_counts = HashMap::new();
    for point in all_points {
        let count = point_counts.entry(point).or_insert(0);
        *count += 1;
    }

    point_counts.values().filter(|&c| *c > 1).count()
}

fn parse_lines(file_name: &str) -> impl Iterator<Item = Line> + '_ {
    helpers::read_lines_panicky(file_name)
        .map(|l| Line::from_str(&l).unwrap())
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn min_x(&self) -> i32 {
        min(self.a.x, self.b.x)
    }

    fn max_x(&self) -> i32 {
        max(self.a.x, self.b.x)
    }

    fn min_y(&self) -> i32 {
        min(self.a.y, self.b.y)
    }

    fn max_y(&self) -> i32 {
        max(self.a.y, self.b.y)
    }

    fn points_on_line(&self) -> Vec<Point> {
        if self.is_horizontal() {
            (self.min_x()..self.max_x() + 1).map(|x| Point { x, y: self.a.y }).collect()
        } else {
            (self.min_y()..self.max_y() + 1).map(|y| Point { x: self.a.x, y }).collect()
        }
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let a = Point::from_str(parts.next().unwrap())?;
        let b = Point::from_str(parts.next().unwrap())?;
        Ok(Self { a, b })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().parse()?;
        let y = parts.next().unwrap().parse()?;
        Ok(Self { x, y })
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
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
}