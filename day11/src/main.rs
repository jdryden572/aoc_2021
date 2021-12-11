use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    iter::FromIterator,
};

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

const GRID_SIZE: usize = 10;

fn part1(file_name: &str) -> usize {
    let vals = helpers::read_lines_panicky(file_name).flat_map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>()
    });

    let mut matrix = Matrix::from_iter(vals);
    //println!("{}", matrix);

    let mut flash_count = 0;

    for _i in 1..=100 {
        //println!("Step {}", i);
        let mut flash_queue = VecDeque::new();
        let mut flashes = HashSet::new();

        for pos in matrix.positions.iter_mut() {
            pos.val += 1;
            if pos.val > 9 {
                flash_queue.push_back(pos.xy());
            }
        }

        while let Some((x, y)) = flash_queue.pop_front() {
            if flashes.insert((x, y)) {
                let neighbors = matrix.neighbors(x, y);
                for (neighbor_x, neighbor_y) in neighbors {
                    let pos = matrix.position_mut(neighbor_x, neighbor_y);
                    if pos.val < 10 {
                        pos.val += 1;
                    }
                    if pos.val == 10 {
                        flash_queue.push_back(pos.xy());
                    }
                }
            }
        }

        flash_count += flashes.len();

        for pos in matrix.positions.iter_mut() {
            if pos.val > 9 {
                pos.val = 0;
            }
        }

        //println!("{}", matrix);
    }

    flash_count
}

fn part2(file_name: &str) -> usize {
    let vals = helpers::read_lines_panicky(file_name).flat_map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>()
    });

    let mut matrix = Matrix::from_iter(vals);
    //println!("{}", matrix);

    for i in 1..=500 {
        //println!("Step {}", i);
        let mut flash_queue = VecDeque::new();
        let mut flashes = HashSet::new();

        for pos in matrix.positions.iter_mut() {
            pos.val += 1;
            if pos.val > 9 {
                flash_queue.push_back(pos.xy());
            }
        }

        while let Some((x, y)) = flash_queue.pop_front() {
            if flashes.insert((x, y)) {
                let neighbors = matrix.neighbors(x, y);
                for (neighbor_x, neighbor_y) in neighbors {
                    let pos = matrix.position_mut(neighbor_x, neighbor_y);
                    if pos.val < 10 {
                        pos.val += 1;
                    }
                    if pos.val == 10 {
                        flash_queue.push_back(pos.xy());
                    }
                }
            }
        }

        if flashes.len() == 100 {
            return i;
        }

        for pos in matrix.positions.iter_mut() {
            if pos.val > 9 {
                pos.val = 0;
            }
        }

        //println!("{}", matrix);
    }

    panic!("Too many loops!")
}

struct Matrix {
    positions: Vec<Position>,
}

impl Matrix {
    // fn neighbors_of(&self, pos: &Position) -> Vec<(u8, u8)> {
    //     let (x, y) = (pos.x, pos.y);
    //     self.neighbors(x, y)
    // }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x, y) = (x as i32, y as i32);
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < GRID_SIZE as i32 && *y >= 0 && *y < GRID_SIZE as i32)
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
    }

    fn position_mut(&mut self, x: usize, y: usize) -> &mut Position {
        self.positions.get_mut(index_of(x, y)).unwrap()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.positions.chunks_exact(10) {
            for p in row {
                write!(f, "{}", p.val)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl FromIterator<usize> for Matrix {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let positions = iter
            .into_iter()
            .enumerate()
            .map(|(i, val)| (i, val))
            .map(|(i, val)| Position {
                x: i % GRID_SIZE,
                y: i / GRID_SIZE,
                val,
            })
            .collect::<Vec<_>>();

        Self { positions }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    val: usize,
}

impl Position {
    // fn index_of(&self) -> usize {
    //     index_of(self.x, self.y)
    // }

    fn xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

fn index_of(x: usize, y: usize) -> usize {
    (y * GRID_SIZE + x) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1656, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(1700, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(195, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(273, part2("input.txt"));
    }
}
