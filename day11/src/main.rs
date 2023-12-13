use std::env;
use std::fs;
use std::vec;

use itertools::Itertools;

use grid::Grid;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();

    let input = parse(&lines);

    part_one(&input);
    part_two(&input);
}

fn parse(lines: &[&str]) -> Grid<bool> {
    let width = lines[0].len();
    let parsed = lines
        .iter()
        .flat_map(|l| l.chars().map(|c| c == '#'))
        .collect();
    let universe = Grid::from_vec(parsed, width);

    return universe;
}

fn expand(universe: &Grid<bool>) -> Grid<bool> {
    let mut new_univ = universe.to_owned();

    // expand empty rows
    let empty_rows: Vec<usize> = new_univ
        .iter_rows()
        .enumerate()
        .filter_map(|(i, r)| match r.to_owned().any(|g| *g) {
            true => None,
            false => Some(i),
        })
        .collect();
    // go backwards so no need to adjust index for newly inserted
    for r in empty_rows.into_iter().rev() {
        new_univ.insert_row(r, vec![false; new_univ.cols()]);
    }

    // expand any cols
    let empty_cols: Vec<usize> = new_univ
        .iter_cols()
        .enumerate()
        .filter_map(|(i, c)| match c.to_owned().any(|g| *g) {
            true => None,
            false => Some(i),
        })
        .collect();
    // go backwards so no need to adjust index for newly inserted
    for c in empty_cols.into_iter().rev() {
        new_univ.insert_col(c, vec![false; new_univ.rows()]);
    }

    return new_univ;
}

// fn printgrid(g: &Grid<bool>) {
//     for r in g.iter_rows() {
//         println!(
//             "{}",
//             r.map(|c| if *c { '#' } else { '.' }).collect::<String>()
//         );
//     }
// }

// Find the sum of the shortest paths between all galaxies after expansion.
fn part_one(universe: &Grid<bool>) {
    let expanded = expand(universe);

    // pull out all galaxies
    let galaxies: Vec<(usize, usize)> = expanded
        .indexed_iter()
        .filter_map(|(coords, c)| if *c { Some(coords) } else { None })
        .collect();

    let distances: Vec<isize> = galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| {
            let [a, b] = pair[0..2] else { unreachable!() };
            (a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()
        })
        .collect();

    // dbg!(&distances);

    println!("Part 1: {}", distances.iter().sum::<isize>());
}

fn expand_n(universe: &Grid<bool>, factor: usize) -> Grid<bool> {
    let mut new_univ = universe.to_owned();

    // expand empty rows
    let empty_rows: Vec<usize> = new_univ
        .iter_rows()
        .enumerate()
        .filter_map(|(i, r)| match r.to_owned().any(|g| *g) {
            true => None,
            false => Some(i),
        })
        .collect();
    // go backwards so no need to adjust index for newly inserted
    for r in empty_rows.into_iter().rev() {
        for _ in (1..factor) {
            new_univ.insert_row(r, vec![false; new_univ.cols()]);
        }
    }

    // expand any cols
    let empty_cols: Vec<usize> = new_univ
        .iter_cols()
        .enumerate()
        .filter_map(|(i, c)| match c.to_owned().any(|g| *g) {
            true => None,
            false => Some(i),
        })
        .collect();
    // go backwards so no need to adjust index for newly inserted
    for c in empty_cols.into_iter().rev() {
        for _ in (1..factor) {
            new_univ.insert_col(c, vec![false; new_univ.rows()]);
        }
    }

    return new_univ;
}

// now expand 1_000_000 times!!!
fn part_two(universe: &Grid<bool>) {
    // let expanded = expand_n(universe, 100);
    let expanded = expand_n(universe, 1_000_000);

    // pull out all galaxies
    let galaxies: Vec<(usize, usize)> = expanded
        .indexed_iter()
        .filter_map(|(coords, c)| if *c { Some(coords) } else { None })
        .collect();

    let distances: Vec<isize> = galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| {
            let [a, b] = pair[0..2] else { unreachable!() };
            (a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()
        })
        .collect();

    // dbg!(&distances);

    println!("Part 2: {}", distances.iter().sum::<isize>());
}
