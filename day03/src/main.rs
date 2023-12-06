use std::collections::HashSet;
use std::env;
use std::fs;

// use colored::Colorize;

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

type Point = (usize, usize);

// What is the sum of part numbers next to symbols (king adjacency)?
fn part_one(schematic: &Vec<Vec<char>>) {
    let mut parts: Vec<Point> = Vec::new();

    let row_max = schematic[0].len() as isize;
    let col_max = schematic.len() as isize;

    // find all parts symbols
    for (i, row) in schematic.iter().enumerate() {
        for (j, sym) in row.iter().enumerate() {
            if !sym.is_digit(10) && *sym != '.' {
                parts.push((i, j));
            }
        }
    }

    // find any adjacent numbers
    #[rustfmt::skip]
    let adjacents: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];
    let digits: Vec<Point> = parts
        .iter()
        .flat_map(|(i, j)| {
            adjacents
                .iter()
                .filter_map(move |(a, b)| {
                    // filter out any OOB coords
                    return match (*i as isize + a, *j as isize + b) {
                        (-1, _) => None,
                        (_, -1) => None,
                        (r, _) if r >= row_max => None,
                        (_, c) if c >= col_max => None,
                        (x, y) => Some((x as usize, y as usize)),
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
        })
        .collect();

    // find start and length of number for each point found
    // use Set for deduplication
    let mut starts: HashSet<Point> = HashSet::new();
    // make string rows from array of chars
    let rows: Vec<String> = schematic
        .iter()
        .map(|r| r.iter().collect::<String>())
        .collect();
    for (i, j) in digits {
        let row = &rows[i as usize];
        // char after first non-digit, searching back from original digit j
        let start = match row[..j].rfind(|c: char| !c.is_digit(10)) {
            Some(x) => x + 1, // char after the found non-digit
            None => 0,        // or the start of string if no non-digits found
        };

        starts.insert((i, start));
    }

    // // highlight found digits
    // for (i, row) in schematic.iter().enumerate() {
    //     for (j, sym) in row.iter().enumerate() {
    //         if starts.contains(&(i, j)) {
    //             print!("{}", String::from(*sym).red())
    //         } else {
    //             print!("{sym}")
    //         }
    //     }
    //     println!("")
    // }

    // now parse those digits
    // (we need to wait until after all are found before parsing because there
    // might be duplicate digits in the found list)
    let part_numbers = starts.iter().map(|(row, start)| {
        schematic[*row][*start..]
            .iter()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    });

    println!("Part 1: {}", part_numbers.sum::<usize>());
}


#[allow(unused)]
fn part_two(schematic: &Vec<Vec<char>>) {

    println!("Part 2: {}", 0);
}
