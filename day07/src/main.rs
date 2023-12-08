use std::collections::HashMap;
use std::env;
use std::fs;


#[repr(isize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
use HandType::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    htype: HandType,
    // cards: String,
    cards: Vec<usize>,
    bid: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();

    let input = parse(&lines, parse_hand);
    part_one(&input);

    let input2 = parse(&lines, parse_hand_pt2);
    part_two(&input2);
}

fn parse(lines: &[&str], hand_parser: fn(&&str) -> Hand) -> Vec<Hand> {
    lines.iter().map(|l| hand_parser(l)).collect()
}

fn parse_hand(raw_hand: &&str) -> Hand {
    let (cards, bid) = raw_hand.split_once(" ").unwrap();

    // count occurences of each char
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in cards.chars() {
        *counts.entry(c).or_default() += 1;
    }
    // figure out hand from counts
    let mut countvals = counts.values().collect::<Vec<&usize>>();
    countvals.sort();
    countvals.reverse();
    let htype = match countvals[..] {
        [5] => FiveKind,
        [4, _] => FourKind,
        [3, 2] => FullHouse,
        [3, ..] => ThreeKind,
        [2, 2, ..] => TwoPair,
        [2, ..] => Pair,
        _ => HighCard,
    };

    // convert card chars to numeric value
    let card_order = "23456789TJQKA";
    let card_vals = cards.chars().map(|c| card_order.find(c).unwrap()).collect();

    Hand {
        htype: htype,
        cards: card_vals,
        bid: bid.parse::<usize>().unwrap(),
    }
}

// its poker! multiply each hand's rank with its bid.
fn part_one(input: &Vec<Hand>) {
    let mut sorted = input.to_vec();

    // sort by order of struct fields
    // type, cards, then bid
    sorted.sort();

    let winnings = sorted
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid);

    println!("Part 1: {}", winnings);
}

fn parse_hand_pt2(raw_hand: &&str) -> Hand {
    let (cards, bid) = raw_hand.split_once(" ").unwrap();

    // count occurences of each char
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in cards.chars() {
        *counts.entry(c).or_default() += 1;
    }

    // pull out jokers
    let jokers = counts.remove(&'J').unwrap_or(0);

    // figure out hand from counts
    let mut countvals = counts.into_values().collect::<Vec<usize>>();
    countvals.sort();
    countvals.reverse();

    // add jokers to first/max card
    *countvals.first_mut().unwrap_or(&mut 0) += jokers;

    let htype = match countvals[..] {
        [5] => FiveKind,
        [4, _] => FourKind,
        [3, 2] => FullHouse,
        [3, ..] => ThreeKind,
        [2, 2, ..] => TwoPair,
        [2, ..] => Pair,
        _ if jokers == 5 => FiveKind,
        _ => HighCard,
    };

    // convert card chars to numeric value
    let card_order = "J23456789TQKA"; // jokers now worst
    let card_vals = cards.chars().map(|c| card_order.find(c).unwrap()).collect();

    Hand {
        htype: htype,
        cards: card_vals,
        bid: bid.parse::<usize>().unwrap(),
    }
}

// J are now Jokers, worst valued but wildcards
fn part_two(input: &[Hand]) {
    let mut sorted = input.to_vec();

    // sort by order of struct fields
    // type, cards, then bid
    sorted.sort();

    // for h in &sorted {
    //     println!("{:?}", h)
    // }

    let winnings = sorted
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid);

    println!("Part 2: {}", winnings);
}
