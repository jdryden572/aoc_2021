use std::{time::Instant, cmp::min, collections::HashMap};

fn main() {
    let start = Instant::now();
    println!("Answer one: {} ({:?})", part1(4, 5), Instant::now() - start);
    let start = Instant::now();
    println!("Answer two: {} ({:?})", part2(4, 5), Instant::now() - start);
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

// Brain-computed potential outcomes of three sequential rolls
const STEP_OUTCOMES: [(usize, usize); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

fn part2(pos1: usize, pos2: usize) -> usize {
    let mut states_before_player1: HashMap<GameState, usize> = HashMap::new();
    let mut states_before_player2: HashMap<GameState, usize> = HashMap::new();
    let initial = GameState {
        pos1: pos1 - 1,
        pos2: pos2 - 1,
        ..Default::default()
    };
    states_before_player1.insert(initial, 1usize);

    let mut player1_wins = 0;
    let mut player2_wins = 0;

    for step in 1..50 {
        println!("Step {}", step);

        for (state, universes) in states_before_player1.drain() {
            for (dist, num) in STEP_OUTCOMES {
                let new_pos = (state.pos1 + dist) % 10;
                let new_score = state.score1 + new_pos + 1;
                let new_universes = universes * num;
                if new_score >= 21 {
                    player1_wins += new_universes
                } else {
                    let new_state = GameState::new(new_pos, new_score, state.pos2, state.score2);
                    *states_before_player2.entry(new_state).or_default() += new_universes;
                }
            }
        }

        println!("After player 1: {} wins, {} game states", player1_wins, states_before_player2.len());

        for (state, universes) in states_before_player2.drain() {
            for (dist, num) in STEP_OUTCOMES {
                let new_pos = (state.pos2 + dist) % 10;
                let new_score = state.score2 + new_pos + 1;
                let new_universes = universes * num;
                if new_score >= 21 {
                    player2_wins += new_universes
                } else {
                    let new_state = GameState::new(state.pos1, state.score1, new_pos, new_score);
                    *states_before_player1.entry(new_state).or_default() += new_universes;
                }
            }
        }

        println!("After player 2: {} wins, {} game states", player2_wins, states_before_player1.len());

        if states_before_player1.is_empty() {
            break;
        }
    }

    println!("Player 1 wins {} times", player1_wins);
    println!("Player 2 wins {} times", player2_wins);

    std::cmp::max(player1_wins, player2_wins)
}

#[derive(Hash, PartialEq, Eq, Default, Clone, Copy, Debug)]
struct GameState {
    pos1: usize,
    pos2: usize,
    score1: usize,
    score2: usize,
}

impl GameState {
    fn new(pos1: usize, score1: usize, pos2: usize, score2: usize) -> Self {
        Self {
            pos1,
            score1,
            pos2,
            score2
        }
    }
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
        assert_eq!(864900, part1(4, 5));
    }

    #[test]
    fn test_part2() {
        assert_eq!(444356092776315, part2(4, 8));
    }

    #[test]
    fn final_part2() {
        assert_eq!(575111835924670, part2(4, 5));
    }

    #[test]
    fn test_move() {
        assert_eq!(10, move_pawn(4, 1+2+3));
        assert_eq!(3, move_pawn(8, 4+5+6));
        assert_eq!(4, move_pawn(10, 7+8+9));
    }
}