use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
use std::num::ParseIntError;
use common::InputError;

fn read_input(filename: &str) -> Result<Vec<isize>, InputError> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents.split(",").map(|x| isize::from_str(x.trim())).collect::<Result<Vec<isize>, ParseIntError>>()?)
}

fn part_1(input: &Vec<isize>) -> isize 
{
    let size = *input.iter().max().unwrap();

    let minimum_fuel = (0..size).map(|pos| {
        input.iter().map(|x| (*x - pos).abs()).sum::<isize>()
    }).min().unwrap();

    minimum_fuel
}

fn cost(a: isize, b: isize) -> isize {
    let diff = (a-b).abs();
    diff * (diff+1) / 2
}

fn part_2(input: &Vec<isize>) -> isize 
{
    let size = *input.iter().max().unwrap();

    let minimum_fuel = (0..size).map(|pos| {
        input.iter().map(|x| cost(*x, pos).abs()).sum::<isize>()
    }).min().unwrap();

    minimum_fuel
}

#[test]
fn test() {
    let input = read_input("test.txt").unwrap();
    assert_eq!(part_1(&input), 37);
    assert_eq!(part_2(&input), 168);
}

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}