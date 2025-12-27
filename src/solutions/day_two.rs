use std::cmp::min;

use crate::utils::parse_input;

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn new(start: &str, end: &str) -> Self {
        Range {
            min: start.parse().unwrap(),
            max: end.parse().unwrap(),
        }
    }
    fn sum_invalid_ids(&mut self) -> u64 {
        /*
         * Since the young Elf was just doing silly patterns, you can find the
         * invalid IDs by looking for any ID which is made only of some sequence
         * of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and
         * 123123 (123 twice) would all be invalid IDs.
         */
        let mut sum = 0;
        for i in self.min..=self.max {
            let str_i = i.to_string();
            let (lhs, rhs) = str_i.split_at(str_i.len() / 2);
            if lhs == rhs {
                sum += i;
            }
        }
        sum
    }
    fn sum_more_invalid_ids(&mut self) -> u64 {
        /*
         * Now, an ID is invalid if it is made only of some sequence of digits
         * repeated at least twice. So, 12341234 (1234 two times),
         * 123123123 (123 three times), 1212121212 (12 five times),
         * and 1111111 (1 seven times) are all invalid IDs.
         */
        let mut sum = 0;
        let mut is_invalid: bool;
        for i in self.min..=self.max {
            let str_i = i.to_string();
            println!("i = {}", str_i);
            for mask_size in 1..=str_i.len() / 2 {
                is_invalid = true;
                let lhs = &str_i[0..mask_size];
                for splits in 1..=str_i.len() / mask_size {
                    let end = min(mask_size * splits + mask_size, str_i.len());
                    let rhs = &str_i[mask_size * splits..end];
                    println!("{}|{}", lhs, rhs);
                    if lhs != rhs {
                        is_invalid = false;
                        break;
                    }
                }
                if is_invalid {
                    panic!("Invalid id {}", str_i);
                    sum += i;
                    break;
                }
            }
        }
        sum
    }
}

pub fn solution() {
    let mut sum = 0;
    let mut other_sum = 0;
    let input = parse_input("input/day2.txt").expect("failed to read input");
    for line in input {
        let line = line.expect("failed to read line");
        let ranges = line.split(",");
        for range in ranges {
            let (start, end) = range.split_once("-").expect("failed to split range");
            let mut range = Range::new(start, end);
            sum += range.sum_invalid_ids();
            other_sum += range.sum_more_invalid_ids();
        }
    }
    println!("Total sum {}", sum);
    println!("Even more total sum {}", other_sum);
}
