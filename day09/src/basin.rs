use std::{collections::{HashSet, VecDeque}, iter::FromIterator, hash::Hash};

pub fn part1(file_name: &str) -> u16 {
    let values = parse_values(file_name);

    let matrix = Matrix::new_with_low_points(values);
    matrix
        .low_points()
        .into_iter()
        .map(|Position { val, .. }| (val + 1) as u16)
        .sum()
}

fn fill_basin_recursive(matrix: &Matrix, pos: &Position, basin: &mut HashSet<Position>) {
    if basin.insert(*pos) {
        for neighbor in matrix.neighbors(pos).into_iter().filter(|p| p.val < 9) {
            fill_basin_recursive(matrix, &neighbor, basin)
        }
    }
}

pub fn part2(file_name: &str) -> usize {
    let values = parse_values(file_name);
    let mut matrix = Matrix::new_with_low_points(values);

    let mut searchers = matrix.low_points().into_iter().map(|p| BasinSearcher::new(p, &matrix)).collect::<VecDeque<_>>();
    let mut finished = Vec::new();
    while !searchers.is_empty() {
        for _ in 0..searchers.len() {
            let mut searcher = searchers.pop_front().unwrap();
            searcher.step(&mut matrix);
            if !searcher.is_done() {
                searchers.push_back(searcher);
            } else {
                finished.push(searcher);
            }
        }
    }

    finished.sort_by(|x, y| y.visited.len().cmp(&x.visited.len())); // reverse
    finished.into_iter().take(3).map(|b| b.visited.len()).product()
}

pub fn parse_values(file_name: &str) -> Vec<Vec<u8>> {
    helpers::read_lines_panicky(file_name)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub struct BasinSearcher {
    low_point: Position,
    visited: HashSet<Position>,
    frontier: Vec<Position>,
}

impl BasinSearcher {
    pub fn new(low_point: Position, matrix: &Matrix) -> Self {
        Self {
            low_point,
            visited: HashSet::from_iter([low_point]),
            frontier: Vec::from_iter(matrix.neighbors(&low_point).into_iter().filter(|p| p.kind != PositionType::HighPoint)),
        }
    }

    pub fn step(&mut self, matrix: &mut Matrix) {
        if let Some(mut pos) = self.frontier.pop() {
            if pos.kind != PositionType::LowPoint {
                pos.kind = PositionType::InBasin;
            }
            if self.visited.insert(pos) {
                if pos.kind != PositionType::LowPoint {
                    matrix.position_mut(pos.x, pos.y).kind = PositionType::InBasin;
                }
                let neighbors_in_basin = matrix.neighbors(&pos).into_iter().filter(|p| p.kind != PositionType::HighPoint && !self.visited.contains(p)).collect::<Vec<_>>();
                self.frontier.extend(neighbors_in_basin);
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.frontier.is_empty()
    }
}

pub struct Matrix {
    max_x: usize,
    max_y: usize,
    positions: Vec<Vec<Position>>,
}

impl Matrix {
    pub fn new_with_low_points(values: Vec<Vec<u8>>) -> Self {
        let max_x = values[0].len() - 1;
        let max_y = values.len() - 1;
        let mut positions = vec![Vec::new(); max_y + 1];
        for y in 0..=max_y {
            for x in 0..=max_x {
                let val = values[y][x];
                positions[y].push(Position {
                    x,
                    y,
                    val,
                    kind: PositionType::from(val)
                });
            }
        }
        let mut matrix = Self {
            max_x,
            max_y,
            positions,
        };
        matrix.mark_low_points();
        matrix
    }

    pub fn iter(&self) -> impl Iterator<Item = &Position> {
        self.positions.iter().flat_map(|row| row.iter())
    }

    fn position_copy(&self, x: usize, y: usize) -> Position {
        self.positions[y][x]
    }

    fn position_mut(&mut self, x: usize, y: usize) -> &mut Position {
        self.positions.get_mut(y).unwrap()
            .get_mut(x).unwrap()
    }

    fn update_position(&mut self, pos: Position) {
        self.positions[pos.y][pos.x] = pos;
    }

    fn mark_low_points(&mut self) {
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                let mut pos = self.position_copy(x, y);
                let neighbors = self.neighbors(&pos);
                if pos.val < neighbors.iter().map(|p| p.val).min().unwrap() {
                    pos.kind = PositionType::LowPoint;
                    self.update_position(pos);
                }
            }
        }
    }

    pub fn low_points(&self) -> Vec<Position> {
        self.iter()
            .filter(|&p| p.kind == PositionType::LowPoint)
            .copied()
            .collect()
    }

    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if pos.x > 0 {
            neighbors.push(self.positions[pos.y][pos.x - 1]);
        }
        if pos.x < self.max_x {
            neighbors.push(self.positions[pos.y][pos.x + 1]);
        }
        if pos.y > 0 {
            neighbors.push(self.positions[pos.y - 1][pos.x]);
        }
        if pos.y < self.max_y {
            neighbors.push(self.positions[pos.y + 1][pos.x]);
        }
        neighbors
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub val: u8,
    pub kind: PositionType,
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum PositionType {
    Unknown,
    LowPoint,
    InBasin,
    HighPoint,
}

impl From<u8> for PositionType {
    fn from(val: u8) -> Self {
        match val {
            9 => Self::HighPoint,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(15, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(462, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1134, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(1397760, part2("input.txt"));
    }
}