use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

#[derive(Debug)]
struct CommandErr {}

impl From<ParseIntError> for CommandErr {
    fn from(_: ParseIntError) -> Self {
        CommandErr{}
    }
}

impl From<std::io::Error> for CommandErr {
    fn from(_: std::io::Error) -> Self {
        CommandErr{}
    }
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    fn new(cmd: &str) -> Result<Command, CommandErr> {
        let x: Vec<&str> = cmd.split(" ").collect();
        match x[0] {
            "forward" => Ok(Command::Forward(str::parse::<i32>(x[1])?)),
            "up" => Ok(Command::Up(str::parse::<i32>(x[1])?)),
            "down" => Ok(Command::Down(str::parse::<i32>(x[1])?)),
            _ => Err(CommandErr{})
        }
    }
}

fn read_input(filename: &str) -> Result<Vec<Command>,CommandErr> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|x| Command::new(&x?)).collect()
}

fn part_1(input: &Vec<Command>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for cmd in input.iter() {
        match cmd {
            Command::Forward(x) => horizontal += x, 
            Command::Up(x) => depth -= x,
            Command::Down(x) => depth += x
        }
    }

    horizontal * depth
}

fn part_2(input: &Vec<Command>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in input.iter() {
        match cmd {
            Command::Forward(x) => {
                horizontal += x; 
                depth += x * aim
            },
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x
        }
    }

    horizontal * depth
}

#[test]
fn example_input() {
    let input = read_input("test.txt").unwrap();
    assert_eq!(part_1(&input), 150);
    assert_eq!(part_2(&input), 900);
}

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("part 1: {:?}", part_1(&input));
    println!("part 2: {:?}", part_2(&input));
}
