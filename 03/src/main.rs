use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct InputError {
    message: &'static str
}

impl From<std::num::ParseIntError> for InputError {
    fn from(_: std::num::ParseIntError) -> Self {
        InputError{  message: "ParseIntError" }
    }
}

impl From<std::io::Error> for InputError {
    fn from(_: std::io::Error) -> Self {
        InputError{  message: "io error"}
    }
}

impl From<&std::io::Error> for InputError {
    fn from(_: &std::io::Error) -> Self {
        InputError{ message: "io error" }
    }
}

fn read_input(filename: &str) -> Result<(Vec<u32>,usize), InputError> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines().peekable();
    let first = lines.peek().ok_or(InputError{message: "wtf"})?;
    let n_bits = match first {
        Ok(x) => {
            x.len()
        }
        Err(e) => Err(e)?
    };

    println!("{:?}", n_bits);

    let argh = lines.map(|x| Ok(u32::from_str_radix(&x?, 2)?)).collect::<Result<Vec<u32>, InputError>>()?;
    Ok((argh, n_bits))
}

fn part_1(input: &Vec<u32>, n_bits: usize) -> u32 {
    let mut counts: [u32; 32] = [0; 32];
    for x in input.iter() {
        for bit in 0..n_bits {
            if x & (1<<bit) == 1<<bit {
                // bit set
                counts[bit] += 1;
            }
        }
    }

    let mut gamma: u32 =0;
    for bit in 0..n_bits {
        if counts[bit] > (input.len()/2) as u32 {
            gamma |= 1<<bit;
        }
    }

    let epsilon = !gamma & ((1<<n_bits) - 1);

    gamma * epsilon
}

fn part_2(input: &Vec<u32>, n_bits: usize) -> u32 {
    let mut left = input.clone();
    let mut oxygen: u32 = 0;

    // oxygen
    for bit in (0..n_bits).rev() {
        let mut counts: [u32; 32] = [0; 32];
        for x in left.iter() {
            for bit in 0..n_bits {
                if x & (1<<bit) == 1<<bit {
                    // bit set
                    counts[bit] += 1;
                }
            }
        }

        if counts[bit] * 2 >= left.len() as u32 {
            // 1 is common
            left = left.iter().filter(|x| (*x & (1<<bit)) == (1<<bit) ).map(|x| *x).collect();
        } else {
            // 0 is common
            left = left.iter().filter(|x| (*x & (1<<bit)) == 0).map(|x| *x).collect();
        }

        if left.len() == 1 {
            oxygen = left[0];
            break
        } else if left.len() == 0 {
            panic!("How did we get here?");
        }
    }

    let mut co2: u32 = 0;
    left = input.clone();
    // co2
    for bit in (0..n_bits).rev() {
        let mut counts: [u32; 32] = [0; 32];
        for x in left.iter() {
            for bit in 0..n_bits {
                if x & (1<<bit) == 1<<bit {
                    // bit set
                    counts[bit] += 1;
                }
            }
        }

        if counts[bit] * 2 < left.len() as u32 {
            // 1
            left = left.iter().filter(|x| (*x & (1<<bit)) == (1<<bit) ).map(|x| *x).collect();
        } else {
            // 0
            left = left.iter().filter(|x| (*x & (1<<bit)) == 0).map(|x| *x).collect();
        }
        if left.len() == 1 {
            co2 = left[0];
            break
        } else if left.len() == 0 {
            panic!("How did we get here?");
        }
    }

    co2 * oxygen
}

#[test]
fn example_input() {
    let (input, n_bits) = read_input("test.txt").unwrap();
    assert_eq!(part_1(&input, n_bits), 198);
    assert_eq!(part_2(&input, n_bits), 230);
}

fn main() {
    let (input, n_bits) = read_input("input.txt").unwrap();
    println!("part 1: {:?}", part_1(&input, n_bits));
    println!("part 2: {:?}", part_2(&input, n_bits));
}