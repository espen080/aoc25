use crate::utils::parse_input;
use std::simd::u8x64;

const NINES_MASK: u8x64 = u8x64::splat(b'9');

fn parse_line(line: &str) -> u8 {
    let lhs = &line[..line.len() / 2];
    assert!(lhs.len() < 64);
    let lhs_mask: u8x64 = u8x64::load_or_default(lhs.as_bytes());
    println!("lhs mask {:?}", lhs_mask);
    println!("nines mask {:?}", NINES_MASK);
    let res = lhs_mask & NINES_MASK;
    println!("res {:?}", res);
    let rhs = &line[line.len() / 2..];
    assert!(rhs.len() < 64);
    0
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
        assert_eq!(parse_line("123456789"), 17);
    }
}
