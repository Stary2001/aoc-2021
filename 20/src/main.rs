use std::fs::File;
use std::io::{BufReader, BufRead};
use common::InputError;
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<(String, HashMap<(isize, isize), bool>), InputError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let enhancement = lines.next().unwrap()?.to_string();
    lines.next().unwrap()?;

    let mut image: HashMap<(isize, isize), bool> = HashMap::new();

    let mut y = 0;
    for line in lines {
        let line = line?;

        for (x, c) in line.chars().enumerate() {
            image.insert((x as isize,y), c == '#');
        }

        y += 1;
    }

    Ok((enhancement, image))
}

fn dump(image: &HashMap<(isize, isize), bool>) {
    let min_x = image.iter().map(|(pos, _)| pos.0).min().unwrap();
    let min_y = image.iter().map(|(pos, _)| pos.1).min().unwrap();

    let max_x = image.iter().map(|(pos, _)| pos.0).max().unwrap();
    let max_y = image.iter().map(|(pos, _)| pos.1).max().unwrap();

    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            if image.get(&(x,y)).unwrap_or(&false) == &true {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }
}

fn enhance(enhancement: &str, image: &HashMap<(isize, isize), bool>, default: bool) -> HashMap<(isize, isize), bool> {
    let mut enhanced_image: HashMap<(isize, isize), bool> = HashMap::new();

    let max_x = image.iter().map(|(pos, _)| pos.0).max().unwrap();
    let max_y = image.iter().map(|(pos, _)| pos.1).max().unwrap();

    for y in -2 ..max_y + 3 {
        for x in -2 ..max_x + 3 {
            let bits = [
                image.get(&(x-1, y-1)).unwrap_or(&default),
                image.get(&(x,   y-1)).unwrap_or(&default),
                image.get(&(x+1, y-1)).unwrap_or(&default),

                image.get(&(x-1, y)).unwrap_or(&default),
                image.get(&(x,   y)).unwrap_or(&default),
                image.get(&(x+1, y)).unwrap_or(&default),

                image.get(&(x-1, y+1)).unwrap_or(&default),
                image.get(&(x,   y+1)).unwrap_or(&default),
                image.get(&(x+1, y+1)).unwrap_or(&default),
            ];

            let mut index = 0;
            for (position, bit) in bits.iter().rev().enumerate() {
                if **bit {
                    index |= 1<<position;
                }
            }
            
            enhanced_image.insert((x + 2,y + 2), enhancement.chars().nth(index).unwrap() == '#');
        }
    }

    enhanced_image
}

fn part_1(enhancement: &str, image: &HashMap<(isize, isize), bool>) -> usize {
    // time to be annoyed at the input, lmao
    let mut default = false;
    let flip = if enhancement.chars().nth(0).unwrap() == '.' {
        false
    } else {
        true
    };

    let one = enhance(enhancement, image, default);

    if flip { default = !default; }
    let two = enhance(enhancement, &one, default);
    
    two.into_values().filter(|x| x == &true).count()
}

fn part_2(enhancement: &str, image: &HashMap<(isize, isize), bool>) -> usize {
    // time to be annoyed at the input, lmao
    let mut default = false;
    let flip = if enhancement.chars().nth(0).unwrap() == '.' {
        false
    } else {
        true
    };

    let mut enhanced = image.clone();
    for _step in 0..50 {
        enhanced = enhance(enhancement, &enhanced, default);
        if flip { default = !default; }
    }

    enhanced.into_values().filter(|x| x == &true).count()
}

#[test]
fn test() {
    let (enhancement, lines) = read_input("test.txt").unwrap();

    assert_eq!(part_1(&enhancement, &lines), 35);
    assert_eq!(part_2(&enhancement, &lines), 3351);
}

fn main() {
    let (enhancement, lines) = read_input("input.txt").unwrap();
    println!("Part 1: {:?}", part_1(&enhancement, &lines));
    println!("Part 2: {:?}", part_2(&enhancement, &lines));
}
