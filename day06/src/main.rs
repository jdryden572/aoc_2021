fn main() {
    println!("Answer one: {}", both_parts("input.txt", 80));
    println!("Answer two: {}", both_parts("input.txt", 256));
}

fn both_parts(file_name: &str, days: usize) -> usize {
    let mut population = Population::default();
    for fish in get_all_fish(file_name) {
        population.add(fish);
    }

    for _ in 0..days {
        population.simulate_day();
    }

    population.total()
}

fn get_all_fish(file_name: &str) -> Vec<usize> {
    let line = helpers::read_lines_panicky(file_name).next().unwrap();
    line.split(",").map(|f| f.parse().unwrap()).collect()
}

#[derive(Default)]
struct Population {
    buckets: [usize; 9],
}

impl Population {
    fn add(&mut self, fish: usize) {
        assert!(fish <= 8);
        self.buckets[fish] += 1;
    }

    fn simulate_day(&mut self) {
        let spawning_fish = self.buckets[0];
        for i in 1..9 {
            self.buckets[i - 1] = self.buckets[i];
        }
        self.buckets[6] += spawning_fish; // adults ready to spawn again in a week
        self.buckets[8] = spawning_fish; // newly spawned fish
    }

    fn total(&self) -> usize {
        self.buckets.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5934, both_parts("test_input.txt", 80));
    }

    #[test]
    fn final_part1() {
        assert_eq!(350605, both_parts("input.txt", 80));
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539, both_parts("test_input.txt", 256));
    }

    #[test]
    fn final_part2() {
        assert_eq!(1592778185024, both_parts("input.txt", 256));
    }
}
