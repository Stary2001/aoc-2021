use std::fs::File;
use std::io::{BufReader, BufRead};
use common::InputError;
use std::str::FromStr;
use std::num::ParseIntError;

fn read_input(filename: &str) -> Result<Vec<Vec<usize>>, InputError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut heightmap = Vec::new();

    for line in reader.lines() {
        let line_maybe = line?;
	let row: Vec<usize> = line_maybe.chars().map(|x| usize::from_str(&x.to_string())).collect::<Result<Vec<usize>, ParseIntError>>()?;
        heightmap.push(row);
    }
    Ok(heightmap)
}

fn try_get(input: &Vec<Vec<usize>>, x: isize, y: isize) -> Option<usize> {
	if y < 0 || y >= input.len() as isize {
		None
	} else if x < 0 || x >= input[y as usize].len() as isize {
		None
	} else {
		Some(input[y as usize][x as usize])
	}
	
}

fn part_1(input: &Vec<Vec<usize>>) -> usize {
	let mut sum = 0;

	for y in 0..input.len() as isize {
		for x in 0..input[y as usize].len() as isize {
			let current = try_get(input, x, y).unwrap(); // better be valid

			let adjacent = [ try_get(input, x-1, y), try_get(input, x+1, y), try_get(input, x, y-1), try_get(input, x, y+1) ];
			if adjacent.iter().filter_map(|x| x.map(|y| current < y)).reduce(|a,b| a&b).unwrap() {
				sum += current + 1;
			}
		}
	}
	
	sum
}

fn dump(input: &Vec<Vec<usize>>, regions: &Vec<Vec<(usize,usize)>>) {
        for y in 0..input.len() {
                for x in 0..input[y].len() {
                        let mut found = false;
                        for (region_id, r) in regions.iter().enumerate() {
                                if r.iter().any(|a| *a == (x,y)) {
                                        print!("{:03x}", region_id);
                                        found = true;
                                }
                        }
                        if !found {
                                print!("###");
                        }
                }
                println!("");
        }
}

fn part_2(input: &Vec<Vec<usize>>) -> usize {
	// flood fill to get all 9s?
	let mut considered: Vec<(usize,usize)> = Vec::new();
	let mut regions: Vec<Vec<(usize,usize)>> = Vec::new();

	// initial fill
	for y in 0..input.len() {
		for x in 0..input[y].len() {
			if !considered.iter().any(|a| *a==(x,y)) {
				println!("considering {:?} {:?}", x,y);
				// figure out what region to add it to
				if input[y][x] != 9 {
					let mut found = false;
					for r in regions.iter_mut() {
						if (x > 0 && r.iter().any(|a| *a==(x-1, y))) ||
						   r.iter().any(|a| *a==(x+1, y)) ||
						   (y > 0 && r.iter().any(|a| *a==(x, y-1))) ||
						   r.iter().any(|a| *a==(x, y+1)) {
							found = true;
							r.push((x,y));
							break;
						}
					}
						
					if !found {
						let mut new_region = Vec::new();
						new_region.push((x,y));
						regions.push(new_region);
					}
				}

				considered.push((x,y));
			}
		}
	}
	
	// merge regions that are touching
	// lol this is definitely o(n^2) but n is small (number of regions)
	let mut merges: Vec<(usize, usize)> = Vec::new();
	
	for (region_id, r) in regions.iter().enumerate() {
		for pos in r.iter() {
			for (region_id_2, r2) in regions.iter().enumerate() {
				if region_id == region_id_2 { continue; }

				if (pos.0 > 0 && r2.iter().any(|a| *a == (pos.0 - 1, pos.1))) || 
				   r2.iter().any(|a| *a == (pos.0 + 1, pos.1)) || 
				   (pos.1 > 0 && r2.iter().any(|a| *a == (pos.0, pos.1 - 1))) ||
				   r2.iter().any(|a| *a == (pos.0, pos.1 + 1)) {
					//println!("merge {:?} {:?}", region_id, region_id_2);

					if ! merges.iter().any(|merge| (merge.0 == region_id && merge.1 == region_id_2) || (merge.1 == region_id && merge.0 == region_id_2)) {
						merges.push((region_id, region_id_2));
					}
				}
			}
		}
	}

	println!("{:?}", regions.iter().map(|x| x.len()).collect::<Vec<usize>>());
	
	// merges SHOULD be sorted
	// if they aren't this is doomed

	for m in merges.iter() {
		println!("Merge {:?} {:?}", m.0, m.1);

		let mut to_append = regions[m.0].clone();
		regions[m.0].clear();
		regions[m.1].append(&mut to_append);
	}

	let mut region_sizes: Vec<usize> = regions.iter().map(|x| x.len()).collect();
	region_sizes.sort();

	region_sizes[region_sizes.len() - 3 .. region_sizes.len()].iter().product::<usize>()
}

#[test]
fn simple() {
    let input = read_input("test.txt").unwrap();
    assert_eq!(part_1(&input), 15);
    assert_eq!(part_2(&input), 1134);
}

fn main() {
    let input = read_input("input.txt").unwrap();

    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}
