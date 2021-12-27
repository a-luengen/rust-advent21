use std::fs::File;
use std::io::prelude::*;

/**
 * Direction the submarine can navigate.
*/
#[derive(Debug)]
enum Directions {
    /**
     * Increase horizontal position.
     * Part 2:
     *  - Increase horizontal position
     *  - Increase depth by aim multiplied with units
     */
    Forward,
    /**
     * Increase depth.
     * Part 2:
     *  - Increase aim
     */
    Down,
    /**
     * Decrease depth.
     * Part 2:
     *  - decrease aim
     */
    Up,
}

#[derive(Debug)]
struct Command {
    pub direction: Directions,
    pub step_size: u32,
}

struct Submarine {
    pub pos_depth: u32,
    pub pos_horizontal: u32,
}

impl Submarine {
    pub fn apply_commands(&mut self, commands: &Vec<Command>) {
        for command in commands {
            match command.direction {
                Directions::Forward => self.pos_horizontal += command.step_size,
                Directions::Down => self.pos_depth += command.step_size,
                Directions::Up => self.pos_depth -= command.step_size,
            }
        }
    }
}

struct AdvancedSubmarine {
    pub pos_depth: i32,
    pub pos_horizontal: i32,
    pub aim: i32,
}

impl AdvancedSubmarine {
    pub fn apply_commands(&mut self, commands: &Vec<Command>) {
        for command in commands {
            match command.direction {
                Directions::Forward => {
                    self.pos_horizontal += command.step_size as i32;
                    self.pos_depth += self.aim * command.step_size as i32;
                }
                Directions::Down => self.aim += command.step_size as i32,
                Directions::Up => self.aim -= command.step_size as i32,
            }
        }
    }
}

fn parse_commands_from_string(content: &mut String) -> Option<Vec<Command>> {
    let commands: Vec<Command> = content
        .split("\n")
        .filter(|val| val.len() > 0)
        .map(|val| {
            let splitted: Vec<&str> = val.split(" ").collect();
            let dir: Directions = match splitted[0] {
                "forward" => Directions::Forward,
                "up" => Directions::Up,
                "down" => Directions::Down,
                _ => panic!("Couldnt parse value: {}", splitted[0]),
            };
            let step = splitted[1].parse().unwrap();
            Command {
                direction: dir,
                step_size: step,
            }
        })
        .collect();

    Some(commands)
}

fn main() -> std::io::Result<()> {
    //let input_path = "./input_test.txt";
    let input_path = "./input_final.txt";
    let mut file = File::open(input_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let commands = parse_commands_from_string(&mut content).unwrap();
    let mut sub = Submarine {
        pos_depth: 0,
        pos_horizontal: 0,
    };
    sub.apply_commands(&commands);
    println!(
        "Current position of submarine: depth = {}, hor = {}",
        sub.pos_depth, sub.pos_horizontal
    );
    println!("Result: {}", sub.pos_horizontal * sub.pos_depth);

    println!("With advanced Submarine");
    let mut a_sub = AdvancedSubmarine {
        pos_depth: 0,
        aim: 0,
        pos_horizontal: 0,
    };

    a_sub.apply_commands(&commands);
    println!(
        "Current position of submarine: depth = {}, hor = {}, aim = {}",
        a_sub.pos_depth, a_sub.pos_horizontal, a_sub.aim
    );
    println!("Result: {}", a_sub.pos_horizontal * a_sub.pos_depth);
    Ok(())
}
