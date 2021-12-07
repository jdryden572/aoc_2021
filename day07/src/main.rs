fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> i32 {
    let line = helpers::read_lines_panicky(file_name).next().unwrap();
    let crabs: Vec<i32> = line.split(",").map(|p| p.parse().unwrap()).collect::<Vec<_>>();

    let &min = crabs.iter().min().unwrap();
    let &max  = crabs.iter().max().unwrap();

    let mut min_cost = i32::MAX;
    for i in min..=max {
        min_cost = get_new_min(&crabs, i, min_cost);
    }

    min_cost
}

fn get_new_min(crabs: &[i32], position: i32, prev_min: i32) -> i32 {
    let mut cost = 0;
    for crab in crabs {
        cost += (crab - position).abs();
        if cost > prev_min {
            return prev_min;
        }
    }

    cost
}

fn part2(file_name: &str) -> i32 {
    let line = helpers::read_lines_panicky(file_name).next().unwrap();
    let crabs: Vec<i32> = line.split(",").map(|p| p.parse().unwrap()).collect::<Vec<_>>();

    let &min = crabs.iter().min().unwrap();
    let &max  = crabs.iter().max().unwrap();

    let mut min_cost = i32::MAX;
    for i in min..=max {
        min_cost = get_new_min_gauss(&crabs, i, min_cost);
    }

    min_cost
}

fn get_new_min_gauss(crabs: &[i32], position: i32, prev_min: i32) -> i32 {
    let mut cost = 0;
    for crab in crabs {
        cost += gauss_sum((crab - position).abs());
        if cost > prev_min {
            return prev_min;
        }
    }

    cost
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