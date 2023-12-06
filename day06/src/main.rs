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
    // find roots, plot, and filter for times above the target

    let winning_times: Vec<Vec<isize>> = input
        .iter()
        .map(|(time, record)|
            // find times faster than
            (1..*time).into_iter()
                // roots are no button (0,0) and all button (time, 0)
                .map(|x| -1 * (x - 0) * (x - time))
                .filter(|dist| dist > record )
                .collect::<Vec<isize>>())
        .collect();

    // count and product the number of winning times
    let winning_counts = winning_times.iter().map(|r| r.len());
    println!("Part 1: {}", winning_counts.product::<usize>());
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

    let winning_times = (1..big_time).into_iter()
        // roots are no button (0,0) and all button (time, 0)
        .map(|x| -1 * (x - 0) * (x - big_time))
        .filter(|dist| dist > &big_record)
        .collect::<Vec<isize>>();

    println!("Part 2: {}", winning_times.len());
}
