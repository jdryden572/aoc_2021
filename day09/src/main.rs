use std::collections::HashSet;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> u32 {
    let values = parse_values(file_name);

    let matrix = Matrix::new(values);
    matrix
        .low_points()
        .into_iter()
        .map(|Position { val, .. }| val + 1)
        .sum()
}

fn fill_basin_recursive(matrix: &Matrix, pos: &Position, basin: &mut HashSet<Position>) {
    if basin.insert(*pos) {
        for neighbor in matrix.neighbors(pos).into_iter().filter(|p| p.val < 9) {
            fill_basin_recursive(matrix, &neighbor, basin)
        }
    }
}

fn part2(file_name: &str) -> usize {
    let values = parse_values(file_name);
    let matrix = Matrix::new(values);

    let mut basins: Vec<HashSet<Position>> = Vec::new();
    for low_point in matrix.low_points() {
        let mut basin = HashSet::new();
        fill_basin_recursive(&matrix, &low_point, &mut basin);
        basins.push(basin);
    }

    basins.sort_by(|x, y| y.len().cmp(&x.len())); // reverse
    basins.into_iter().take(3).map(|b| b.len()).product()
}

fn parse_values(file_name: &str) -> Vec<Vec<u32>> {
    helpers::read_lines_panicky(file_name)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

struct Matrix {
    max_x: usize,
    max_y: usize,
    positions: Vec<Vec<Position>>,
}

impl Matrix {
    fn new(values: Vec<Vec<u32>>) -> Self {
        let max_x = values[0].len() - 1;
        let max_y = values.len() - 1;
        let mut positions = vec![Vec::new(); max_y + 1];
        for y in 0..=max_y {
            for x in 0..=max_x {
                positions[y].push(Position {
                    x,
                    y,
                    val: values[y][x],
                });
            }
        }
        Self {
            max_x,
            max_y,
            positions,
        }
    }

    fn position(&self, x: usize, y: usize) -> Position {
        self.positions[y][x]
    }

    fn low_points(&self) -> Vec<Position> {
        let mut low_points = Vec::new();
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                let pos = self.position(x, y);
                let neighbors = self.neighbors(&pos);
                if pos.val < neighbors.iter().map(|p| p.val).min().unwrap() {
                    low_points.push(pos);
                }
            }
        }
        low_points
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
    val: u32,
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
