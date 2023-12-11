use std::env;
use std::fs;
use std::ops::Index;

use graph::prelude::*;

use sqrid;

mod smash;

type Edge = ((usize, usize), (usize, usize));

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();


    let a: Edge =  ( (0,0), (1,1) );
    let b: Edge =  ( (0,1), (1,1) );
    let c: Edge =  ( (1,1), (0,0) );

    assert!(!mutuals(&a, &b));
    assert!(mutuals(&a, &c));

    // let input = parse(&lines);

    // part_one(&input);
    // part_two(&input);
}

fn mutuals(a: &Edge, b: &Edge) -> bool {
    a.0 == b.1 && a.1 == b.0
}

fn parse(lines: &[&str]) -> UndirectedCsrGraph<u64> {
    let map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut edges: Vec<Edge> = Vec::new();

    // find all tiles that try to connect to their neighbors
    println!("parsing edges...");
    for (i, line) in map.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            match tile {
                '|' => {
                    if i > 0 {
                        edges.push(((i, j), (i - 1, j)))
                    }
                    if i < map.len() {
                        edges.push(((i, j), (i + 1, j)))
                    }
                }
                '-' => {
                    if j > 0 {
                        edges.push(((i, j), (i, j - 1)))
                    }
                    if j < line.len() {
                        edges.push(((i, j), (i, j + 1)))
                    }
                }
                'L' => {
                    if i > 0 {
                        edges.push(((i, j), (i - 1, j)))
                    }
                    if j < line.len() {
                        edges.push(((i, j), (i, j + 1)))
                    }
                }
                'J' => {
                    if i > 0 {
                        edges.push(((i, j), (i - 1, j)))
                    }
                    if j > 0 {
                        edges.push(((i, j), (i, j - 1)))
                    }
                }
                '7' => {
                    if i < map.len() {
                        edges.push(((i, j), (i + 1, j)))
                    }
                    if j > 0 {
                        edges.push(((i, j), (i, j - 1)))
                    }
                }
                'F' => {
                    if i < map.len() {
                        edges.push(((i, j), (i + 1, j)))
                    }
                    if j < line.len() {
                        edges.push(((i, j), (i, j + 1)))
                    }
                }
                '.' => {}
                'S' => {
                    // try to associate with all 4 neighbors
                    if i > 0 {
                        edges.push(((i, j), (i - 1, j)))
                    }
                    if i < map.len() {
                        edges.push(((i, j), (i + 1, j)))
                    }
                    if j > 0 {
                        edges.push(((i, j), (i, j - 1)))
                    }
                    if j < line.len() {
                        edges.push(((i, j), (i, j + 1)))
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    for (from, to) in &edges {
        println!(
            "{:?} {} -> {:?} {}",
            from, map[from.0][from.1], to, map[to.0][to.1]
        );
    }

    // remove all non mutual edges
    println!("deduplicating...");
    let only_mutuals: Vec<&Edge> = edges
        .iter()
        .filter(|a|
            edges.iter().any(|b|
                mutuals(a, b)
            )
        )
        .collect();

    for (from, to) in &only_mutuals {
        println!(
            "{:?} {} -> {:?} {}",
            from, map[from.0][from.1], to, map[to.0][to.1]
        );
    }

    println!("{} to {}", edges.len(), only_mutuals.len());
    assert!(edges.len() != only_mutuals.len());

    println!("packing...");
    let packed: Vec<(u64, u64)> = only_mutuals
        .iter()
        .map(|(f, t)| (smash::from_usize(*f), smash::from_usize(*t)))
        .collect();

    println!("building graph...");
    let graph: UndirectedCsrGraph<u64> = GraphBuilder::new()
        .csr_layout(CsrLayout::Sorted)
        .edges(packed)
        .build();

    println!("done!");
    return graph;
}

fn part_one(input: &UndirectedCsrGraph<u64>) {
    println!("Part 1: {}", 0);
}

#[allow(unused)]
fn part_two(input: &UndirectedCsrGraph<u64>) {
    println!("Part 2: {}", 0);
}
