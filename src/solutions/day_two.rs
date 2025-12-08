use crate::utils::parse_input;

struct Range {
    start: String,
    end: String,
    min: u64,
    max: u64,
}

impl Range {
    fn new(start: &str, end: &str) -> Self {
        Range {
            start: String::from(start),
            end: String::from(end),
            min: start.parse().unwrap(),
            max: end.parse().unwrap(),
        }
    }
    fn find_invalid_ids(&mut self) {
        println!("{} {}", self.start, self.end);
        for i in self.min..self.max {
            println!("{}", i);
        }
    }
}

pub fn solution() {
    let input = parse_input("input/day2.txt").expect("failed to read input");
    for line in input {
        let line = line.expect("failed to read line");
        let ranges = line.split(",");
        for range in ranges {
            let (start, end) = range.split_once("-").expect("failed to split range");
            let mut range = Range::new(start, end);
            range.find_invalid_ids();
        }
    }
}
