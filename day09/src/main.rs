use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();

    let input = parse(&lines);

    part_one(&input);
    part_two(&input);
}

fn parse(lines: &[&str]) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

fn calc_deltas(row: &Vec<isize>) -> Vec<isize> {
    let mut prev = row.first().unwrap();
    return row[1..]
        .iter()
        .map(|v| {
            let delta = v - prev;
            prev = v;
            return delta;
        })
        .collect();
}

fn calc_all_deltas(row: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut all_deltas = vec![row.to_owned()];

    let mut curr = row.to_owned();
    // while deltas are non zero
    while !curr.iter().all(|v| *v == 0) {
        curr = calc_deltas(&curr);
        all_deltas.push(curr.to_owned());
    }

    return all_deltas;
}

// Derive until the delta is zero. What is the sum of all the next numbers?
fn part_one(input: &Vec<Vec<isize>>) {
    let mut nexts = Vec::new();

    for row in input {
        let deltas = calc_all_deltas(row);
        let lasts: Vec<isize> = deltas.iter().map(|d| *d.last().unwrap()).rev().collect();
        // apply each delta to next row up
        nexts.push(lasts.iter().fold(0, |delta, curr| curr + delta));
    }

    println!("Part 1: {}", nexts.iter().sum::<isize>());
}

// As before, but find a previous value instead.
fn part_two(input: &Vec<Vec<isize>>) {
    let mut prevs = Vec::new();

    for row in input {
        let deltas = calc_all_deltas(row);
        let lasts: Vec<isize> = deltas.iter().map(|d| *d.first().unwrap()).rev().collect();
        // apply each delta to next row up
        prevs.push(lasts.iter().fold(0, |delta, curr| curr - delta));
    }

    println!("Part 2: {}", prevs.iter().sum::<isize>());
}
