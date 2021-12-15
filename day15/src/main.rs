use std::{
    collections::{BinaryHeap, HashMap},
    time::Instant,
};

fn main() {
    let start = Instant::now();
    println!(
        "Answer one: {} ({:?})",
        both_parts("input.txt", 1),
        Instant::now() - start
    );

    let start = Instant::now();
    println!(
        "Answer two: {} ({:?})",
        both_parts("input.txt", 5),
        Instant::now() - start
    );
}

fn both_parts(file_name: &str, grid_multiplier: usize) -> u32 {
    let matrix = parse_matrix(file_name);
    let max_y = matrix.len() * grid_multiplier;
    let max_x = matrix[0].len() * grid_multiplier;

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

        for next in neighbors(x, y, (max_x, max_y)) {
            let risk = current_risk + expanded_matrix_value(next.0, next.1, &matrix);
            if !location_risks.contains_key(&next) || &risk < location_risks.get(&next).unwrap() {
                *location_risks.entry(next).or_default() = risk;
                frontier.push(PositionRisk { xy: next, risk });
            }
        }
    }

    least_risk
}

fn expanded_matrix_value(x: usize, y: usize, matrix: &Vec<Vec<u32>>) -> u32 {
    let max_y = matrix.len();
    let max_x = matrix[0].len();
    let x_mod = x % max_x;
    let y_mod = y % max_y;
    let (grid_num_x, grid_num_y) = (x / max_x, y / max_y);
    let add = grid_num_x + grid_num_y;
    let result = matrix[y_mod][x_mod] + add as u32;
    if result < 10 {
        result
    } else {
        (result % 10) + 1
    }
}

fn parse_matrix(file_name: &str) -> Vec<Vec<u32>> {
    helpers::read_lines_panicky(file_name)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbors(x: usize, y: usize, max_xy: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = (x as i32, y as i32);
    let max_x = max_xy.0 as i32;
    let max_y = max_xy.1 as i32;

    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
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
        other
            .risk
            .cmp(&self.risk) // order by risk level first, flipped (lowest first)
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
        assert_eq!(40, both_parts("test_input.txt", 1));
    }

    #[test]
    fn final_part1() {
        assert_eq!(748, both_parts("input.txt", 1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(315, both_parts("test_input.txt", 5));
    }

    #[test]
    fn final_part2() {
        assert_eq!(3045, both_parts("input.txt", 5));
    }

    #[test]
    fn test_grid_expand() {
        let expected = helpers::read_lines_panicky("expanded_grid.txt").collect::<Vec<_>>();
        let matrix = parse_matrix("test_input.txt");
        for y in 0..50 {
            let mut row = String::new();
            for x in 0..50 {
                row.push_str(&format!("{}", expanded_matrix_value(x, y, &matrix)));
            }
            assert_eq!(expected[y], row);
        }
    }
}
