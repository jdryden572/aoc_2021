fn main() {
    println!("Answer one: {}", part1("input.txt"));
}

fn part1(file_name: &str) -> u32 {
    let values = helpers::read_lines_panicky(file_name)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let matrix = Matrix::new(values);
    let mut sum = 0;
    for x in 0..=matrix.max_x {
        for y in 0..=matrix.max_y {
            let value = matrix.value(x, y);
            let neighbors = matrix.neighbors(x, y);
            if &value < neighbors.iter().min().unwrap() {
                sum += value + 1;
            }
        }
    }

    sum
}

struct Matrix {
    max_x: usize,
    max_y: usize,
    values: Vec<Vec<u32>>,
}

impl Matrix {
    fn new(values: Vec<Vec<u32>>) -> Self {
        let max_x = values[0].len() - 1;
        let max_y = values.len() - 1;
        Self {
            max_x,
            max_y,
            values,
        }
    }

    fn value(&self, x: usize, y: usize) -> u32 {
        self.values[y][x]
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<u32> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push(self.values[y][x - 1]);
        }
        if x < self.max_x {
            neighbors.push(self.values[y][x + 1]);
        }
        if y > 0 {
            neighbors.push(self.values[y - 1][x]);
        }
        if y < self.max_y {
            neighbors.push(self.values[y + 1][x]);
        }
        neighbors
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
}
