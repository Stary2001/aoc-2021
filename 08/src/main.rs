use std::fs::File;
use std::io::{BufReader, BufRead};
use common::InputError;

fn string_to_bits(input: &str) -> u8 {
    input.chars().map(|x| 1 << (x as usize - 'a' as usize)).reduce(|a,b| a|b).unwrap()
}

fn sevenseg(input: u8) {
    for bit in 0..8 {

    }
}

fn read_input(filename: &str) -> Result<Vec<(Vec<u8>, Vec<u8>)>, InputError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut results = Vec::new();

    for line in reader.lines() {
        let line_maybe = line?;
        let halves: Vec<&str> = line_maybe.split(" | ").collect();
        let first = halves[0].split(" ").map(|x| string_to_bits(x)).collect();
        let second = halves[1].split(" ").map(|x| string_to_bits(x)).collect();
        results.push((first, second));
    }
    Ok(results)
}

fn part_1(input: &Vec<(Vec<u8>,Vec<u8>)>) -> usize {    
    // 1 uses two segments
    // 4 uses four segments
    // 7 uses three segments
    // 8 uses all 7 segements

    input.iter().map(|y| {
        y.1.iter().filter(|x| x.count_ones() == 2 || x.count_ones() == 4 || x.count_ones() == 3 || x.count_ones() == 7).count()
    }).sum()
}

fn digit_mapping(input: &Vec<u8>) -> [u8; 10] {
    // the canonical 7seg i am using:
    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg

    let one = *input.iter().filter(|x| x.count_ones() == 2).next().unwrap();
    let four = *input.iter().filter(|x| x.count_ones() == 4).next().unwrap();
    let seven = *input.iter().filter(|x| x.count_ones() == 3).next().unwrap();
    let eight = *input.iter().filter(|x| x.count_ones() == 7).next().unwrap();

    // count_zeros also counts the high bit we don't use...
    // really we need a u7 type, but.. no

    // zero, six, and nine all miss one segment
    // two, three, and five miss two segments
    // one, four, seven, eight are unique

    let mut one_missing: Vec<u8> = input.iter().filter(|x| (eight & *x).count_zeros() == 2).map(|x| *x).collect();
    let mut two_missing: Vec<u8> = input.iter().filter(|x| (eight & *x).count_zeros() == 3).map(|x| *x).collect();

    // find the top segment
    let a = seven & !one;

    // !six & one gets us c
    let six = one_missing.remove(one_missing.iter().position(|x| (*x & one).count_ones() == 1).unwrap());
    let c = !six & one;

    // with c, we can find f
    let f = one & !c;

    // now we can find five, to find e
    let five = two_missing.remove(two_missing.iter().position(|x| (*x & c) == 0).unwrap());
    let e = (!five & !c) & (0x7f);

    // with e, we can find b
    let three = two_missing.remove(two_missing.iter().position(|x| (*x & e) == 0).unwrap());
    let b = (!three & !e) & (0x7f);

    // we have: a,b,c,e,f
    // find d,g

    let zero = one_missing.remove(one_missing.iter().position(|x| (*x & e) != 0).unwrap());
    let nine = one_missing.remove(one_missing.iter().position(|x| (*x & e) == 0).unwrap());

    let two = two_missing.remove(two_missing.iter().position(|x| (*x & f) == 0).unwrap());

    [zero, one, two, three, four, five, six, seven, eight, nine]
}

fn part_2(input: &Vec<(Vec<u8>, Vec<u8>)>) -> usize {
    input.iter().map(|entry| {
        let mapping = digit_mapping(&entry.0);
        let mut num = 0;
        let mut current = 1000;
        for digit in entry.1.iter() {
            let value = mapping.iter().position(|x| x == digit).unwrap();
            num += value * current;
            current /= 10;
        }

        num
    }).sum()
}

#[test]
fn simple() {
    let input = read_input("test_small.txt").unwrap();
    assert_eq!(part_1(&input), 0);
    assert_eq!(part_2(&input), 5353);
}

#[test]
fn complex() {
    let input = read_input("test_complex.txt").unwrap();
    assert_eq!(part_1(&input), 26);
    assert_eq!(part_2(&input), 61229);
}


fn main() {
    let input = read_input("input.txt").unwrap();

    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}
