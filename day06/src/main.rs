use std::env;
use std::fs;

type Race = (isize, isize); // (time, dist)

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();

    let input = parse(&lines);

    part_one(&input);
    part_two(&input);
}

fn map_parse(str: &str) -> Vec<isize> {
    return str
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
}

fn parse(lines: &[&str]) -> Vec<Race> {
    let times = map_parse(lines[0].strip_prefix("Time:").unwrap());
    let distances = map_parse(lines[1].strip_prefix("Distance:").unwrap());

    return times.into_iter().zip(distances.into_iter()).collect();
}

// What is the product of the number of ways you could beat the record in each race?
fn part_one(input: &[Race]) {
    // this is a quadratic equation! -(x - 0)(x - time)
    // to find times greater than the record, sub record from the eqn
    // positive times are above the record
    // so: -(x - 0)(x - Time) - Record == -x^2 + (Time)x - Record
    // find roots of this, and count integers between

    // per-race count of winning solutions
    let winning_counts: Vec<isize> = input
        .iter()
        .map(|(time, record)|{
            // add extra bit to record time to force greater instead of equal
            let (a, b, c) = (-1_f32, *time as f32, -(*record as f32 + 0.1));

            let quad_p = (-b + (b*b - 4_f32*a*c).sqrt()) / (2_f32 * a);
            let quad_m = (-b - (b*b - 4_f32*a*c).sqrt()) / (2_f32 * a);

            return (quad_m.floor() - quad_p.ceil()) as isize + 1;
        })
        .collect();

    println!("Part 1: {}", winning_counts.iter().product::<isize>());
}

// This time, theres only one race (concat all the numbers). How many ways ways can this be beat?
fn part_two(input: &[Race]) {
    // squish all the races together
    let (times, records): (Vec<isize>, Vec<isize>) = input.iter().cloned().unzip();

    let big_time = times
        .iter()
        .fold(String::new(), |acc, t| acc + &t.to_string())
        .parse::<isize>().unwrap();
    let big_record = records
        .iter()
        .fold(String::new(), |acc, t| acc + &t.to_string())
        .parse::<isize>().unwrap();

    // add extra bit to record time to force greater instead of equal
    let (a, b, c) = (-1_f32, big_time as f32, -(big_record as f32 + 0.1));

    let quad_p = (-b + (b*b - 4_f32*a*c).sqrt()) / (2_f32 * a);
    let quad_m = (-b - (b*b - 4_f32*a*c).sqrt()) / (2_f32 * a);

    let winning_count = (quad_m.floor() - quad_p.ceil()) as isize + 1;

    println!("Part 2: {}", winning_count);
}
