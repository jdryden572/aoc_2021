use std::{fs::File, error::Error};
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let answer = part1("input.txt")?;
    println!("{}", answer);

    Ok(())
}

fn part1(file_name: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file).lines();
    let mut last_reading = None;
    let mut increased_reading_count = 0;
    for line in reader {
        let reading: i32 = line?.parse()?;
        if let Some(last) = last_reading {
            if reading > last { 
                increased_reading_count += 1;
            }
        }
        last_reading = Some(reading);
    }
    
    Ok(increased_reading_count)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = part1("test_input_1.txt").unwrap();
        assert_eq!(3, answer);
    }
}