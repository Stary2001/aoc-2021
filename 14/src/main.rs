use std::fs::File;
use std::io::{BufReader, BufRead};
use common::InputError;
use std::collections::HashMap;

#[derive(Debug)]
struct Insert {
    from: (char, char),
    to: char
}

impl Insert {
    fn new(s: &str) -> Insert {
        let mut parts = s.split(" -> ");

        let a = parts.next().unwrap().to_string();
        let b = parts.next().unwrap().to_string();

        let mut c = a.chars();
        Insert {
            from: (c.next().unwrap(), c.next().unwrap()),
            to: b.chars().nth(0).unwrap()
        }
    }
}

fn read_input(filename: &str) -> Result<(String, Vec<Insert>), InputError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut inserts: Vec<Insert> = Vec::new();

    let mut lines = reader.lines();
    let template = lines.next().ok_or(InputError{message: "not enough lines"})??;
    lines.next();

    for line in lines {
        let line = line?;
        inserts.push(Insert::new(&line));
    }

    Ok((template.to_string(), inserts))
}

fn adjust(p: (char, char), pairs: &mut HashMap<(char, char), usize>, amt: isize) {
    let current: isize = *pairs.get(&p).unwrap_or(&0) as isize;
    if current + amt == 0 {
        // remove
        pairs.remove(&p);
    } else {
        pairs.insert(p, (current + amt) as usize);
    }
}

fn polymer(template: &str, inserts: &Vec<Insert>, num_steps: usize) -> usize {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();

    for i in 0 .. template.len() - 1 {
        let pair = &template[i .. i + 2];
        let mut c = pair.chars();
        let tup = (c.next().unwrap(), c.next().unwrap());
        *pairs.entry(tup).or_insert(0) += 1;
    }

    for _step in 0 .. num_steps {
        let mut staging: HashMap<(char, char), isize> = HashMap::new();

        for (pair, num) in pairs.iter() {
            for r in inserts {
                if *pair == r.from {
                    *staging.entry(r.from).or_insert(0) -= *num as isize;
                    *staging.entry((r.from.0, r.to)).or_insert(0) += *num as isize;
                    *staging.entry((r.to, r.from.1)).or_insert(0) += *num as isize;
                    break;
                }
            }
        }

        for (pair, num) in staging {
            adjust(pair, &mut pairs, num);
        }
    }

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for (pair, num) in pairs {
        *char_counts.entry(pair.0).or_insert(0) += num;
        *char_counts.entry(pair.1).or_insert(0) += num;
    }

    let mut char_numbers: Vec<usize> = char_counts.into_values().collect();
    char_numbers.sort();

    // "that's right - we're gonna cheat"
    // this is double counting all the numbers, so just divide by 2 (rounding up)
    println!("{:?}", char_numbers);
    (char_numbers[char_numbers.len() - 1] + 1)/2 - (char_numbers[0]+1)/2
}

fn part_1(template: &str, inserts: &Vec<Insert>) -> usize {
    polymer(template, inserts, 10)
}

fn part_2(template: &str, inserts: &Vec<Insert>) -> usize {
    polymer(template, inserts, 40)
}

#[test]
fn test() {
    let (template, inserts) = read_input("test.txt").unwrap();
    assert_eq!(part_1(&template, &inserts), 1588);
    assert_eq!(part_2(&template, &inserts), 2188189693529);
}

fn main() {
    let (template, inserts) = read_input("input.txt").unwrap();
    println!("part 1: {:?}", part_1(&template, &inserts));
    println!("part 2: {:?}", part_2(&template, &inserts));
}