use std::cmp;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> i32 {
    get_crab_align_cost(file_name, |c| c)
}

fn part2(file_name: &str) -> i32 {
    get_crab_align_cost(file_name, |c| gauss_sum(c))
}

fn get_crab_align_cost<F: Fn(i32) -> i32>(file_name: &str, cost_fn: F) -> i32 {
    let line = helpers::read_lines_panicky(file_name).next().unwrap();
    let crabs: Vec<i32> = line
        .split(",")
        .map(|p| p.parse().unwrap())
        .collect::<Vec<_>>();

    let &min = crabs.iter().min().unwrap();
    let &max = crabs.iter().max().unwrap();

    let mut min_cost = i32::MAX;
    for position in min..=max {
        let cost_for_position = total_cost(&crabs, position, &cost_fn);
        min_cost = cmp::min(min_cost, cost_for_position);
    }

    min_cost
}

fn total_cost<F: Fn(i32) -> i32>(crabs: &[i32], position: i32, cost_fn: &F) -> i32 {
    crabs
        .iter()
        .map(|c| {
            let distance = (c - position).abs();
            cost_fn(distance)
        })
        .sum()
}

fn gauss_sum(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(37, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(347449, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(98039527, part2("input.txt"));
    }
}
