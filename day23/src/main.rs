mod display;

use std::{
    cmp::{min, Ordering},
    collections::{BinaryHeap, HashMap, VecDeque},
    time::Instant,
};

/*
#############
#...........#
###A#D#B#D###
  #B#C#A#C#
  #########
*/
const INPUT: &str = "ADBDBCAC";

fn main() {
    let start = Instant::now();
    println!(
        "Answer one: {} ({:?})",
        part1(INPUT),
        Instant::now() - start
    );
}

fn part1(input: &str) -> usize {
    let burrow = Burrow::parse(input);

    let mut lowest_cost = usize::MAX;
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        burrow: burrow.clone(),
    });
    let mut costs: HashMap<Burrow, usize> = HashMap::new();
    costs.insert(burrow, 0);

    //let mut last_cost = 0;

    while let Some(State { cost, burrow }) = heap.pop() {
        // if cost > last_cost + 50 {
        //     last_cost = cost;
        //     println!("Cost: {} Heap: {} Seen: {}", cost, heap.len(), costs.len());
        // }

        if burrow.is_complete() {
            lowest_cost = min(lowest_cost, cost);
            //println!("Found solution! {}", lowest_cost);
            //println!("{}", burrow);
            continue;
        }

        let &lowest_burrow_cost = costs.get(&burrow).unwrap();
        if cost > lowest_burrow_cost {
            continue;
        }

        for (new_burrow, add_cost) in PodMoves::new(&burrow) {
            let cost = cost + add_cost;
            //println!("Cost: {}", cost);
            //println!("{}", new_burrow);
            let burrow = new_burrow.clone();
            if cost < lowest_cost {
                let entry = costs.entry(new_burrow).or_insert(usize::MAX);
                if cost < *entry {
                    *entry = cost;
                    heap.push(State { cost, burrow })
                }
            }
        }
    }
    lowest_cost
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    burrow: Burrow,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.burrow.cmp(&other.burrow))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*
 0  1  2  3  4  5  6  7  8  9 10
      11    12    13    14
      15    16    17    18
*/
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Burrow {
    pods: [u8; 8],
}

impl Burrow {
    fn parse(input: &str) -> Self {
        let mut pods = Vec::new();
        for kind in ['A', 'B', 'C', 'D'] {
            pods.extend(input.match_indices(kind).map(|(i, _)| i as u8 + 11))
        }
        let pods = pods.try_into().unwrap();
        Self { pods }
    }

    fn occupied(&self, position: u8) -> bool {
        self.pods.contains(&position)
    }

    fn is_in_room(&self, i: usize) -> bool {
        self.pods[i] > 10
    }

    fn is_complete(&self) -> bool {
        (0..8).all(|i| self.is_pod_home(i))
    }

    fn is_pod_home(&self, i: usize) -> bool {
        let slot = i as u8 / 2;
        let position = self.pods[i];
        if position == slot + 15 {
            // if we're in the deeper spot, we're home
            return true;
        }
        if position == slot + 11 {
            // we're in the outer spot, only home if the other is home too...
            let other_idx = Self::pair_idx(i);
            return self.is_pod_home(other_idx);
        }

        false
    }

    fn pair_idx(i: usize) -> usize {
        if i % 2 == 0 {
            i + 1
        } else {
            i - 1
        }
    }

    fn entrance(i: usize) -> u8 {
        (i / 2 + 11) as u8
    }

    fn steps_from(mut a: u8, mut b: u8) -> u8 {
        let mut dist = 0;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        let outside = match b {
            11 | 15 => 2,
            12 | 16 => 4,
            13 | 17 => 6,
            14 | 18 => 8,
            _ => unreachable!(),
        };
        dist += (a as i16 - outside).abs() as u8;
        if b > 14 {
            dist += 2;
        } else {
            dist += 1;
        }

        dist
    }

    fn cost(i: usize) -> usize {
        match i / 2 {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            _ => unreachable!(),
        }
    }

    fn move_pod(&self, i: usize, new_pos: u8) -> Self {
        let mut new = self.clone();
        new.pods[i] = new_pos;
        new
    }
}

lazy_static::lazy_static! {
    static ref OUT_MOVES: HashMap<u8, Vec<Vec<u8>>> = {
        let mut m = HashMap::new();
        m.insert(11, vec![vec![1, 0], vec![3, 5, 7, 9, 10]]);
        m.insert(12, vec![vec![3, 1, 0], vec![5, 7, 9, 10]]);
        m.insert(13, vec![vec![5, 3, 1, 0], vec![7, 9, 10]]);
        m.insert(14, vec![vec![7, 5, 3, 1, 0], vec![9, 10]]);
        m
    };
}

struct PodMoves<'b> {
    burrow: &'b Burrow,
    current_idx: usize,
    queue: VecDeque<u8>,
}

impl<'b> PodMoves<'b> {
    fn new(burrow: &'b Burrow) -> Self {
        let mut s = Self {
            burrow,
            current_idx: 0,
            queue: VecDeque::new(),
        };
        s.add_possible_positions();
        s
    }

    fn add_possible_positions(&mut self) {
        if self.burrow.is_pod_home(self.current_idx) {
            return;
        }

        let mut position = self.burrow.pods[self.current_idx];
        if self.burrow.is_in_room(self.current_idx) {
            // in a room
            if position > 14 {
                if self.burrow.occupied(position - 4) {
                    // stuck in room behind another pod
                    return;
                }

                position -= 4;
            }

            let directions = OUT_MOVES.get(&position).unwrap();
            for direction in directions {
                let new_positions = direction
                    .into_iter()
                    .take_while(|&&p| !self.burrow.occupied(p));
                self.queue.extend(new_positions);
            }
        } else {
            // in the hallway
            let entrance = Burrow::entrance(self.current_idx);

            let direction = OUT_MOVES
                .get(&entrance)
                .unwrap()
                .iter()
                .find(|&d| d.contains(&position))
                .unwrap();
            if direction
                .iter()
                .rev()
                .skip_while(|&&p| p != position)
                .skip(1)
                .any(|&p| self.burrow.occupied(p))
            {
                // path to entrance is blocked, can't move
                return;
            }

            if !self.burrow.occupied(entrance) {
                // entrance is vacant
                let other_idx = Burrow::pair_idx(self.current_idx);
                if self.burrow.is_pod_home(other_idx) {
                    // pair is home, go to entrance
                    self.queue.push_back(entrance);
                } else if !self.burrow.occupied(entrance + 4) {
                    // bottom spot is open, go there
                    self.queue.push_back(entrance + 4);
                }
            }
        }
    }

    fn cost_for_move(&self, new_pos: u8) -> usize {
        let pos = self.burrow.pods[self.current_idx];
        let dist = Burrow::steps_from(pos, new_pos) as usize;
        let cost = Burrow::cost(self.current_idx);
        dist * cost
    }
}

impl<'b> Iterator for PodMoves<'b> {
    type Item = (Burrow, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx > 7 {
            return None;
        }

        if let Some(new_pos) = self.queue.pop_front() {
            return Some((
                self.burrow.move_pod(self.current_idx, new_pos),
                self.cost_for_move(new_pos),
            ));
        }

        while self.current_idx < 7 && self.queue.is_empty() {
            self.current_idx += 1;
            self.add_possible_positions();
            if let Some(new_pos) = self.queue.pop_front() {
                return Some((
                    self.burrow.move_pod(self.current_idx, new_pos),
                    self.cost_for_move(new_pos),
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #########
    */
    const TEST_INPUT: &str = "BCBDADCA";

    #[test]
    fn test_part1() {
        assert_eq!(12521, part1(TEST_INPUT));
    }

    #[test]
    fn final_part1() {
        assert_eq!(12240, part1(INPUT));
    }

    #[test]
    fn test_is_complete() {
        let burrow = Burrow::parse("ABCDABCD");
        assert!(burrow.is_complete());

        let burrow = Burrow::parse("BCBDADCA");
        assert!(!burrow.is_complete());
    }

    #[test]
    fn test_moves() {
        let burrow = Burrow::parse("BCBDADCA");
        let moves = PodMoves::new(&burrow);
        let all_moves = moves.collect::<Vec<_>>();
        assert_eq!(28, all_moves.len());
    }

    #[test]
    fn test_steps_from() {
        assert_eq!(3, Burrow::steps_from(0, 11));
        assert_eq!(4, Burrow::steps_from(0, 15));

        assert_eq!(3, Burrow::steps_from(10, 14));
        assert_eq!(4, Burrow::steps_from(10, 18));

        assert_eq!(9, Burrow::steps_from(0, 14));
        assert_eq!(10, Burrow::steps_from(0, 18));
    }

    #[test]
    fn test_last_move() {
        let pods = [9u8, 15, 12, 16, 13, 17, 14, 18];
        let burrow = Burrow { pods };
        println!("{}", burrow);
        let moves = PodMoves::new(&burrow);
        for (b, cost) in moves {
            println!("Cost: {}", cost);
            println!("{}", b);
        }
    }
}
