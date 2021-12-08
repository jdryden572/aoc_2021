use core::panic;
use std::collections::HashMap;

/*
 AAAA
B    C
B    C
 DDDD
E    F
E    F
 GGGG
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Seg {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

static N_0: &[Seg] = &[Seg::A, Seg::B, Seg::C, Seg::E, Seg::F, Seg::G];
//static N_1: &[Seg] = &[Seg::C, Seg::F];
static N_2: &[Seg] = &[Seg::A, Seg::C, Seg::D, Seg::E, Seg::G];
static N_3: &[Seg] = &[Seg::A, Seg::C, Seg::D, Seg::F, Seg::G];
//static N_4: &[Seg] = &[Seg::B, Seg::C, Seg::D, Seg::F];
//static N_5: &[Seg] = &[Seg::A, Seg::B, Seg::D, Seg::F, Seg::G];
static N_6: &[Seg] = &[Seg::A, Seg::B, Seg::D, Seg::E, Seg::F, Seg::G];
//static N_7: &[Seg] = &[Seg::A, Seg::C, Seg::F];
//static N_8: &[Seg] = &[Seg::A, Seg::B, Seg::C, Seg::D, Seg::E, Seg::F, Seg::G];
//static N_9: &[Seg] = &[Seg::A, Seg::B, Seg::C, Seg::D, Seg::F, Seg::G];

pub struct Decoder {
    mappings: HashMap<char, Seg>,
}

impl Decoder {
    pub fn build(inputs: Vec<String>) -> Self {
        let mut mappings = HashMap::new();
        let mut inputs: Vec<Vec<char>> = inputs.into_iter().map(|i| i.chars().collect()).collect();
        inputs.sort_by(|x, y| x.len().cmp(&y.len()));

        let one = &inputs[0];
        let seven = &inputs[1];
        let four = &inputs[2];
        let eight = &inputs[9];

        let two_three_five = &inputs[3..6];
        let zero_six_nine = &inputs[6..9];
        
        let &seg_a = seven.iter().filter(|c| !one.contains(*c)).next().unwrap();
        //println!("'{}' = SegA", seg_a);
        mappings.insert(seg_a, Seg::A);

        let combined = || two_three_five.iter().chain(zero_six_nine.iter());
        let &seg_g = eight.iter().filter(|&c| c != &seg_a && combined().filter(|d| d.contains(c)).count() == 6).next().unwrap();
        //println!("'{}' = SegG", seg_g);
        mappings.insert(seg_g, Seg::G);

        let &seg_f = one.iter().filter(|&c| zero_six_nine.iter().filter(|d| d.contains(c)).count() == 3).next().unwrap();
        //println!("'{}' = SegF", seg_f);
        mappings.insert(seg_f, Seg::F);

        let &seg_c = one.iter().filter(|&c| c != &seg_f).next().unwrap();
        //println!("'{}' = SegC", seg_c);
        mappings.insert(seg_c, Seg::C);

        let &seg_d = four.iter().filter(|&c| c != &seg_c && zero_six_nine.iter().filter(|d| d.contains(c)).count() == 2).next().unwrap();
        //println!("'{}' = SegD", seg_d);
        mappings.insert(seg_d, Seg::D);

        let &seg_b = four.iter().filter(|&c| c != &seg_d && c != &seg_c && c != &seg_f).next().unwrap();
        //println!("'{}' = SegB", seg_b);
        mappings.insert(seg_b, Seg::B);

        let &seg_e = eight.iter().filter(|c| !mappings.contains_key(c)).next().unwrap();
        //println!("'{}' = SegE", seg_e);
        mappings.insert(seg_e, Seg::E);

        Self { mappings }
    }

    pub fn decode(&self, input: &str) -> usize {
        let mut mapped = input.chars().map(|c| self.mappings.get(&c).copied().unwrap()).collect::<Vec<_>>();
        mapped.sort();

        match mapped.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                if &mapped == N_2 {
                    2
                } else if &mapped == N_3 {
                    3
                } else {
                    5
                }
            },
            6 => {
                if &mapped == N_0 {
                    0
                } else if &mapped == N_6 {
                    6
                } else {
                    9
                }
            },
            7 => 8,
            _ => panic!("Shit!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let inputs: Vec<String> = vec!["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"].into_iter().map(String::from).collect();
        let mut mappings = HashMap::new();
        mappings.insert('a', Seg::C);
        mappings.insert('b', Seg::F);
        mappings.insert('c', Seg::G);
        mappings.insert('d', Seg::A);
        mappings.insert('e', Seg::B);
        mappings.insert('f', Seg::D);
        mappings.insert('g', Seg::E);
        assert_eq!(mappings, Decoder::build(inputs).mappings);
    }

    #[test]
    fn test_decode() {
        let mut mappings = HashMap::new();
        mappings.insert('a', Seg::C);
        mappings.insert('b', Seg::F);
        mappings.insert('c', Seg::G);
        mappings.insert('d', Seg::A);
        mappings.insert('e', Seg::B);
        mappings.insert('f', Seg::D);
        mappings.insert('g', Seg::E);

        let decoder = Decoder { mappings };
        assert_eq!(5, decoder.decode("cdfeb"));
        assert_eq!(3, decoder.decode("fcadb"));
        assert_eq!(3, decoder.decode("cdbaf"));
    }
}