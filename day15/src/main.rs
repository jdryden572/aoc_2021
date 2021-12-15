use std::collections::{BinaryHeap, HashMap};

fn main() {
    println!("Answer one: {}", part1("input.txt"));
}

fn part1(file_name: &str) -> u32 {
    let matrix = parse_matrix(file_name);
    let max_y = matrix.len();
    let max_x = matrix[0].len();

    let start = PositionRisk { xy: (0, 0), risk: 0 };
    let end = (max_x - 1, max_y - 1);

    let mut frontier = BinaryHeap::from_iter([start]);
    let mut location_risks: HashMap<(usize, usize), u32> = HashMap::new();
    location_risks.insert((0, 0), 0);

    let mut least_risk = u32::MAX;
    
    while let Some(PositionRisk { xy: (x, y), risk }) = frontier.pop() {
        if (x, y) == end {
            least_risk = risk;
            break;
        }
        
        let &current_risk = location_risks.get(&(x, y)).unwrap();
        if risk > current_risk {
            continue;
        }

        for next in neighbors(x, y, &matrix) {
            let risk = current_risk + matrix[next.1][next.0];
            if !location_risks.contains_key(&next) || &risk < location_risks.get(&next).unwrap() {
                * location_risks.entry(next).or_default() = risk;
                frontier.push(PositionRisk { xy: next, risk });
            }
        }
    }

    least_risk
}

fn parse_matrix(file_name: &str) -> Vec<Vec<u32>> {
    helpers::read_lines_panicky(file_name)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbors(x: usize, y: usize, matrix: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let (x, y) = (x as i32, y as i32);
    let max_y = matrix.len() as i32;
    let max_x = matrix[0].len() as i32;

    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
    ]
    .iter()
    .filter(|&&(x, y)| x >= 0 && x < max_x && y >= 0 && y < max_y)
    .map(|&(x, y)| (x as usize, y as usize))
    .collect()
}

#[derive(PartialEq, Eq)]
struct PositionRisk {
    xy: (usize, usize),
    risk: u32,
}

impl Ord for PositionRisk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk) // order by risk level first, flipped
            .then_with(|| self.xy.cmp(&other.xy))
    }
}

impl PartialOrd for PositionRisk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(40, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(748, part1("input.txt"));
    }
}