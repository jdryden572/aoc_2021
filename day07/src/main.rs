use std::collections::HashMap;

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer one: {}", part2("input.txt"));
}

fn part1(file_name: &str) -> i32 {
    let line = helpers::read_lines_panicky(file_name).next().unwrap();
    let positions: Vec<i32> = dbg!(line.split(",").map(|p| p.parse().unwrap()).collect::<Vec<_>>());

    let min = *(positions.iter().min().unwrap());
    let max  = *(positions.iter().max().unwrap());

    let mut costs = HashMap::new();
    for i in min..max + 1 {
        //println!("{}", i);
        let mut cost = 0;
        for pos in &positions {
            cost += (pos - i).abs();
            //println!("--> {}", cost);
        }
        costs.insert(i, cost);
    }

    let min = costs.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
    *dbg!(min).1
}

fn part2(file_name: &str) -> i64 {
    let line = helpers::read_lines_panicky(file_name).next().unwrap();
    let positions: Vec<i64> = dbg!(line.split(",").map(|p| p.parse().unwrap()).collect::<Vec<_>>());

    let min = *(positions.iter().min().unwrap());
    let max  = *(positions.iter().max().unwrap());

    let mut costs = HashMap::new();
    for i in min..max + 1 {
        //println!("{}", i);
        let mut cost = 0;
        for pos in &positions {
            cost += calc((pos - i).abs());
            //println!("--> {}", cost);
        }
        costs.insert(i, cost);
    }

    let min = costs.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
    *dbg!(min).1
}

fn calc(num: i64) -> i64 {
    match num {
        0  => 1,
        1.. => (1..num+1).sum(),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(37, part1("test_input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, part2("test_input.txt"));
    }
}