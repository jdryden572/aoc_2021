use std::{fs::File, error::Error};
use std::io::{BufReader, BufRead};

type EasyResult<T> = Result<T, Box<dyn Error>>;

fn main() -> EasyResult<()> {
    let answer = part1("input.txt")?;
    println!("{}", answer);

    let answer_two = part2("input.txt")?;
    println!("{}", answer_two);

    Ok(())
}

fn part1(file_name: &str) -> EasyResult<i32> {
    let readings = parse_ints_from_file(file_name)?;
    let mut last_reading = None;
    let mut increased_reading_count = 0;
    for reading in readings {
        if let Some(last) = last_reading {
            if reading > last { 
                increased_reading_count += 1;
            }
        }
        last_reading = Some(reading);
    }
    
    Ok(increased_reading_count)
}

fn part2(file_name: &str) -> EasyResult<i32> {
    let readings = parse_ints_from_file(file_name)?;

    let mut last_reading = None;
    let mut increased_reading_count = 0;
    for i in 0..readings.len() - 2 {
        let slice = &readings[i..i+3];
        let reading: i32 = slice.iter().sum();
        if let Some(last) = last_reading {
            if reading > last {
                increased_reading_count += 1;
            }
        }
        last_reading = Some(reading);
    }

    Ok(increased_reading_count)
}

fn parse_ints_from_file(file_name: &str) -> EasyResult<Vec<i32>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file).lines();
    let lines: Result<Vec<_>, _> = reader.collect();
    let integers: Vec<i32> = lines?
        .into_iter()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(integers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = part1("test_input_1.txt").unwrap();
        assert_eq!(3, answer);
    }

    #[test]
    fn part1_final() {
        let answer = part1("input.txt").unwrap();
        assert_eq!(1521, answer);
    }

    #[test]
    fn test_part2() {
        let answer = part2("test_input_2.txt").unwrap();
        assert_eq!(5, answer);
    }

    #[test]
    fn part2_final() {
        let answer = part2("input.txt").unwrap();
        assert_eq!(1543, answer);
    }
}