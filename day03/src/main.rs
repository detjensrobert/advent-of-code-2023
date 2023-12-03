use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    // 2d vec of line chars
    let schematic: Vec<Vec<char>> = infile
        .split_terminator("\n")
        .map(|l| l.chars().collect())
        .collect();

    part_one(&schematic);
    part_two(&schematic);
}

// What is the sum of part numbers next to symbols (king adjaceny)?
fn part_one(schematic: &Vec<Vec<char>>) {
    // find position of all engine symbols
    // i.e. not digit, not .
    // let chars: Vec<(usize, usize)> = input
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(i, row)| {
    //         row.iter()
    //             .enumerate()
    //             .filter_map(|(j, sym)| {
    //                 if !sym.is_digit(10) && *sym != '.' {
    //                     Some(j)
    //                 } else {
    //                     None
    //                 }
    //             })
    //     })
    //     .collect()
    //     .flatten();

    let mut parts: Vec<(i32, i32)> = Vec::new();

    let row_max = schematic[0].len() as i32;
    let col_max = schematic.len() as i32;

    // find all parts symbols
    for (i, row) in schematic.iter().enumerate() {
        for (j, sym) in row.iter().enumerate() {
            if !sym.is_digit(10) && *sym != '.' {
                parts.push((i as i32, j as i32));
            }
        }
    }

    // find any adjacent numbers
    let adjacents: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    //
    let mut digits: Vec<(i32, i32)> = Vec::new();
    for (i, j) in parts {
        // check surroundings around part
        digits.extend(
            adjacents
                .iter()
                .filter_map(|(a, b)| {
                    // filter out any OOB coords
                    return match (i + a, j + b) {
                        (-1, _) => None,
                        (_, -1) => None,
                        (r, _) if r >= row_max => None,
                        (_, c) if c >= col_max => None,
                        x @ (_, _) => Some(x),
                    };
                })
                // find any adjacent numbers
                .filter_map(|(i, j)| {
                    if schematic[i as usize][j as usize].is_digit(10) {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        );
    }

    let mut starts: HashSet<(i32, i32)> = HashSet::new();



    println!("found digits at:");
    for (r, c) in digits {
        println!("({r},{c}): '{}'", schematic[r as usize][c as usize]);
    }

    println!("Part 1: {}", 0);
}

fn part_two(input: &Vec<Vec<char>>) {
    println!("Part 2: {}", 0);
}
