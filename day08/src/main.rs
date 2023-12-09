use std::collections::HashMap;
use std::env;
use std::fs;

use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    // name: char,
    left: String,
    right: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();

    let (instructions, nodes) = parse(&lines);

    part_one(&instructions, &nodes);
    part_two(&instructions, &nodes);
}

fn parse(lines: &[&str]) -> (String, HashMap<String, Node>) {
    let instructions = String::from(*lines.first().unwrap());

    let mut nodes = HashMap::new();
    for l in &lines[2..] {
        let split =
            l.split(|c| !char::is_alphanumeric(c))
            .filter(|s| s.len() > 0);

        let [name, left, right] = split.collect::<Vec<&str>>()[..] else { unreachable!() };

        nodes.insert(
            name.to_owned(),
            Node {
                left:  left.to_owned(),
                right: right.to_owned(),
            },
        );
    }

    return (instructions, nodes);
}

// Follow instructions from AAA to ZZZ. How many steps does it take?
fn part_one(instr: &String, nodes: &HashMap<String, Node>) {
    // repeat instructions endlessly
    let instr_loop = instr.chars().cycle();
    // start at AAA
    let mut current = "AAA";
    let mut steps = 0;
    // get to ZZZ
    for step in instr_loop {
        steps += 1;
        let next = nodes.get(current).unwrap();

        // println!("going {step} from {current}");

        current = match step {
            'L' => &next.left,
            'R' => &next.right,
            _ => unreachable!()
        };

        if current == "ZZZ" { break; }
    }


    println!("Part 1: {}", steps);
}


// Follow instructions from any node that ends in A to any ending in Z. How many
// steps does it take for all to arrive at *Z nodes?
fn part_two(instr: &String, nodes: &HashMap<String, Node>) {
    // try all starts, find longest one
    let starts = nodes.keys().filter(|name| name.ends_with("A") );
    let mut currents: Vec<&String> = starts.collect();

    let mut steps = 0;

    // repeat instructions endlessly
    let instr_loop = instr.chars().cycle();
    // repeat instructions forever
    for step in instr_loop {
        steps += 1;
        // try each node
        currents = currents.par_iter().map(|c| {
            let next = nodes.get(c.to_owned()).unwrap();

            // println!("going {step} from {c}");

            return match step {
                'L' => &next.left,
                'R' => &next.right,
                _ => unreachable!()
            };
        }).collect();

        // println!("z count: {}", currents.iter().filter(|n| n.ends_with("Z")).count()  );

        // only end when all are z
        if currents.iter().all(|n| n.ends_with("Z")) { break }
    }

    println!("Part 2: {}", steps);
}
