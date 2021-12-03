use std::cmp::Ordering;

fn main() {
    let answer_one = part1("input.txt");
    println!("Answer one: {}", answer_one);

    let answer_two = part2("input.txt");
    println!("Answer two: {}", answer_two);
}

fn part1(file_name: &str) -> u32 {
    let (readings, num_bits) = get_readings_and_num_bits(file_name);

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..num_bits {
        match DigitInfo::from_bit_index(&readings, i) {
            DigitInfo::MoreOnes => gamma |= 1 << i,
            DigitInfo::MoreZeros => epsilon |= 1 << i,
            DigitInfo::Same => panic!(
                "Same number of 0s as 1s, the prompt didn't cover this, I'm fucking panicking"
            ),
        }
    }

    gamma * epsilon
}

fn part2(file_name: &str) -> u32 {
    let (readings, num_bits) = get_readings_and_num_bits(file_name);

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

    assert_eq!(1, readings.len());
    readings[0]
}

fn filter_readings(readings: &[u32], bit: usize, oxygen: bool) -> Vec<u32> {
    let digit_info = DigitInfo::from_bit_index(readings, bit);
    let more_ones = digit_info != DigitInfo::MoreZeros;

    readings
        .iter()
        // this may be the most evil line I've ever written
        .filter(|&r| (get_nth_digit(*r, bit) > 0) ^ oxygen ^ more_ones)
        .cloned()
        .collect()
}

#[derive(PartialEq)]
enum DigitInfo {
    MoreOnes,
    MoreZeros,
    Same,
}

impl DigitInfo {
    pub fn from_bit_index(readings: &[u32], bit: usize) -> Self {
        let sum = readings.iter().map(|r| get_nth_digit(*r, bit)).sum::<u32>() as f64;
        let half_count = readings.len() as f64 / 2f64;
        
        match sum.partial_cmp(&half_count) {
            Some(Ordering::Greater) => DigitInfo::MoreOnes,
            Some(Ordering::Less) => DigitInfo::MoreZeros,
            Some(Ordering::Equal) => DigitInfo::Same,
            None => panic!("Oh shit, where'd the NaN come from?"),
        }
    }
}

fn get_readings_and_num_bits(file_name: &str) -> (Vec<u32>, usize) {
    let lines: Vec<_> = helpers::read_lines_panicky(file_name).collect();
    let readings = lines
        .iter()
        .map(|l| u32::from_str_radix(&l, 2).unwrap())
        .collect();

    (readings, lines[0].len())
}

fn get_nth_digit(reading: u32, i: usize) -> u32 {
    reading >> i & 1
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
