use std::{cmp::max, time::Instant};

// target area: x=25..67, y=-260..-200
const TARGET: &TargetArea = &TargetArea {
    min_x: 25,
    max_x: 67,
    min_y: -260,
    max_y: -200,
};

fn main() {
    let start = Instant::now();
    println!(
        "Answer one: {} ({:?})",
        part1(TARGET),
        Instant::now() - start
    );

    let start = Instant::now();
    println!(
        "Answer two: {} ({:?})",
        part2(TARGET),
        Instant::now() - start
    );
}

fn part1(target: &TargetArea) -> i32 {
    let mut max_y = i32::MIN;
    for x_vel in 1..68 {
        for y_vel in 1..1000 {
            if let Some(height) = get_height_if_hit(x_vel, y_vel, target) {
                max_y = max(max_y, height);
            }
        }
    }
    max_y
}

fn part2(target: &TargetArea) -> usize {
    let mut hits = 0;
    for x_vel in 1..68 {
        for y_vel in -261..1000 {
            if let Some(_) = get_height_if_hit(x_vel, y_vel, target) {
                hits += 1;
            }
        }
    }
    hits
}

fn get_height_if_hit(x_vel: i32, y_vel: i32, target: &TargetArea) -> Option<i32> {
    let trajectory = Trajectory::new(x_vel, y_vel);
    let mut max_y = i32::MIN;
    for (x, y) in trajectory {
        max_y = max(max_y, y);
        if target.contains(x, y) {
            return Some(max_y);
        }
        if target.is_past(x, y) {
            return None;
        }
    }

    unreachable!()
}

struct Trajectory {
    x_vel: i32,
    y_vel: i32,
    position: (i32, i32),
}

impl Trajectory {
    fn new(x_vel: i32, y_vel: i32) -> Self {
        Self {
            x_vel,
            y_vel,
            position: (0, 0),
        }
    }
}

impl Iterator for Trajectory {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let (mut x, mut y) = self.position;
        x += self.x_vel;
        y += self.y_vel;
        self.x_vel -= self.x_vel.signum();
        self.y_vel -= 1;
        self.position = (x, y);
        Some((x, y))
    }
}

struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl TargetArea {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }

    fn is_past(&self, x: i32, y: i32) -> bool {
        self.max_x < x || y < self.min_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TARGET: &TargetArea = &TargetArea {
        min_x: 20,
        max_x: 30,
        min_y: -10,
        max_y: -5,
    };

    #[test]
    fn test_part1() {
        assert_eq!(45, part1(TEST_TARGET));
    }

    #[test]
    fn final_part1() {
        assert_eq!(33670, part1(TARGET));
    }

    #[test]
    fn test_part2() {
        assert_eq!(112, part2(TEST_TARGET));
    }

    #[test]
    fn final_part2() {
        assert_eq!(4903, part2(TARGET));
    }
}
