use std::cmp::Ordering;
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
struct RGB {
    r: usize,
    g: usize,
    b: usize,
}
impl PartialOrd for RGB {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self == other {
            Some(Ordering::Equal)
        } else if self.r <= other.r && self.g <= other.g && self.b <= other.b {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "no input given!");

    let infile = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = infile.split_terminator("\n").collect();

    let input = parse(&lines);

    part_one(&input);
    part_two(&input);
}

fn parse(lines: &[&str]) -> Vec<Vec<RGB>> {
    lines
        .iter()
        .map(|l| {
            // ("Game X", "x red, y blue; z green")
            let (_name, outcome) = l.split_once(": ").unwrap();

            return outcome
                // split outcome into each set
                // ["x red, y blue", "z green"]
                .split("; ")
                .map(|set| {
                    // split set into each color
                    // ["x red", "y blue"]
                    let colors = set.split(", ");
                    let mut count = RGB { r: 0, g: 0, b: 0 };

                    // convert into rgb triple
                    // (x, 0, y)
                    for cl in colors {
                        match cl.split_once(' ').unwrap() {
                            (r @ _, "red") => count.r = r.parse::<usize>().unwrap(),
                            (g @ _, "green") => count.g = g.parse::<usize>().unwrap(),
                            (b @ _, "blue") => count.b = b.parse::<usize>().unwrap(),
                            (_, &_) => panic!("didnt recognize pattern!"),
                        }
                    }

                    return count;
                })
                .collect();
        })
        .collect()
}

// Determine which games would have been possible if the bag had been loaded
// with only 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those games?
fn part_one(input: &Vec<Vec<RGB>>) {
    // squash games into maximum occurences of each color
    let maxs = input.iter().map(|game| RGB {
        r: game.iter().map(|set| set.r).max().unwrap(),
        g: game.iter().map(|set| set.g).max().unwrap(),
        b: game.iter().map(|set| set.b).max().unwrap(),
    });

    // find games that are possible i.e. game max is less than bag
    let bag = RGB {
        r: 12,
        g: 13,
        b: 14,
    };
    let ids: Vec<usize> = maxs
        // add game index to max list
        .zip(1..)
        // find ganmes that are possible with the given bag
        .filter_map(|(game_max, idx)| if game_max <= bag { Some(idx) } else { None })
        .collect();

    // dbg!(&ids);

    // convert 0-index to 1-index name before summing
    println!("Part 1: {}", ids.iter().sum::<usize>());
}

// Find the product of the minimum possible cube counts for each game?
// What is the sum of those products?
#[allow(unused)]
fn part_two(input: &Vec<Vec<RGB>>) {
    // squash games into maximum occurences of each color
    let maxs = input.iter().map(|game| RGB {
        r: game.iter().map(|set| set.r).max().unwrap(),
        g: game.iter().map(|set| set.g).max().unwrap(),
        b: game.iter().map(|set| set.b).max().unwrap(),
    });

    // find product of those
    let  products = maxs.map(|m|
        m.r * m.g * m.b
    );

    println!("Part 2: {}", products.sum::<usize>());
}
