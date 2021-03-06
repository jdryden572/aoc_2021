use std::collections::HashSet;

type Coord = (u32, u32);

fn main() {
    println!("Answer one: {}", part1("input.txt"));
    println!("Answer two: \r\n{}", part2("input.txt"));
}

fn part1(file_name: &str) -> usize {
    let mut coords: HashSet<Coord> = parse_coordinates(file_name).collect();

    let fold = parse_folds(file_name).next().unwrap();

    match fold {
        Fold::X(x) => fold_x(x, &mut coords),
        Fold::Y(y) => fold_y(y, &mut coords),
    }

    coords.len()
}

fn part2(file_name: &str) -> String {
    let mut coords: HashSet<Coord> = parse_coordinates(file_name).collect();

    for fold in parse_folds(file_name) {
        match fold {
            Fold::X(x) => fold_x(x, &mut coords),
            Fold::Y(y) => fold_y(y, &mut coords),
        }
    }

    let x_max = coords.iter().map(|&(x, _)| x).max().unwrap() as usize;
    let y_max = coords.iter().map(|&(_, y)| y).max().unwrap();

    let empty = " ";
    let filled = "#";
    let mut grid = Vec::new();
    for _ in 0..=y_max {
        grid.push(vec![empty; x_max + 1]);
    }

    for (x, y) in coords {
        let (x, y) = (x as usize, y as usize);
        grid[y][x] = filled;
    }

    let mut code = String::from("\n");
    for row in grid {
        code.push_str(&row.join(""));
        code.push_str("\n");
    }

    code
}

fn fold_x(fold_x: u32, coords: &mut HashSet<Coord>) {
    let temp: Vec<Coord> = coords
        .iter()
        .filter(|&&(x, _)| x > fold_x)
        .cloned()
        .collect();
    for coord in temp {
        let (mut x, y) = coords.take(&coord).unwrap();
        x = fold_x - (x - fold_x);
        coords.insert((x, y));
    }
}

fn fold_y(fold_y: u32, coords: &mut HashSet<Coord>) {
    let temp: Vec<Coord> = coords
        .iter()
        .filter(|&&(_, y)| y > fold_y)
        .cloned()
        .collect();
    for coord in temp {
        let (x, mut y) = coords.take(&coord).unwrap();
        y = fold_y - (y - fold_y);
        coords.insert((x, y));
    }
}

fn parse_coordinates(file_name: &str) -> impl Iterator<Item = Coord> + '_ {
    helpers::read_lines_panicky(file_name)
        .filter(|l| !l.is_empty() && !l.starts_with("fold"))
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
}

fn parse_folds(file_name: &str) -> impl Iterator<Item = Fold> + '_ {
    helpers::read_lines_panicky(file_name)
        .filter(|l| l.starts_with("fold"))
        .map(|l| {
            let axis = l.chars().nth(11).unwrap();
            let (_, num) = l.split_once("=").unwrap();
            let num = num.parse().unwrap();
            match axis {
                'x' => Fold::X(num),
                'y' => Fold::Y(num),
                _ => unreachable!(),
            }
        })
}

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(17, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(751, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        let expected = "
#####
#   #
#   #
#   #
#####
";
        assert_eq!(expected, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        let expected = "
###   ##  #  # ###  #  # #    #  # #   
#  # #  # #  # #  # # #  #    # #  #   
#  # #    #### #  # ##   #    ##   #   
###  # ## #  # ###  # #  #    # #  #   
#    #  # #  # # #  # #  #    # #  #   
#     ### #  # #  # #  # #### #  # ####
";
        assert_eq!(expected, part2("input.txt"));
    }
}
