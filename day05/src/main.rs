use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Range;

use indicatif::*;
use rayon::prelude::*;

type Input = HashMap<Range<usize>, Range<usize>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();

    let (seeds, maps) = parse(&infile);

    // dbg!(&seeds);
    // dbg!(&maps);

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
    // almanac sections split by blank lines
    let mut sections = lines.split_terminator("\n\n");

    // first section is special (the initial seeds)
    let seeds = map_parse(sections.next().unwrap().split_once(':').unwrap().1);

    // store mappings as ranges, since expanding them out is a) memory exhausting and b) dog slow
    let mut maps = Vec::new();
    for chunk in sections {
        let mut lines = chunk.split_terminator("\n");
        lines.next(); // discard label line

        let mut map = HashMap::new();

        for range in lines.map(|l| map_parse(l)) {
            let [dst_start, src_start, length] = range[0..3] else {
                unreachable!() // input will always match three
            };
            map.insert(src_start..src_start + length, dst_start..dst_start + length);
        }

        maps.push(map);
    }

    return (seeds, maps);
}

// what is the minimum location for any seed after all conversion?
fn part_one(seeds: &[usize], maps: &[Input]) {
    // create working array from seeds we can modify
    let mut current = Vec::from(seeds);

    for map in maps {
        // update progress with result of current translation
        current = current
            .iter()
            .map(|src|
                // try to find range key that contains `src`
                *match map.keys().find(|r| r.contains(&src)) {
                    // miss (not contained, use src as dst)
                    None => src,

                    // hit, lookup in corresponding dst range
                    Some(src_range) => {
                        let dst_range = &map[src_range];

                        // index into dest based on position in src range
                        return dst_range.start + (src - src_range.start);
                    },
                })
            .collect();
    }

    println!("Part 1: {}", current.iter().min().unwrap());
}

// The seeds are actually start/length range pairs. What is the minimum location
// out of all seeds from these ranges?
fn part_two(seeds: &[usize], maps: &[Input]) {
    let style1 =
        ProgressStyle::with_template("expanding {percent}%, {eta} [{bar}] {pos}/{len}").unwrap();
    let style2 =
        ProgressStyle::with_template("iterating {percent}%, {eta} [{bar}] {pos}/{len}").unwrap();

    // create working array from seeds we can modify
    // expand range here
    let mut current: Vec<usize> = seeds
        .chunks_exact(2)
        .collect::<Vec<&[usize]>>()
        .par_iter()
        .progress_with_style(style1)
        .flat_map(|pair| pair[0]..pair[0] + pair[1])
        .collect();

    for map in maps {
        // update progress with result of current translation
        // current = current
        //     .par_iter()
        //     .progress_with_style(style2.clone())
        //     .map(|src|
        //         // try to find range key that contains `src`
        //         *match map.keys().find(|r| r.contains(&src)) {
        //             // miss (not contained, use src as dst)
        //             None => src,

        //             // hit, lookup in corresponding dst range
        //             Some(src_range) => {
        //                 let dst_range = &map[src_range];
        //                 return dst_range.start + (src - src_range.start);
        //             },
        //         })
        //     .collect();


        // this is dumb and brute force and takes like 10m but it does work
        current.par_iter_mut().progress_with_style(style2.clone()).for_each(|src| {
            // try to find range key that contains `src`
            *src = match map.keys().find(|r| r.contains(&src)) {
                // miss (not contained, use src as dst)
                None => *src,

                // hit, lookup in corresponding dst range
                Some(src_range) => map[src_range].start + (*src - src_range.start),

            };
        });
    }

    println!("Part 1: {}", current.iter().min().unwrap());
}
