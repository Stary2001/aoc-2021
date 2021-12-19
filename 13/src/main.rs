use std::fs::File;
use std::io::{BufReader, BufRead};
use common::InputError;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    Horizontal(usize),
    Vertical(usize)
}

fn read_input(filename: &str) -> Result<(HashSet<(usize,usize)>, Vec<Fold>), InputError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut locations: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();

    let mut found_empty = false;
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            found_empty = true;
            continue;
        }

        if !found_empty {
            // location
            let parts: Result<Vec<usize>, ParseIntError> = line.split(",").map(|x| usize::from_str(x)).collect();
            let parts = parts?;
            locations.insert((parts[0], parts[1]));
        } else {
            // fold
            if line.starts_with("fold along x=") {
                folds.push(Fold::Vertical(usize::from_str(&line["fold along x=".len()..])?));
            } 
            else if line.starts_with("fold along y=") {
                folds.push(Fold::Horizontal(usize::from_str(&line["fold along x=".len()..])?));
            }
        }
    }

    Ok((locations, folds))
}

fn dump(locations: &HashSet<(usize, usize)>) {
    let max_x = locations.iter().map(|x| x.0).max().unwrap();
    let max_y = locations.iter().map(|y| y.1).max().unwrap();
    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            if locations.contains(&(x,y)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

}

fn fold_point_horizontal(point: &(usize, usize), y: usize) -> (usize, usize) {
    if point.1 < y {
        *point
    } else {
        (point.0, y - (point.1-y))
    }
}


fn fold_point_vertical(point: &(usize, usize), x: usize) -> (usize, usize) {
    if point.0 < x {
        *point
    } else {
        (x - (point.0-x), point.1)
    }
}

fn do_fold(locations: &mut HashSet<(usize, usize)>, fold: Fold) {
    
}

fn part_1(input_locs: &HashSet<(usize, usize)>, folds: &Vec<Fold>) -> usize {
    let mut locations = input_locs.clone();

    let first_fold = folds.iter().next().unwrap();
    match first_fold {
        // fold up
        Fold::Horizontal(y) => {
            locations = locations.iter().map(|point| fold_point_horizontal(point, *y)).collect();
        },

        // fold left
        Fold::Vertical(x) => {
            locations = locations.iter().map(|point| fold_point_vertical(point, *x)).collect();
        }
    }

    locations.len()
}

fn part_2(input_locs: &HashSet<(usize, usize)>, folds: &Vec<Fold>) -> usize {
    let mut locations = input_locs.clone();

    for fold in folds.iter() {
        match fold {
            // fold up
            Fold::Horizontal(y) => {
                locations = locations.iter().map(|point| fold_point_horizontal(point, *y)).collect();
            },

            // fold left
            Fold::Vertical(x) => {
                locations = locations.iter().map(|point| fold_point_vertical(point, *x)).collect();
            }
        }
    }
    
    dump(&locations);

    locations.len()
}
#[test]
fn test() {
    let (locations, folds) = read_input("test.txt").unwrap();
    assert_eq!(part_1(&locations, &folds), 17);
    // ugh i don't want to auto test part 2, it works
}

fn main() {
    let (locations, folds) = read_input("input.txt").unwrap();
    println!("part 1: {:?}", part_1(&locations, &folds));
    println!("part 2: {:?}", part_2(&locations, &folds));
}
