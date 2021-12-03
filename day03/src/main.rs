fn main() {
    let answer_one = part1("input.txt");
    println!("Answer one: {}", answer_one);

    let answer_two = part2("input.txt");
    println!("Answer two: {}", answer_two);
}

fn part1(file_name: &str) -> u32 {
    let lines = get_lines(file_name);
    let num_bits = lines[0].len();
    let readings = get_readings(lines);

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..num_bits {
        let sum_at_digit = readings
            .iter()
            .map(|r| get_nth_digit(*r, i))
            .sum::<u32>() as f64;
        
        let half_count = readings.len() as f64 / 2f64;
        if sum_at_digit > half_count {
            // 1 is most common digit
            gamma |= 1 << i;
        } else if sum_at_digit < half_count {
            // 0 is most common digit
            epsilon |= 1 << i;
        } else {
            panic!("Same number of 0s as 1s, the prompt didn't cover this, I'm fucking panicking");
        }
    }
  
    gamma * epsilon
}

fn part2(file_name: &str) -> u32 {
    let lines = get_lines(file_name);
    let num_bits = lines[0].len();
    let readings = get_readings(lines);

    let oxygen = get_rating(readings.clone(), num_bits, true);
    let co2 = get_rating(readings, num_bits, false);

    oxygen * co2
}

fn get_rating(mut readings: Vec<u32>, num_bits: usize, oxygen: bool) -> u32 {
    for i in (0..num_bits).rev() {
        readings = filter_readings(&readings, i, oxygen);
        if readings.len() == 1 {
            break;
        }
    }

    readings[0]
}

fn filter_readings(readings: &[u32], bit: usize, oxygen: bool) -> Vec<u32> {
    let sum = readings.iter().map(|r| get_nth_digit(*r, bit)).sum::<u32>() as f64;
    let half_count = readings.len() as f64 / 2f64;
    let more_ones = sum >= half_count;

    readings
        .iter()
        // this may be the most evil line I've ever written
        .filter(|&r| ((r & 1 << bit) > 0) ^ oxygen ^ more_ones)
        .cloned()
        .collect()
}

fn get_lines(file_name: &str) -> Vec<String> {
    helpers::read_lines_panicky(file_name).collect()
}

fn get_readings(lines: Vec<String>) -> Vec<u32> {
    lines
        .iter()
        .map(|l| u32::from_str_radix(&l, 2).unwrap())
        .collect()
}

fn get_nth_digit(reading: u32, i: usize) -> u32 {
    if (reading & 1 << i) > 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(198, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(4006064, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(230, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(5941884, part2("input.txt"));
    }
}
