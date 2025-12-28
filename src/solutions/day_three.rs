use crate::utils::parse_input;
use std::simd::{prelude::SimdPartialEq, u8x64};

const NINES_MASK: u8x64 = u8x64::splat(b'9');

fn parse_line(line: &str) -> u8 {
    let lhs = &line[..line.len() / 2];
    let rhs = &line[line.len() / 2..];
    for i in 9..0 {
        println!("{}", i);
    }
    let lhs_mask: u8x64 = u8x64::load_or_default(lhs.as_bytes());
    let res = lhs_mask.simd_eq(NINES_MASK);
    let count = res.to_bitmask().count_ones() as u8;
    count
}

pub fn solution() {
    let input = parse_input("input/day3.txt").expect("failed to read input");
    let mut sum = 0;
    for line in input {
        let line = line.expect("failed to read line");
        sum += parse_line(&line);
    }
    println!("Total sum {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(parse_line("193456789"), 17);
    }
}
