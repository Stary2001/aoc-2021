use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect()
}

fn part_1(filename: &str) -> i32 {
    let depths = read_input(filename);
    let mut count = 0;

    for slice in depths.as_slice().windows(2) {
        if slice[1] > slice[0] {
            count += 1;
        }
    }

    count
}

fn part_2(filename: &str) -> i32 {
    let depths = read_input(filename);
    let windows: Vec<i32> = depths.as_slice().windows(3).map(|x| x.iter().sum()).collect();

    let mut count = 0;

    for slice in windows.as_slice().windows(2) {
        if slice[1] > slice[0] {
            count += 1;
        }
    }

    count
}

#[test]
fn example_input() {
    assert_eq!(part_1("test.txt"), 7);
    assert_eq!(part_2("test.txt"), 5);
}

fn main() {
    println!("part 1: {}", part_1("input.txt"));
    println!("part 2: {}", part_2("input.txt"));
}
