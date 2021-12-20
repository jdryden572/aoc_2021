use std::{time::Instant};

fn main() {
    let start = Instant::now();
    println!("Answer one: {} ({:?})", part1("input.txt"), Instant::now() - start);
}

fn part1(file_name: &str) -> usize {
    let (algo, mut image) = parse_input(file_name);

    for step in 0..2 {
        let default = if algo[0] == 1 {
            // have to toggle the "infinite" part of the image every step
            if step % 2 == 0 { 0 } else { 1 }
        } else {
            0
        };

        let max_x = image[0].len() as i32 + 1;
        let max_y = image.len() as i32 + 1;
        let mut new_image = Vec::with_capacity(image.len() + 2);

        for y in -1..max_y {
            let mut row = Vec::with_capacity(image[0].len() + 2);
            for x in -1..max_x {
                row.push(enhance_pixel(x, y, &image, &algo, default))
            }
            new_image.push(row);
        }

        // for y in 0..new_image.len() {
        //     for x in 0..new_image[0].len() {
        //         let c = if new_image[y][x] > 0 { '#' } else { '.' };
        //         print!("{}", c);
        //     }
        //     println!()
        // }

        image = new_image;
    }

    image.into_iter().map(|row| row.iter().filter(|&&b| b == 1).count()).sum()
}

fn enhance_pixel(x: i32, y: i32, image: &[Vec<u8>], algo: &[u8], default: u8) -> u8 {
    let window_value = pixel_window_value(x, y, image, default);
    algo[window_value]
}

fn pixel_window_value(x: i32, y: i32, image: &[Vec<u8>], default: u8) -> usize {
    let max_x = image[0].len() as i32;
    let max_y = image.len() as i32;

    let mut value = 0usize;
    let mut position = 9usize;
    for j in -1..2 {
        let out_of_bounds = y + j < 0 || y + j >= max_y;
        for i in -1..2 {
            let out_of_bounds = out_of_bounds || x + i  < 0 || x + i >= max_x;
            let bit = if out_of_bounds {
                default
            } else {
                image[(y + j) as usize][(x + i) as usize]
            };

            position -= 1;
            if bit > 0 {
                value |= 1 << position;
            }
        }
    }

    value
}

fn parse_input(file_name: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let mut lines = helpers::read_lines_panicky(file_name);
    let algo: Vec<u8> = zeroes_and_ones(lines.next().unwrap());
    assert_eq!(512, algo.len());

    let mut image: Vec<Vec<u8>> = Vec::new();
    for line in lines.skip(1) {
        image.push(zeroes_and_ones(line));
    }

    (algo, image)
}

fn zeroes_and_ones(s: String) -> Vec<u8> {
    s.bytes()
        .map(|b| if b == '#' as u8 { 1u8 } else { 0u8 })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(35, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(5680, part1("input.txt"));
    }

    #[test]
    fn test_pixel_window() {
        let image = parse_input("test_input.txt").1;

        assert_eq!(1, pixel_window_value(-1, -1, &image, 0));
        assert_eq!(34, pixel_window_value(2, 2, &image, 0));
        assert_eq!(256, pixel_window_value(5, 5, &image, 0));
    }
}