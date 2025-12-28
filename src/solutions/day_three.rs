use crate::utils::parse_input;
use std::simd::{prelude::SimdPartialEq, u8x64};

fn parse_line(line: &str) -> u8 {
    let offset = line.len() / 2;
    let lhs = &line[..offset];
    let rhs = &line[offset..];
    for i in (0..=9).rev() {
        if let Some(result) = simd_parse(lhs, i + b'0') {
            println!("{:?}", result);
        }
        if let Some(mut result) = simd_parse(rhs, i + b'0') {
            result.index += offset as u8;
            println!("{:?}", result);
        }
    }
    0
}

#[derive(Debug)]
struct ParseResult {
    pub value: char,
    pub count: u8,
    pub index: u8,
}

fn simd_parse(input: &str, i: u8) -> Option<ParseResult> {
    let target = u8x64::splat(i);
    let chunk = u8x64::load_or_default(input.as_bytes());
    let mask = chunk.simd_eq(target);
    if mask.any() {
        let bits = mask.to_bitmask();
        let index = bits.trailing_zeros() as u8;
        let count = bits.count_ones() as u8;
        return Some(ParseResult {
            value: i as char,
            count,
            index,
        });
    }
    None
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
        assert_eq!(parse_line("123456789"), 89);
        assert_eq!(parse_line("987654321"), 98);
    }
}
