use crate::Point;

pub struct Rotate {
    points: Vec<Point>,
    idx: i32,
}

impl Rotate {
    pub fn new(points: &Vec<Point>) -> Self {
        Self {
            points: Vec::from_iter(points.into_iter().cloned()),
            idx: 0,
        }
    }
}

impl Iterator for Rotate {
    type Item = Vec<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < 24 {
            let rotated = self.points.iter().map(|&p| rotate(p, self.idx)).collect();
            self.idx += 1;
            Some(rotated)
        } else {
            None
        }
    }
}

fn rotate(point: Point, n: i32) -> Point {
    let (x, y, z) = point;

    // figuring these out with the right-hand rule has physically hurt me
    match n {
        // facing positive x
        0 => (x, y, z),
        1 => (x, -z, y),
        2 => (x, -y, -z),
        3 => (x, z, -y),

        // facing negative x
        4 => (-x, y, -z),
        5 => (-x, z, y),
        6 => (-x, -y, z),
        7 => (-x, -z, -y),

        // facing positive y
        8 => (y, -x, z),
        9 => (y, -z, -x),
        10 => (y, x, -z),
        11 => (y, z, x),

        // facing negative y
        12 => (-y, x, z),
        13 => (-y, -z, x),
        14 => (-y, -x, -z),
        15 => (-y, z, -x),

        // facing positive z
        16 => (z, y, -x),
        17 => (z, x, y),
        18 => (z, -y, x),
        19 => (z, -x, -y),

        // facing negative z
        20 => (-z, y, x),
        21 => (-z, -x, y),
        22 => (-z, -y, -x),
        23 => (-z, x, -y),
        
        _ => panic!("Oops!"),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_point_rotate() {
        let point = (1, 2, 3);
        let mut points = HashSet::new();
        for n in 0..24 {
            let rotated = rotate(point, n);
            println!("{},{},{}", rotated.0, rotated.1, rotated.2);
            points.insert(rotated);
        }
        assert_eq!(24, points.len());
    }

    #[test]
    fn test_rotate_iterator() {
        let points = vec![(1, 2, 3), (4, 5, 6)];
        let mut rotate = Rotate::new(&points);
        let mut all_points = HashSet::new();
        while let Some(next) = rotate.next() {
            all_points.extend(next);
        }

        assert_eq!(48, all_points.len());
    }
}