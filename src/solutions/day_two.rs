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
            for mask_size in 1..=str_i.len() / 2 {
                if str_i.len() % mask_size != 0 {
                    // The pieces are not the same size, no match possible
                    continue;
                }
                is_invalid = true;
                let lhs = &str_i[0..mask_size];
                for splits in 1..str_i.len() / mask_size {
                    let end = min(mask_size * splits + mask_size, str_i.len());
                    let rhs = &str_i[mask_size * splits..end];
                    if lhs != rhs {
                        is_invalid = false;
                        break;
                    }
                }
                if is_invalid {
                    sum += i;
                    break;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_more() {
        assert_eq!(Range::new("11", "22").sum_more_invalid_ids(), 33);
        assert_eq!(Range::new("95", "115").sum_more_invalid_ids(), 210);
        assert_eq!(Range::new("998", "1012").sum_more_invalid_ids(), 2009);
        assert_eq!(
            Range::new("1188511880", "1188511890").sum_more_invalid_ids(),
            1188511885
        );
        assert_eq!(
            Range::new("222220", "222224").sum_more_invalid_ids(),
            222222
        );
        assert_eq!(Range::new("1698522", "1698528").sum_more_invalid_ids(), 0);
        assert_eq!(
            Range::new("446443", "446449").sum_more_invalid_ids(),
            446446
        );
        assert_eq!(
            Range::new("38593856", "38593862").sum_more_invalid_ids(),
            38593859
        );
        assert_eq!(
            Range::new("565653", "565659").sum_more_invalid_ids(),
            565656
        );
        assert_eq!(
            Range::new("824824821", "824824827").sum_more_invalid_ids(),
            824824824
        );
        assert_eq!(
            Range::new("2121212118", "2121212124").sum_more_invalid_ids(),
            2121212121
        );
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
