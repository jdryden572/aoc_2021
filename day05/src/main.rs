use std::{
    cmp::{max, min},
    collections::HashMap,
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> usize {
    let lines = parse_lines(file_name).filter(|l| l.is_horizontal() || l.is_vertical());

    count_most_dangerous_points(lines)
}

fn part2(file_name: &str) -> usize {
    let lines = parse_lines(file_name);
    count_most_dangerous_points(lines)
}

fn count_most_dangerous_points<I: Iterator<Item = Line>>(lines: I) -> usize {
    let all_points = lines.map(|l| l.points_on_line()).flatten();

    let mut point_counts = HashMap::new();
    for point in all_points {
        let count = point_counts.entry(point).or_insert(0);
        *count += 1;
    }

    point_counts.values().filter(|&c| *c > 1).count()
}

fn parse_lines(file_name: &str) -> impl Iterator<Item = Line> + '_ {
    helpers::read_lines_panicky(file_name).map(|l| Line::from_str(&l).unwrap())
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

    fn points_on_line(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let (left, right) = self.left_to_right();
            (left.x..right.x + 1)
                .map(|x| Point { x, y: self.a.y })
                .collect()
        } else if self.is_vertical() {
            let (top, bottom) = self.top_to_bottom();
            (top.y..bottom.y + 1)
                .map(|y| Point { x: self.a.x, y })
                .collect()
        } else {
            self.points_on_diagonal()
        }
    }

    fn points_on_diagonal(&self) -> Vec<Point> {
        let (left, right) = self.left_to_right();
        let (top, bottom) = self.top_to_bottom();
        if left == top {
            // top-left to bottom-right
            (left.x..right.x + 1)
                .zip(top.y..bottom.y + 1)
                .map(Point::from)
                .collect()
        } else {
            // bottom-left to top-right
            (left.x..right.x + 1)
                .zip((top.y..bottom.y + 1).rev())
                .map(Point::from)
                .collect()
        }
    }

    fn left_to_right(&self) -> (Point, Point) {
        if self.a.x <= self.b.x {
            (self.a, self.b)
        } else {
            (self.b, self.a)
        }
    }

    fn top_to_bottom(&self) -> (Point, Point) {
        if self.a.y <= self.b.y {
            (self.a, self.b)
        } else {
            (self.b, self.a)
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    #[test]
    fn test_part2() {
        assert_eq!(12, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(19939, part2("input.txt"));
    }

    #[test]
    fn points_on_diagonal() {
        let line = Line {
            a: Point { x: 8, y: 0 },
            b: Point { x: 0, y: 8 },
        };
        let expected = vec![
            Point { x: 0, y: 8 },
            Point { x: 1, y: 7 },
            Point { x: 2, y: 6 },
            Point { x: 3, y: 5 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 3 },
            Point { x: 6, y: 2 },
            Point { x: 7, y: 1 },
            Point { x: 8, y: 0 },
        ];
        assert_eq!(expected, line.points_on_line());
    }
}
