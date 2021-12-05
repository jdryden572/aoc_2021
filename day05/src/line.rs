use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub fn points_on_line(&self) -> Vec<Point> {
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
pub struct Point {
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
