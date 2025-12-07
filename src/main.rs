use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Rotation {
    pub direction: char,
    pub count: i32,
}

impl Rotation {
    fn new(dir: char, count: i32) -> Self {
        Rotation {
            direction: dir,
            count,
        }
    }
}

#[derive(Debug)]
struct Lock {
    pub position: i32,
    count: HashMap<i32, i32>,
}

impl Lock {
    fn new() -> Self {
        Lock {
            position: 50,
            count: HashMap::new(),
        }
    }

    fn turn(&mut self, rotation: &Rotation) -> Result<(), &'static str> {
        match rotation.direction {
            'L' => {
                self.position -= rotation.count;
                if self.position < 0 {
                    println!("{:?} - {:?}", self.position, rotation);
                    self.position += 99;
                }
                *self.count.entry(self.position).or_insert(0) += 1;
                Ok(())
            }
            'R' => {
                self.position += rotation.count;
                if self.position > 99 {
                    self.position -= 99;
                }
                *self.count.entry(self.position).or_insert(0) += 1;
                Ok(())
            }
            _ => Err("Got invalid direction"),
        }
    }

    fn get_count(&self, key: i32) -> Option<&i32> {
        self.count.get(&key)
    }
}

fn parse_input(path: &Path) -> io::Result<std::io::Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}

fn parse_line(line: &str) -> Rotation {
    let mut chars = line.chars();
    let direction = chars.next().unwrap();
    let count: i32 = chars.collect::<String>().parse().unwrap();
    Rotation::new(direction, count)
}

fn main() {
    let input_file = Path::new("input/day1.txt");
    let input = parse_input(input_file).expect("failed to parse input");

    let mut lock = Lock::new();

    for line in input {
        let line = line.expect("failed to read line");
        let rotation = parse_line(&line);
        // println!("{:?}", rotation);
        lock.turn(&rotation).expect("Failed to turn lock");
    }

    if let Some(count) = lock.get_count(0) {
        println!("Lock at position 0 count: {}", count);
    } else {
        println!("Lock never hit 0...");
    }
}
