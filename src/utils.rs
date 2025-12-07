use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse_input(path: &Path) -> io::Result<std::io::Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}
