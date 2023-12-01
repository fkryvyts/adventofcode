use std::fs;
use std::ops::Add;

fn main() {
    let data = fs::read_to_string("inputs/day_01.txt").expect("Unable to read file");
    let result: Vec<u64> = data
        .trim()
        .split("\n")
        .map(|s| {
            let first = parse_digits(s.chars(), false);
            let last = parse_digits(s.chars().rev(), true);

            let value: u64 = match (first, last) {
                (Some(first), None) => first * 11,
                (Some(first), Some(last)) => first * 10 + last,
                _ => 0,
            };

            value
        })
        .collect::<Vec<_>>();

    let s: u64 = result.iter().sum();

    println!("{:?}", s);
}

fn parse_digits(chars: impl Iterator<Item = char>, is_rev: bool) -> Option<u64> {
    let mut digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .map(String::from)
    .to_vec();

    if is_rev {
        digit_words = digit_words
            .iter()
            .map(|s| s.chars().rev().collect())
            .collect();
    }

    let mut buff = String::new();

    for c in chars {
        let d = c.to_digit(10).unwrap_or(0);

        if d > 0 {
            return Some(d as u64);
        }

        buff = buff.add(format!("{}", c).as_str());

        for (i, word) in digit_words.iter().enumerate() {
            if buff.ends_with(word) {
                return Some((i + 1) as u64);
            }
        }
    }

    return None;
}
