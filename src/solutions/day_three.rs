use crate::utils::parse_input;
use std::simd::u8x64;

const NINES_MASK = u8x64::splat(9);

pub fn solution() {
    let input = parse_input("input/day3.txt").expect("failed to read input");
    for line in input {
        let line = line.expect("failed to read line");
        println!("{}", line);
    }
}
