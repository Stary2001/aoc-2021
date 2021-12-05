use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use common::InputError;
use std::cmp::max;
use std::cmp::min;

#[derive(Debug)]
struct Vec2 {
    x: usize,
    y: usize
}

impl Vec2 {
    fn parse(s: &str) -> Result<Vec2, InputError> {
        let parts: Vec<&str> = s.split(",").collect();
        Ok(Vec2 {
            x: usize::from_str(parts[0])?,
            y: usize::from_str(parts[1])?
        })
    }
}

#[derive(Debug)]
struct Line {
    start: Vec2,
    end: Vec2
}

impl Line {
    fn parse(s: &str) -> Result<Line, InputError> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        Ok(Line {
            start: Vec2::parse(parts[0])?,
            end: Vec2::parse(parts[1])?,
        })
    }
}

fn read_input(filename: &str) -> Result<Vec<Line>, InputError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    lines.map(|x| Line::parse(&x?)).collect()
}

fn part_1(input: &Vec<Line>) -> usize 
{
    let mut grid: Vec<usize> = Vec::new();
    let max_x = max(
                    input.iter().map(|line| line.start.x).max(), 
                    input.iter().map(|line| line.end.x).max()
                ).unwrap();

    let max_y = max(
                    input.iter().map(|line| line.start.y).max(), 
                    input.iter().map(|line| line.end.y).max()
                ).unwrap();

    grid.resize((max_x+1) * (max_y+1), 0);

    for line in input.iter() {
        // for part 1, only straight lines
        if line.start.x == line.end.x {
            // vertical
            let start = min(line.start.y, line.end.y);
            let end = max(line.start.y, line.end.y);

            for y in start .. end+1 {
                grid[line.start.x + y * max_x] += 1;
            }
        } else if line.start.y == line.end.y {
            // horizontal

            let start = min(line.start.x, line.end.x);
            let end = max(line.start.x, line.end.x);
            for x in start .. end+1 {
                grid[x + line.start.y * max_x] += 1;
            }
        }
    }

    grid.iter().filter(|x| **x >= 2).count()
}

fn part_2(input: &Vec<Line>) -> usize 
{
    let mut grid: Vec<usize> = Vec::new();
    let max_x = max(
                    input.iter().map(|line| line.start.x).max(), 
                    input.iter().map(|line| line.end.x).max()
                ).unwrap();

    let max_y = max(
                    input.iter().map(|line| line.start.y).max(), 
                    input.iter().map(|line| line.end.y).max()
                ).unwrap();

    grid.resize((max_x+1) * (max_y+1), 0);

    for line in input.iter() {
        let mut x: usize = line.start.x;
        let mut y: usize = line.start.y;

        let x_step: isize = if line.start.x > line.end.x { -1 } else if line.start.x == line.end.x { 0 } else { 1 };
        let y_step: isize = if line.start.y > line.end.y { -1 } else if line.start.y == line.end.y { 0 } else { 1 };
        loop {
            grid[x + y * (max_x+1)] += 1;

            if x == line.end.x && y == line.end.y {
                break
            }

            x = (x as isize + x_step) as usize;
            y = (y as isize + y_step) as usize;
        }
    }

    grid.iter().filter(|x| **x >= 2).count()
}


#[test]
fn test() {
    let input = read_input("test.txt").unwrap();
    assert_eq!(part_1(&input), 5);
    assert_eq!(part_2(&input), 12);
}

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}
