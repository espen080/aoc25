use crate::utils::parse_input;
use std::simd::{prelude::SimdPartialEq, u8x64};

fn parse_line(line: &str) -> u8 {
    let offset = line.len() / 2;
    let lhs = &line[..offset];
    let rhs = &line[offset..];
    let mut results = Vec::new();
    for i in (0..=9).rev() {
        if let Some(result) = simd_parse(lhs, i + b'0') {
            results.push(result);
        }
        if results.len() == 2 {
            break;
        }
        if let Some(mut result) = simd_parse(rhs, i + b'0') {
            result.index += offset as u8;
            results.push(result);
        }
        if results.len() == 2 {
            break;
        }
    }
    if results[0].count > 1 {
        results[1] = results[0].clone();
    } else if results[0].index > results[1].index {
        results.swap(0, 1);
    }
    format!("{}{}", results[0].value, results[1].value)
        .parse()
        .expect("failed to parse result values")
}

#[derive(Debug, Clone)]
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
    let mut sum: u64 = 0;
    for line in input {
        let line = line.expect("failed to read line");
        sum += parse_line(&line) as u64;
    }
    println!("Total sum {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(parse_line("123456789"), 89, "1");
        assert_eq!(parse_line("987654321"), 98, "2");
        assert_eq!(parse_line("123856799"), 99, "3");
        assert_eq!(parse_line("8989"), 99, "4");
        assert_eq!(parse_line("9988"), 99, "5");
    }
}
