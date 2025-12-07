use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn parse_input(path: &Path) -> io::Result<std::io::Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

fn parse_line(line: &str) {
    let mut chars = line.chars();
    let direction = chars.next().unwrap().to_string();
    let count: i8 = chars.collect::<String>().parse().unwrap();
    println!("{}", direction);
    println!("{}", count);
}

fn main() {
    let input_file = Path::new("input/day1.txt");
    let input = parse_input(input_file).expect("failed to parse input");

    for line in input {
        let line = line.expect("failed to read line");
        parse_line(&line);
        // println!("{}", line);
    }
}
