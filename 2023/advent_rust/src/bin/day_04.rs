use regex::Regex;
use std::collections::HashMap;
use std::fs;

// 6284877

fn main() {
    let data = fs::read_to_string("inputs/day_04.txt").expect("Unable to read file");
    let rows: Vec<&str> = data.trim().split("\n").collect();

    let num_re = Regex::new(r"(?<num>\d+)").expect("Unable to compile regex");

    let mut card_copies = HashMap::new();

    let sum: u64 = rows
        .iter()
        .map(|s| parse_card(&mut card_copies, &num_re, s))
        .sum();

    println!("{:?}", sum);
}

fn parse_card(card_copies: &mut HashMap<u64, u64>, num_re: &Regex, line: &str) -> u64 {
    let card_parts: Vec<String> = line
        .split([':', '|'].as_ref())
        .map(str::to_string)
        .collect();

    if card_parts.len() < 3 {
        return 0;
    }

    let card_num = match num_re.captures(card_parts[0].as_str()) {
        Some(capt) => capt["num"].parse::<u64>().unwrap_or(0),
        _ => 0,
    };

    let mut winning_nums = HashMap::new();

    num_re.captures_iter(card_parts[1].as_str()).for_each(|v| {
        let num = String::from(&v["num"]);
        winning_nums.insert(num, true);
    });

    let mut res = 0;

    num_re.captures_iter(card_parts[2].as_str()).for_each(|v| {
        if winning_nums.contains_key(&v["num"]) {
            res += 1
        }
    });

    let copies = match card_copies.get(&card_num) {
        Some(val) => *val + 1,
        _ => 1,
    };

    for i in card_num + 1..=card_num + res {
        match card_copies.get(&i) {
            Some(val) => {
                card_copies.insert(i, val + copies);
            }
            _ => {
                card_copies.insert(i, copies);
            }
        }
    }

    copies
}
