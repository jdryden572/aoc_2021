use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

fn main() {
    let input = "CV-mk,gm-IK,sk-gm,ca-sk,sx-mk,gm-start,sx-ca,kt-sk,ca-VS,kt-ml,kt-ca,mk-IK,end-sx,end-sk,gy-sx,end-ca,ca-ml,gm-CV,sx-kt,start-CV,IK-start,CV-kt,ml-mk,ml-CV,ml-gm,ml-IK";
    let start = Instant::now();
    println!(
        "Answer one: {} (elapsed: {:?})",
        part1(input),
        Instant::now() - start
    );

    let start = Instant::now();
    println!(
        "Answer two: {} (elapsed: {:?})",
        part2(input),
        Instant::now() - start
    );
}

fn part1(input: &str) -> usize {
    let caves = parse_caves_graph(input);

    let mut path_count = 0;
    let mut queue = caves["start"]
        .iter()
        .map(|&c| vec!["start", c])
        .collect::<VecDeque<_>>();

    while let Some(path) = queue.pop_front() {
        let &cave = path.iter().last().unwrap();
        if cave == "end" {
            path_count += 1;
            continue;
        }

        if is_lowercase(cave) && path.iter().filter(|&&p| p == cave).count() > 1 {
            continue;
        }

        for &next in caves[cave].iter() {
            let mut path = clone_vec(&path);
            path.push(next);
            queue.push_back(path);
        }
    }

    path_count
}

fn part2(input: &str) -> usize {
    let caves = parse_caves_graph(input);

    let mut parts = Vec::new();
    count_paths_recursive("start", false, &mut parts, &caves)
}

fn count_paths_recursive<'a>(
    current: &'a str,
    mut seen_twice: bool,
    parts: &mut Vec<&'a str>,
    caves: &HashMap<&str, Vec<&'a str>>,
) -> usize {
    // inspiration from (read: basically stolen from) my AoC Rust hero: AxlLind
    // https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/12.rs

    if current == "end" {
        return 1;
    }

    if is_lowercase(current) && parts.contains(&current) {
        if seen_twice {
            return 0;
        }
        seen_twice = true;
    }

    parts.push(current);
    let count = caves[current]
        .iter()
        .map(|&cave| count_paths_recursive(cave, seen_twice, parts, caves))
        .sum();

    parts.pop();
    count
}

fn parse_caves_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let edges = input.split(",").map(|e| e.split_once("-").unwrap());
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in edges {
        if a != "end" && b != "start" {
            let cave = caves.entry(a).or_default();
            cave.push(b);
        }
        if a != "start" && b != "end" {
            let cave = caves.entry(b).or_default();
            cave.push(a);
        }
    }
    caves
}

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn clone_vec<T>(vec: &Vec<T>) -> Vec<T>
where
    T: Clone,
{
    vec.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_dead_simple() {
        assert_eq!(1, part1("start-A,A-b,b-end"));
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(10, part1("start-A,start-b,A-c,A-b,b-d,A-end,b-end"));
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(
            19,
            part1("dc-end,HN-start,start-kj,dc-start,dc-HN,LN-dc,HN-end,kj-sa,kj-HN,kj-dc")
        );
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(226, part1("fs-end,he-DX,fs-he,start-DX,pj-DX,end-zg,zg-sl,zg-pj,pj-he,RW-he,fs-DX,pj-RW,zg-RW,start-pj,he-WI,zg-he,pj-fs,start-RW"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(4186, part1("CV-mk,gm-IK,sk-gm,ca-sk,sx-mk,gm-start,sx-ca,kt-sk,ca-VS,kt-ml,kt-ca,mk-IK,end-sx,end-sk,gy-sx,end-ca,ca-ml,gm-CV,sx-kt,start-CV,IK-start,CV-kt,ml-mk,ml-CV,ml-gm,ml-IK"));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(36, part2("start-A,start-b,A-c,A-b,b-d,A-end,b-end"));
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(
            103,
            part2("dc-end,HN-start,start-kj,dc-start,dc-HN,LN-dc,HN-end,kj-sa,kj-HN,kj-dc")
        );
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(3509, part2("fs-end,he-DX,fs-he,start-DX,pj-DX,end-zg,zg-sl,zg-pj,pj-he,RW-he,fs-DX,pj-RW,zg-RW,start-pj,he-WI,zg-he,pj-fs,start-RW"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(92111, part2("CV-mk,gm-IK,sk-gm,ca-sk,sx-mk,gm-start,sx-ca,kt-sk,ca-VS,kt-ml,kt-ca,mk-IK,end-sx,end-sk,gy-sx,end-ca,ca-ml,gm-CV,sx-kt,start-CV,IK-start,CV-kt,ml-mk,ml-CV,ml-gm,ml-IK"));
    }
}
