use std::collections::VecDeque;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> i32 {
    let mut boards = parse_boards(file_name);
    let numbers = parse_called_numbers(file_name);

    println!("Loaded {} boards, starting the game.", boards.len());

    for number in numbers {
        println!("Calling: {}", number);

        for board in boards.iter_mut() {
            board.mark(number);
            if board.bingo() {
                println!("Bingo! Board {} has won!", board.index);
                return board.unmarked_total() * number;
            }
        }
    }

    panic!("Didn't find a winner!")
}

fn part2(file_name: &str) -> i32 {
    let mut boards: VecDeque<Board> = parse_boards(file_name).into();
    let numbers = parse_called_numbers(file_name);

    println!(
        "Loaded {} boards, beginning search for worst board.",
        boards.len()
    );

    for number in numbers {
        println!("Calling: {}", number);

        if boards.len() > 1 {
            for _ in 0..boards.len() {
                let mut board = boards.pop_front().unwrap();
                board.mark(number);
                if !board.bingo() {
                    boards.push_back(board);
                } else {
                    println!("Removing board {}", board.index);
                }
            }
        } else {
            let board = boards.get_mut(0).unwrap();
            board.mark(number);
            if board.bingo() {
                println!(
                    "Found the worst board! Board {} is the biggest stinker.",
                    board.index
                );
                return board.unmarked_total() * number;
            }
        }
    }

    panic!("Didn't find the last board to win!")
}

#[derive(Debug)]
struct Board {
    index: usize,
    rows: [[Number; 5]; 5],
}

impl Board {
    fn new<I: Iterator<Item = i32>>(index: usize, mut numbers: I) -> Self {
        let mut rows: [[Number; 5]; 5] = [[Number::Unmarked(0); 5]; 5];
        for row in 0..5 {
            for col in 0..5 {
                rows[row][col] = numbers.next().unwrap().into();
            }
        }
        Self { index, rows }
    }

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
        self.rows
            .iter()
            .any(|row| row.iter().all(Number::is_marked))
    }

    fn has_full_column(&self) -> bool {
        self.columns()
            .iter()
            .any(|col| col.iter().all(Number::is_marked))
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
    let lines: Vec<String> = helpers::read_lines_panicky(file_name)
        .skip(1)
        .filter(|l| !l.is_empty())
        .collect();

    lines
        .chunks_exact(5)
        .map(flatten_single_board)
        .enumerate()
        .map(|(i, nums)| Board::new(i, nums))
        .collect()
}

fn flatten_single_board(chunk: &[String]) -> impl Iterator<Item = i32> + '_ {
    chunk
        .iter()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()))
        .flatten()
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

    #[test]
    fn final_part1() {
        assert_eq!(69579, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1924, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(14877, part2("input.txt"));
    }
}
