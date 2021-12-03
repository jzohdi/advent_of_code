pub struct BinaryParser {
    count: usize,
    bit_counts: Vec<usize>,
}

impl BinaryParser {
    pub fn new() -> BinaryParser {
        BinaryParser {
            count: 0,
            bit_counts: vec![],
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        for (idx, bit) in line.chars().enumerate() {
            match bit {
                '0' => self.count_bit(idx, Bit::Zero),
                '1' => self.count_bit(idx, Bit::One),
                _ => panic!("Bit found from parse line is not a zero or one."),
            }
        }
        self.count += 1;
    }

    fn count_bit(&mut self, idx: usize, bit: Bit) -> () {
        if self.bit_counts.len() <= idx {
            self.bit_counts.resize(idx + 1, 0);
        }
        match bit {
            Bit::Zero => self.bit_counts[idx] += 0,
            Bit::One => self.bit_counts[idx] += 1,
        }
    }

    pub fn most_common_bits(&self) -> Vec<u8> {
        self.bit_counts
            .clone()
            .into_iter()
            .map(|count| mcb(count, self.count))
            .collect()
    }

    pub fn lest_common_bits(&self) -> Vec<u8> {
        self.bit_counts
            .clone()
            .into_iter()
            .map(|count| lcb(count, self.count))
            .collect()
    }
}

fn mcb(count: usize, total: usize) -> u8 {
    if count >= (total / 2) {
        1
    } else {
        0
    }
}

fn lcb(count: usize, total: usize) -> u8 {
    if count >= (total / 2) {
        0
    } else {
        1
    }
}

pub struct PowerCalculator {}

impl PowerCalculator {
    pub fn power_consumption(mcb: Vec<u8>, lcb: Vec<u8>) -> usize {
        let gamma = PowerCalculator::calc_gamma(mcb);
        let epsilon = PowerCalculator::calc_epsilon(lcb);
        gamma * epsilon
    }

    fn calc_gamma(bits: Vec<u8>) -> usize {
        PowerCalculator::bits_to_int(bits)
    }

    fn calc_epsilon(bits: Vec<u8>) -> usize {
        PowerCalculator::bits_to_int(bits)
    }

    pub fn bits_to_int(bits: Vec<u8>) -> usize {
        let length = bits.len();
        let mut answer: usize = 0;
        for (idx, bit) in bits.into_iter().enumerate() {
            answer |= (bit as usize) << length - 1 - idx;
        }
        answer
    }
}

enum Bit {
    Zero,
    One,
}
