use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let input: Vec<&str> = infile.split_terminator("\n").collect();

    part_one(&input);
    part_two(&input);
}

// Combine the first and last digit to get a 2-digit number
fn part_one(input: &[&str]) {
    let numbers = input.iter().map(|l| {
        let first = l.chars().find(|c| c.is_numeric()).unwrap();
        let last = l.chars().rfind(|c| c.is_numeric()).unwrap();

        10 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap()
    });

    println!("Part 1: {}", numbers.sum::<u32>());
}

// As above, but also recognize digits as words i.e. 'one' == 1
fn part_two(input: &[&str]) {
    let numbers = input.iter().map(|l| {
        let mut l2 = String::from(l.to_owned());

        // iterative replacements
        for _ in 1..10 {
            l2 = l2
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .replace("zero", "z0o");
        }

        let first = l2.chars().find(|c| c.is_numeric()).unwrap();
        let last = l2.chars().rfind(|c| c.is_numeric()).unwrap();

        10 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap()
    });

    println!("Part 2: {}", numbers.sum::<u32>());
}
