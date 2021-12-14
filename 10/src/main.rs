use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn read_input(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

#[derive(Debug, PartialEq)]
enum ValidationResult<'a> {
    Ok,
    InProgress(&'a str),
    Bad(&'a str),
    Incomplete(String),
}

use crate::ValidationResult::*;

fn validate(s: &str) -> ValidationResult {
    let closing = HashSet::from([')', ']', '}', '>']);
    let correct = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);

    if s.len() == 0 {
        return Ok
    }

    if s.len() == 1 {
        let ch = s.chars().nth(0).unwrap();
        if closing.contains(&ch) {
            return InProgress(s)
        } else {
            return Incomplete(correct[&ch].to_string())
        }
    }

    // Recursion time (oh no)
    // Do we have a chunk to consume?

    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();

    if closing.contains(&first) {
        // hmm
        return InProgress(s)
    }
    
    if correct[&first] == second {
        return validate(&s[2..])
    } else {
        let result = validate(&s[1..]);
        match result {
            Ok => return Incomplete(correct[&first].to_string()),
            Incomplete(rest) => {
                //println!("incomplete: {:?} {:?} {:}?", s, rest, first);
                return Incomplete(rest + &correct[&first].to_string());
            },
            Bad(x) => return Bad(x),
            InProgress(rest) => {
                // validate s[0] -> s[-1]
                let last = rest.chars().nth(0).unwrap();
                if correct[&first] == last {
                    if rest.len() == 1 {
                        return Ok
                    } else {
                        return validate(&rest[1..])
                    }
                } else {
                    // potential bad
                    return Bad(rest)
                }
            },
        }
    }
}

fn complete(s: &str) -> usize {
    let table = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4)
    ]);

    match validate(s) {
        Ok => 0,
        Bad(_) => 0,
        Incomplete(rest) => {
            let mut score = 0;

            for ch in rest.chars() {
                score *= 5;
                score += table[&ch];
            }
            score
        },
        InProgress(_) => unreachable!()
    }
}

fn part_1(input: &Vec<String>) -> usize {
    let lookup = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    input.iter().map(|x| {
        match validate(x) {
            Bad(c) => {
                lookup[&c.chars().nth(0).unwrap()]
            },
            Ok => 0,
            Incomplete(_) => 0,
            InProgress(_) => unreachable!(),
        }
    }).sum()
}

fn part_2(input: &Vec<String>) -> usize {
    let mut scores: Vec<usize> = input.iter().map(|x| complete(x)).filter(|x| *x != 0).collect();
    scores.sort();
    scores[scores.len()/2]
}

#[test]
fn simple_validation() {
    assert_eq!(validate("([])"), Ok);
    assert_eq!(validate("{()()()}"), Ok);
    assert_eq!(validate("<([{}])>"), Ok);
    assert_eq!(validate("[<>({}){}[([])<>]]"), Ok);
    assert_eq!(validate("(((((((((())))))))))"), Ok);
}

#[test]
fn simple_completion()
{
    assert_eq!(complete("[({(<(())[]>[[{[]{<()<>>"), 288957);
}

#[test]
fn test() {
    let input = read_input("test.txt").unwrap();
    assert_eq!(part_1(&input), 26397);
    assert_eq!(part_2(&input), 288957);
}

fn main() {
    let input = read_input("input.txt").unwrap();
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}
