use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;

use common::InputError;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct BingoBoard {
    board: [[u32; 5]; 5],
    marks: [[bool; 5]; 5]
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            board: [[0; 5]; 5],
            marks: [[false; 5]; 5]
        }
    }

    fn score(&self, draw: u32) -> u32 {
        let mut score: u32 = 0;
        for x in 0..5 {
            for y in 0..5 {
                if self.marks[x][y] == false {
                    score += self.board[x][y];
                }
            }
        }
        
        score * draw
    }
}

#[derive(Clone, Debug)]
struct BingoInput {
    order: Vec<u32>,
    boards: Vec<BingoBoard>
}

fn read_input(filename: &str) -> Result<BingoInput, InputError> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let order_string = lines.next().ok_or(InputError{message: "Not enough lines"})??;

    let order_maybe: Result<Vec<u32>, ParseIntError> = order_string.split(",").map(|x| u32::from_str(x)).collect();
    let order = order_maybe?;

    let mut boards = Vec::new();

    for x in &lines.chunks(6) {
        let mut board = BingoBoard::new();
        // do it
        let chunk: Vec<String> = x.collect::<Result<Vec<String>, std::io::Error>>()?;
        if chunk.len() != 6 {
            return Err(InputError { message: "Not enough lines" })
        }

        for i in 0..5 {
            let numbers: Result<Vec<u32>, ParseIntError> = chunk[i+1].split(" ").filter(|x| !x.is_empty()).map(|x| u32::from_str(&x)).collect();
            let a = numbers?;
            board.board[i].copy_from_slice(&a[0..5]);
        }

        boards.push(board);
    }

    Ok(BingoInput {
        order: order,
        boards: boards
    })
}

fn part_1(input: &BingoInput, find_first: bool) -> u32 {
    // bingo time

    let mut boards = input.boards.clone();
    
    for draw in &input.order {
        for i in 0..boards.len() {
            // Mark it off on each board
            for x in 0..5 {
                for y in 0..5 {
                    if boards[i].board[x][y] == *draw {
                        boards[i].marks[x][y] = true;
                    }
                }
            }
        }

        let mut winners = Vec::new();

        // check boards
        for i in 0..boards.len() {
            let mut found = false;

            for y in 0..5 {
                let mut row = true;
                for x in 0..5 {
                    if boards[i].marks[x][y] == false {
                        row = false;
                    }
                }
                if row && !found {
                    if find_first {
                        return boards[i].score(*draw);
                    } else {
                        winners.push(i);
                        found = true;
                    }
                }
            }

            for x in 0..5 {
                let mut col = true;
                for y in 0..5 {
                    if boards[i].marks[x][y] == false {
                        col = false;
                    }
                }
                
                if col && !found {
                    if find_first {
                        return boards[i].score(*draw);
                    } else {
                        winners.push(i);
                        found = true;
                    }
                }
            }
        }

        winners.sort();
        let mut offset: usize = 0;
        for idx in winners {
            //println!("How many boards are left: {:?}", boards.len());

            if boards.len() == 1 {
                return boards[0].score(*draw);
            }

            boards.remove(idx - offset);
            offset += 1;
        }
    }

    0
}

#[test]
fn test() {
    let bingo = read_input("test.txt").unwrap();
    assert_eq!(part_1(&bingo, true), 4512);
    assert_eq!(part_1(&bingo, false), 1924);
}

fn main() {
    let bingo = read_input("input.txt").unwrap();
    println!("part 1: {:?}", part_1(&bingo, true));
    println!("part 2: {:?}", part_1(&bingo, false));
}
