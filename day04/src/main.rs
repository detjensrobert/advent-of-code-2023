use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct Card {
    numbers: Vec<usize>,
    winners: Vec<usize>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();

    let mut input = parse(&lines);

    part_one(&input);
    part_two(&mut input);
}

fn map_parse(str: &str) -> Vec<usize> {
    return str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
}

fn parse(lines: &[&str]) -> Vec<Card> {
    lines
        .iter()
        .map(|l| {
            let (_name, card) = l.split_once(':').unwrap();
            let (numbers, winners) = card.split_once('|').unwrap();

            return Card {
                numbers: map_parse(numbers),
                winners: map_parse(winners),
            };
        })
        .collect()
}

fn win_count(c: &Card) -> usize {
    let mut num_set: HashSet<usize> = HashSet::new();
    num_set.extend(&c.numbers);
    let mut win_set: HashSet<usize> = HashSet::new();
    win_set.extend(&c.winners);

    return num_set.intersection(&win_set).count();
}

// what is the sum of all the cards' scores? 2^(# matching numbers)
fn part_one(input: &[Card]) {
    let points = input.iter().map(|c| match win_count(c) {
        0 => 0,
        hits => 0b1 << hits - 1,
    });

    println!("Part 1: {}", points.sum::<usize>());
}

// instead of points, wins give extra copies of following cards. how many cards
// are left once all copies have been won?
fn part_two(input: &mut Vec<Card>) {
    let len = input.len();
    let mut copies: Vec<usize> = vec![0; len];

    for (i, card) in input.into_iter().enumerate() {
        let wins = win_count(&card);
        // println!("found {} wins for card {} (and {} copies)", wins, i, copies[i]);

        // add one to copies of next $win cards
        for copy_i in (i + 1..len).take(wins) {
            // println!("  adding copy of {}", copy_i);
            copies[copy_i] += 1 + copies[i];
        }
    }

    println!(
        "Part 2: {}",
        len + copies.iter().sum::<usize>()
    );
}
