use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum HandType {
    HighCard = 7,
    OnePair = 6,
    TwoPair = 5,
    ThreeOfKind = 4,
    FullHouse = 3,
    FourOfKind = 2,
    FiveOfKind = 1,
}

#[derive(Debug)]
struct Hand {
    val: String,
    bid: i64,
    hand_type: HandType,
}

fn main() {
    let data = fs::read_to_string("inputs/day_07.txt").expect("Unable to read file");
    let rows: Vec<&str> = data.trim().split("\n").collect();

    part_1(&rows);
    part_2(&rows);
}

fn part_1(rows: &Vec<&str>) {
    let mut hands: Vec<Hand> = rows.iter().map(|r| parse_hand(r, false)).collect();

    let cards_order: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    hands.sort_by(|a, b| {
        let hta = a.hand_type as i64;
        let htb = b.hand_type as i64;

        if hta < htb {
            return Ordering::Greater;
        }

        if hta > htb {
            return Ordering::Less;
        }

        cmp_cards(a.val.as_str(), b.val.as_str(), &cards_order)
    });

    let sum: i64 = hands
        .iter()
        .enumerate()
        .map(|v| (v.0 as i64 + 1) * v.1.bid)
        .sum();

    println!("Part 1 answer: {:?}", sum);
}

fn part_2(rows: &Vec<&str>) {
    let mut hands: Vec<Hand> = rows.iter().map(|r| parse_hand(r, true)).collect();

    let cards_order: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    hands.sort_by(|a, b| {
        let hta = a.hand_type as i64;
        let htb = b.hand_type as i64;

        if hta < htb {
            return Ordering::Greater;
        }

        if hta > htb {
            return Ordering::Less;
        }

        cmp_cards(a.val.as_str(), b.val.as_str(), &cards_order)
    });

    let sum: i64 = hands
        .iter()
        .enumerate()
        .map(|v| (v.0 as i64 + 1) * v.1.bid)
        .sum();

    println!("Part 2 answer: {:?}", sum);
}

fn cmp_cards(a: &str, b: &str, cards_order: &[char; 13]) -> Ordering {
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();

    loop {
        let a_char = a_chars.next();
        let b_char = b_chars.next();

        match (a_char, b_char) {
            (Some(a_char), Some(b_char)) => {
                let a_idx = cards_order.iter().position(|&c| a_char == c).unwrap_or(0);
                let b_idx = cards_order.iter().position(|&c| b_char == c).unwrap_or(0);

                if a_idx < b_idx {
                    return Ordering::Greater;
                }

                if a_idx > b_idx {
                    return Ordering::Less;
                }
            }
            _ => {
                break;
            }
        }
    }

    Ordering::Equal
}

fn parse_hand(row: &str, check_jokers: bool) -> Hand {
    let row_parts: Vec<&str> = row.split(" ").collect();

    Hand {
        val: String::from(row_parts[0]),
        bid: row_parts[1].parse::<i64>().unwrap_or(0),
        hand_type: calc_hand_type(row_parts[0], check_jokers),
    }
}

fn calc_hand_type(hand_val: &str, check_jokers: bool) -> HandType {
    let mut counts: HashMap<char, i64> = HashMap::new();

    for c in hand_val.chars() {
        *(counts.entry(c).or_default()) += 1;
    }

    let jokers_count = counts.get(&'J').unwrap_or(&0);
    let mut cards_counts = (0, 0, 0, 0);

    for e in counts.iter() {
        if check_jokers && *e.0 == 'J' {
            continue;
        }

        match *e.1 {
            5 => cards_counts.0 += 1,
            4 => cards_counts.1 += 1,
            3 => cards_counts.2 += 1,
            2 => cards_counts.3 += 1,
            _ => {}
        }
    }

    if check_jokers {
        return match (jokers_count, cards_counts) {
            (_, (1, _, _, _)) => HandType::FiveOfKind,
            (5, _) => HandType::FiveOfKind,
            (4, (0, 0, 0, 0)) => HandType::FiveOfKind,
            (3, (_, _, _, 1)) => HandType::FiveOfKind,
            (2, (_, _, 1, _)) => HandType::FiveOfKind,
            (1, (_, 1, _, _)) => HandType::FiveOfKind,

            (_, (_, 1, _, _)) => HandType::FourOfKind,
            (4, _) => HandType::FourOfKind,
            (3, (0, 0, 0, 0)) => HandType::FourOfKind,
            (2, (_, _, 0, 1)) => HandType::FourOfKind,
            (1, (_, _, 1, _)) => HandType::FourOfKind,

            (0, (_, _, 1, 1)) => HandType::FullHouse,
            (1, (_, _, 0, 2)) => HandType::FullHouse,

            (0, (0, 0, 1, 0)) => HandType::ThreeOfKind,
            (1, (0, 0, 0, 1)) => HandType::ThreeOfKind,
            (2, (0, 0, 0, 0)) => HandType::ThreeOfKind,

            (0, (0, 0, 0, 2)) => HandType::TwoPair,

            (0, (0, 0, 0, 1)) => HandType::OnePair,
            (1, (0, 0, 0, 0)) => HandType::OnePair,
            _ => HandType::HighCard,
        };
    }

    match cards_counts {
        (1, _, _, _) => HandType::FiveOfKind,
        (_, 1, _, _) => HandType::FourOfKind,
        (_, _, 1, 1) => HandType::FullHouse,
        (_, _, 1, 0) => HandType::ThreeOfKind,
        (_, _, 0, 2) => HandType::TwoPair,
        (_, _, 0, 1) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}
