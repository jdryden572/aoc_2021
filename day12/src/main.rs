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

    let mut path_count = 0;
    let mut queue = caves["start"]
        .iter()
        .map(|&c| vec!["start", c])
        .collect::<VecDeque<_>>();

    while let Some(path) = queue.pop_back() {
        let &cave = path.iter().last().unwrap();
        if cave == "end" {
            path_count += 1;
            //println!("{}", path.join(","));
            continue;
        }

        if is_lowercase(cave) {
            let this_cave_count = path.iter().filter(|&&p| p == cave).count();
            if this_cave_count > 2 {
                continue;
            }

            if this_cave_count == 2 {
                let mut lowercase_counts = HashMap::new();
                for &lowercase in path.iter().filter(|&&p| is_lowercase(p)) {
                    let entry = lowercase_counts.entry(lowercase).or_insert(0);
                    *entry += 1;
                }
                if lowercase_counts
                    .into_iter()
                    .any(|(c, count)| count > 1 && c != cave)
                {
                    continue;
                }
            }
        }

        for &next in caves[cave].iter() {
            let mut path = clone_vec(&path);
            path.push(next);
            queue.push_back(path);
        }
    }

    path_count
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
