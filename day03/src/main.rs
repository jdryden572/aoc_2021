fn main() {
    let answer_one = part1("input.txt", 12);
    println!("Answer one: {}", answer_one);

    let answer_two = part2("input.txt", 12);
    println!("Answer two: {}", answer_two);
}

fn part1(file_name: &str, num_bits: usize) -> usize {
    let readings = parse_input(file_name).collect();
    let (sums, count) = get_bit_counts(&readings, num_bits);

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for i in 0..num_bits {
        let bit_set = sums[i] > count / 2;
        if bit_set {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    gamma * epsilon
}

fn part2(file_name: &str, num_bits: usize) -> usize {
    let all_readings: Vec<usize> = parse_input(file_name).collect();

    let oxygen = find_oxygen_reading(all_readings.clone(), num_bits);
    let co2 = find_co2_reading(all_readings, num_bits);

    oxygen * co2
}

fn find_oxygen_reading(mut readings: Vec<usize>, num_bits: usize) -> usize {
    for i in (0..num_bits).rev() {
        readings = filter_oxygen_by_bit(readings, i);
        if readings.len() == 1 {
            break;
        }
    }   

    readings[0]
}

fn filter_oxygen_by_bit(readings: Vec<usize>, bit: usize) -> Vec<usize> {
    let (sums, count) = get_bit_counts(&readings, bit + 1);

    let iter = readings.into_iter();

    if sums[bit] as f64 >= (count as f64) / 2f64 {
        // most common bit is 1 or tie
        iter.filter(|r| (r & 1 << bit) > 0).collect()
    } else {
        // most common bit is 0
        iter.filter(|r| (r & 1 << bit) == 0).collect()
    }
}

fn find_co2_reading(mut readings: Vec<usize>, num_bits: usize) -> usize {
    for i in (0..num_bits).rev() {
        readings = filter_co2_by_bit(readings, i);
        if readings.len() == 1 {
            break;
        }
    }   

    readings[0]
}

fn filter_co2_by_bit(readings: Vec<usize>, bit: usize) -> Vec<usize> {
    let (sums, count) = get_bit_counts(&readings, bit + 1);

    let iter = readings.into_iter();

    if (sums[bit] as f64) < (count as f64) / 2f64 {
        // least common bit is 1
        iter.filter(|r| (r & 1 << bit) > 0).collect()
    } else {
        // least common bit is 0 or tie
        iter.filter(|r| (r & 1 << bit) == 0).collect()
    }
}

fn parse_input(file_name: &str) -> impl Iterator<Item = usize> + '_ {
    helpers::read_lines_panicky(file_name)
        .map(|l| usize::from_str_radix(&l, 2))
        .map(Result::unwrap)
}

fn get_bit_counts(readings: &Vec<usize>, num_bits: usize) -> ([usize; 12], usize) {
    let mut sums: [usize; 12] = [0; 12];
    let mut count: usize = 0;

    for reading in readings.into_iter() {
        count += 1;
        for i in 0..num_bits {
            let bit_set = (reading & 1 << i) > 0;
            if bit_set {
                sums[i] += 1;
            }
        }
    }

    (sums, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(198, part1("test_input.txt", 5));
    }

    #[test]
    fn final_part1() {
        assert_eq!(4006064, part1("input.txt", 12));
    }

    #[test]
    fn test_part2() {
        assert_eq!(230, part2("test_input.txt", 5));
    }

    #[test]
    fn final_part2() {
        assert_eq!(5941884, part2("input.txt", 12));
    }
}