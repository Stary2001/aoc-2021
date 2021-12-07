use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
use std::num::ParseIntError;
use common::InputError;
use std::collections::VecDeque;

fn read_input(filename: &str) -> Result<Vec<usize>, InputError> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents.split(",").map(|x| usize::from_str(&x)).collect::<Result<Vec<usize>, ParseIntError>>()?)
}

// This was, of course, slow as hell on part 2.
/*fn fish(input: &Vec<u8>, num_days: usize) -> usize 
{
    let mut fishtank = input.clone();
    for i in 0.. num_days {
        println!("day {:?}: {:?}", i, fishtank.len());
        let mut new_fish = 0;

        for fish in fishtank.iter_mut() {  
            if *fish == 0 {
                *fish = 7;
                new_fish += 1;
            }

            *fish -= 1;
        }

        for j in 0..new_fish {
            fishtank.push(8);
        }
    }

    fishtank.len()
}*/

fn fish(input: &Vec<usize>, num_days: usize) -> usize 
{
    let mut fishtank: VecDeque<usize> = VecDeque::new();
    fishtank.resize(9, 0);

    for i in input.iter() {
        fishtank[*i] += 1;
    }

    for i in 0.. num_days {
        println!("day {:?}: {:?}", i, fishtank.iter().sum::<usize>());
        let new_fish = fishtank.pop_front().unwrap();
        fishtank.push_back(new_fish);
        fishtank[6] += new_fish;
    }

    fishtank.iter().sum::<usize>()
}

fn part_1(input: &Vec<usize>) -> usize 
{
   fish(input, 80)
}

fn part_2(input: &Vec<usize>) -> usize 
{
   fish(input, 256)
}

#[test]
fn test() {
    let input = read_input("test.txt").unwrap();
    assert_eq!(part_1(&input), 5934);
    assert_eq!(part_2(&input), 26984457539);
}

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}