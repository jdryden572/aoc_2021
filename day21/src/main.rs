use std::{time::Instant, cmp::min};

fn main() {
    let start = Instant::now();
    println!("Answer one: {} ({:?})", part1(4, 5), Instant::now() - start);
}

fn part1(mut pos1: usize, mut pos2: usize) -> usize {
    let mut score1 = 0;
    let mut score2 = 0;
    let mut roll = (1usize..=100).cycle();
    let mut roll_count = 0;
    loop {
        let mut move1 = 0;
        for _ in 0..3 {
            move1 += roll.next().unwrap();
            roll_count += 1;
        }
        pos1 = move_pawn(pos1, move1);
        score1 += pos1;

        if score1 >= 1000 {
            break;
        }

        let mut move2 = 0;
        for _ in 0..3 {
            move2 += roll.next().unwrap();
            roll_count += 1;
        }
        pos2 = move_pawn(pos2, move2);
        score2 += pos2;

        if score2 >= 1000 {
            break;
        }
    }

    let loser = min(score1, score2);
    loser * roll_count
}

fn move_pawn(start: usize, distance: usize) -> usize {
    (start + distance - 1) % 10 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(739785, part1(4, 8));
    }

    #[test]
    fn final_part1() {
        assert_eq!(739864900785, part1(4, 5));
    }

    #[test]
    fn test_move() {
        assert_eq!(10, move_pawn(4, 1+2+3));
        assert_eq!(3, move_pawn(8, 4+5+6));
        assert_eq!(4, move_pawn(10, 7+8+9));
    }
}