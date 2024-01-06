use regex::Regex;
use std::fs;

fn main() {
    let data = fs::read_to_string("inputs/day_06.txt").expect("Unable to read file");
    let rows: Vec<&str> = data.trim().split("\n").collect();

    let num_re = Regex::new(r"(?<num>\d+)").expect("Unable to compile regex");

    part_1(&rows, &num_re);
    part_2(&rows, &num_re)
}

fn part_1(rows: &Vec<&str>, num_re: &Regex) {
    let times = parse_nums(num_re, rows[0]);
    let dists = parse_nums(num_re, rows[1]);

    let mut res = 1;

    for (i, time) in times.iter().enumerate() {
        res *= game_win_ways(*time, dists[i]);
    }

    println!("Part 1 answer: {:?}", res);
}

fn part_2(rows: &Vec<&str>, num_re: &Regex) {
    let time_row: String = rows[0].split_whitespace().collect();
    let dist_row: String = rows[1].split_whitespace().collect();

    let times = parse_nums(&num_re, time_row.as_str());
    let dists = parse_nums(&num_re, dist_row.as_str());

    println!("Part 2 answer: {:?}", game_win_ways(times[0], dists[0]));
}

fn game_win_ways(time: i64, dist: i64) -> i64 {
    let mut ways = 0;

    for i in 1..=time {
        if dist / i >= time - i {
            continue;
        }

        ways += 1;
    }

    ways
}

fn parse_nums(num_re: &Regex, row: &str) -> Vec<i64> {
    let mut nums = Vec::new();

    num_re.captures_iter(row).for_each(|v| {
        let num = v["num"].parse::<i64>().unwrap_or(0);
        nums.push(num);
    });

    nums
}
