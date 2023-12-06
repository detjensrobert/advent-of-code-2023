use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

type Input = BTreeMap<usize, usize>;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();

    let (seeds, maps) = parse(&infile);

    part_one(&seeds, &maps);
    part_two(&seeds, &maps);
}

fn map_parse(str: &str) -> Vec<usize> {
    return str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
}

fn parse(lines: &str) -> (Vec<usize>, Vec<Input>) {
    print!("parsing");

    // almanac sections split by blank lines
    let mut sections = lines.split_terminator("\n\n");

    // first section is special (the initial seeds)
    let seeds = map_parse(sections.next().unwrap().split_once(':').unwrap().1);

    // rest of the sections are maps
    let maps = sections
        .map(|chunk| {
            print!(".");
            io::stdout().flush();

            let mut lines = chunk.split_terminator("\n");
            lines.next(); // discard label line

            let mut map = BTreeMap::new();

            for range in lines.map(|l| map_parse(l)) {
                let [dst_start, src_start, length] = range[0..3] else {
                    unreachable!()
                };

                for i in 0..length {
                    map.insert(src_start + i, dst_start + i);
                }
            }

            return map;
        })
        .collect();

    println!(" done!");
    return (seeds, maps);
}

// what is the minimum location for any seed after all conversion?
fn part_one(seeds: &[usize], maps: &[Input]) {
    // create working array from seeds we can modify
    let mut scratch = Vec::from(seeds);

    print!("converting");

    for map in maps {
        print!(".");
        io::stdout().flush();
        // update progress with result of current lookup
        scratch = scratch
            .iter()
            .map(|src| match map.get(src) {
                dst @ Some(_) => *dst.unwrap(),
                None => *src,
            })
            .collect();
    }

    println!("Part 1: {}", scratch.iter().min().unwrap());
}

fn part_two(seeds: &[usize], maps: &[Input]) {
    println!("Part 2: {}", 0);
}
