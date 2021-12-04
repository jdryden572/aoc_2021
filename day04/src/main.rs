mod bingo;

use std::collections::VecDeque;
use bingo::Board;

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
