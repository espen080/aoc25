use crate::utils::parse_input;
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
    pub zero_count: i32,
    pub rotation_count: i32,
}

impl Lock {
    fn new() -> Self {
        Lock {
            position: 50,
            zero_count: 0,
            rotation_count: 0,
        }
    }

    fn turn(&mut self, rotation: &Rotation) -> Result<(), &'static str> {
        // The shift is the number of clicks the lock position will actually move.
        let shift = rotation.count % 100;
        match rotation.direction {
            'L' => {
                self.position -= shift;
                if self.position < 0 {
                    // The lock now passed 0 during the shift.
                    self.position += 100;

                    // Check for special case where starting position was 0
                    if self.position + shift - 100 != 0 {
                        self.rotation_count += 1;
                    }
                }
            }
            'R' => {
                self.position += shift;
                if self.position > 99 {
                    // The lock now passed 0 during the shift.
                    self.position -= 100;

                    // Check for special case where position ended at 0
                    if self.position != 0 {
                        self.rotation_count += 1;
                    }
                }
            }
            _ => return Err("Got invalid direction"),
        }
        // If the rotation is more than the number of clicks in the lock then
        // we need to account for the additional full rotations.
        let extra_rotations = (rotation.count - shift) / 100;
        self.rotation_count += extra_rotations;
        if self.position == 0 {
            self.zero_count += 1;
        }
        Ok(())
    }
}

fn parse_line(line: &str) -> Rotation {
    let mut chars = line.chars();
    let direction = chars.next().unwrap();
    let count: i32 = chars.collect::<String>().parse().unwrap();
    Rotation::new(direction, count)
}

pub fn solution() {
    let input_file = Path::new("input/day1.txt");
    let input = parse_input(input_file).expect("failed to parse input");

    let mut lock = Lock::new();

    for line in input {
        let line = line.expect("failed to read line");
        let rotation = parse_line(&line);
        lock.turn(&rotation).expect("Failed to turn lock");
    }

    println!("Lock landed at position zero {} times", lock.zero_count);
    println!("Lock passed position zero {} times", lock.rotation_count);
    println!(
        "Lock was pointing to zero a total of {} times",
        lock.zero_count + lock.rotation_count
    );
}
