fn main() {
    println!("Answer one: {}", part1("input.txt"));
}

fn part1(file_name: &str) -> i32 {

    let mut boards = parse_boards(file_name);
    let numbers = parse_called_numbers(file_name);

    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.bingo() {
                return board.unmarked_total() * number;
            }
        }
    }

    panic!("Didn't find a winner!")
}

#[derive(Debug)]
struct Board {
    rows: [[Number; 5]; 5],
}

impl Board {
    fn mark(&mut self, number: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.rows[row][col] == Number::Unmarked(number) {
                    self.rows[row][col] = Number::Marked(number);
                    return;
                }
            }
        }
    }

    fn columns(&self) -> [[Number; 5]; 5] {
        let mut columns = [[Number::Unmarked(0); 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                columns[col][row] = self.rows[row][col];
            }
        }

        columns
    }

    fn bingo(&self) -> bool {
        self.has_full_row() || self.has_full_column()
    }

    fn has_full_row(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(Number::is_marked))
    }

    fn has_full_column(&self) -> bool {
        self.columns().iter().any(|col| col.iter().all(Number::is_marked))
    }

    fn unmarked_total(&self) -> i32 {
        let mut score = 0;
        for row in self.rows {
            for number in row {
                if let Number::Unmarked(num) = number {
                    score += num;
                }
            }
        }

        score
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Number {
    Unmarked(i32),
    Marked(i32),
}

impl Number {
    fn is_marked(&self) -> bool {
        match self {
            Number::Marked(_) => true,
            Number::Unmarked(_) => false,
        }
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Number::Unmarked(n)
    }
}

fn parse_boards(file_name: &str) -> Vec<Board> {
    let mut boards = Vec::new();

    let lines: Vec<String> = helpers::read_lines_panicky(file_name)
        .skip(1)
        .filter(|l| !l.is_empty())
        .collect();
    
    let chunks = lines.chunks_exact(5);
    for chunk in chunks {
        let mut rows: [[Number; 5]; 5] = [[Number::Unmarked(0); 5]; 5];
        for (i, row_str) in chunk.iter().enumerate() {
            let mut nums = row_str.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            rows[i] = [
                nums.next().unwrap().into(),
                nums.next().unwrap().into(),
                nums.next().unwrap().into(),
                nums.next().unwrap().into(),
                nums.next().unwrap().into(),
            ];
        }
        boards.push(Board { rows });
    }
    
    boards
}

fn parse_called_numbers(file_name: &str) -> Vec<i32> {
    helpers::read_lines_panicky(file_name)
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(4512, part1("test_input.txt"));
    }
}