use std::fs::File;
use std::io::{BufReader, BufRead};
use common::InputError;
use std::str::FromStr;

fn read_input(filename: &str) -> Result<Vec<Vec<usize>>, InputError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().map(|x| x?.chars().map(|y| usize::from_str(&y.to_string()).map_err(|err| InputError::from(err))).collect()).collect()
}

fn dump(input: &Vec<Vec<usize>>) {
    let height = input.len();
    for y in 0..height {
        let width = input[y].len();
        for x in 0..width {
            if input[y][x] > 9 {
                print!("9");
            } else {
                print!("{:?}", input[y][x]);
            }
        }
        println!("");
    }

    println!("======================");
}

fn part_1_2(input: &Vec<Vec<usize>>, part_1: bool) -> usize {
    let mut board = input.clone();
    let mut flash_count: usize = 0;
    let mut step: usize = 0;

    loop {
        let row = [false; 10].to_vec();
        // todo: better
        let mut flashed: Vec<Vec<bool>> = Vec::new();
        for _ in 0..10 {
            flashed.push(row.clone());
        }

        let height = board.len();
        let width = board[0].len();

        println!("Step {:?}", step);
        dump(&board);

        // increase
        for y in 0..height {
            for x in 0..width {
                board[y][x] += 1;
            }
        }

        loop {
            let mut did_work = false;
            for y in 0..height {
                for x in 0..width {
                    if board[y][x] > 9 && !flashed[y][x] {
                        flashed[y][x] = true;
                        did_work = true;

                        flash_count += 1;

                        // increase adjacent 
                        // left
                        if x > 0 { board[y][x-1] += 1; }
                        // right
                        if x < width-1 { board[y][x+1] += 1; }
                        // top
                        if y > 0 { board[y-1][x] += 1; }
                        // bottom
                        if y < height-1 { board[y+1][x] += 1; }

                        // top left
                        if x > 0 && y > 0 { board[y-1][x-1] += 1; }
                        // top right
                        if x < width-1 && y > 0 { board[y-1][x+1] += 1; }
                        // bottom left
                        if x > 0 && y < height-1 { board[y+1][x-1] += 1; }
                        // bottom right
                        if x < width-1 && y < height-1 { board[y+1][x+1] += 1; }
                    }
                }
            }

            for y in 0..height {
                for x in 0..width {
                    if flashed[y][x] {
                        board[y][x] = 0;
                    }
                }
            }

            dump(&board);
            if !did_work {
                println!("settled");
                break;
            }
        }

        let mut all_flashed = true;
        for y in 0..height {
            for x in 0..width {
                if !flashed[y][x] {
                    all_flashed = false;
                }
            }
        }

        step += 1;

        if !part_1 && all_flashed {
            break
        }

        if part_1 && step == 100 {
            break
        }
    }

    if part_1 { flash_count } else { step }
}

#[test]
fn test() {
    let input = read_input("test.txt").unwrap();
    assert_eq!(part_1_2(&input, true), 1656);
    assert_eq!(part_1_2(&input, false), 195);
}

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("Part 1: {:?}", part_1_2(&input, true));
    println!("Part 2: {:?}", part_1_2(&input, false));
}
