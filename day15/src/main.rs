use std::{
    collections::{BinaryHeap, HashMap},
    process::Command,
    time::Instant,
};

use plotters::{prelude::*, coord::Shift};

fn main() {
    let frame_delay = 40;

    let drawing_area = BitMapBackend::gif(
        "images/animated.gif", 
        (500, 500), 
        frame_delay,
    ).unwrap().into_drawing_area();

    let grid_multiplier = 5;
    let matrix = parse_matrix("input.txt");
    let max_y = matrix.len() * grid_multiplier;
    let max_x = matrix[0].len() * grid_multiplier;

    let plotter = Plotter {
        drawing_area,
        matrix: parse_matrix("input.txt"),
        grid_multiplier,
    };

    plotter.draw_matrix();
    plotter.present();

    let start = PositionRisk { xy: (0, 0), risk: 0 };
    let end = (max_x - 1, max_y - 1);

    let mut frontier = BinaryHeap::from_iter([start]);
    let mut location_risks: HashMap<(usize, usize), u32> = HashMap::new();
    location_risks.insert((0, 0), 0);
    let mut came_from = HashMap::new();

    let mut least_risk = u32::MAX;
    let mut steps = 0usize;

    while let Some(PositionRisk { xy: (x, y), risk }) = frontier.pop() {
        steps += 1;

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
                *came_from.entry(next).or_default() = (x, y);
            }
        }

        if steps % 2000 == 0 {
            let start = Instant::now();
            print!("Step {} ", steps);
    
            plotter.draw_matrix();
            plotter.draw_visited(&location_risks, &came_from, &frontier);
            plotter.present();
    
            println!("{:?}", Instant::now() - start);
        }
    }

    println!("Least risk: {}", least_risk);
    println!("Steps: {}", steps);

    frontier.push(PositionRisk { xy: end, risk: least_risk });

    for _ in 0..(3 * (1000 / frame_delay)) {
        plotter.draw_matrix();
        plotter.draw_visited(&location_risks, &came_from, &frontier);
        plotter.present();
    }

    println!("Calling ffmpeg to generate .mp4...");
    Command::new("ffmpeg.exe")
        .arg("-i")
        .arg("images/animated.gif")
        .arg("-movflags")
        .arg("faststart")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-vf")
        .arg("scale=trunc(iw/2)*2:trunc(ih/2)*2")
        .arg("-r")
        .arg("60")
        .arg("images/animated.mp4")
        .output()
        .unwrap();
}

struct Plotter<'a> {
    drawing_area: DrawingArea<BitMapBackend<'a>, Shift>,
    matrix: Vec<Vec<u32>>,
    grid_multiplier: usize,
}

impl<'a> Plotter<'a> {
    fn draw_matrix(&self) {
        self.drawing_area.fill(&WHITE).unwrap();
        for x in 0..self.matrix.len() * self.grid_multiplier {
            for y in 0..self.matrix.len() * self.grid_multiplier {
                let mix = expanded_matrix_value(x, y, &self.matrix) as f64 / 20.0;
                self.drawing_area.draw_pixel((x as i32, y as i32), &BLACK.mix(mix)).unwrap();
            }
        }
    }

    fn draw_visited<'b>(&self, visited: &HashMap<(usize,usize), u32>, came_from: &HashMap<(usize, usize), (usize, usize)>, frontier: &BinaryHeap<PositionRisk>) {
        for (&(x, y), &risk) in visited.iter() {
            self.drawing_area.draw_pixel((x as i32, y as i32), &WHITE).unwrap();
            let mix = risk as f64 / 3049.0;
            self.drawing_area.draw_pixel((x as i32, y as i32), &BLUE.mix(mix)).unwrap();
        }

        for pos in frontier.iter() {
            let mut current = pos.xy;
            while let Some(&(x, y)) = came_from.get(&current) {
                self.drawing_area.draw_pixel((x as i32, y as i32), &BLACK).unwrap();
                current = (x, y);
            }
        }
    }

    fn present(&self) {
        self.drawing_area.present().unwrap();
    }
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

    let mut most_risk = 0;
    let mut least_risk = u32::MAX;
    let mut steps = 0usize;

    while let Some(PositionRisk { xy: (x, y), risk }) = frontier.pop() {
        steps += 1;

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
            if risk > most_risk {
                most_risk = risk;
            }
            if !location_risks.contains_key(&next) || &risk < location_risks.get(&next).unwrap() {
                *location_risks.entry(next).or_default() = risk;
                frontier.push(PositionRisk { xy: next, risk });
            }
        }
    }

    println!("Steps: {}", steps);
    println!("Most risk: {}", most_risk);
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