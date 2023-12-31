use regex::Regex;
use std::cmp;
use std::fs;

#[derive(Debug)]
struct SeedMapEntry {
    offset: i64,
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    end: i64,
}

fn main() {
    let data = fs::read_to_string("inputs/day_05.txt").expect("Unable to read file");
    let rows: Vec<&str> = data.trim().split("\n").collect();

    let num_re = Regex::new(r"(?<num>\d+)").expect("Unable to compile regex");

    let mut seeds = parse_seeds(&num_re, rows[0]);
    let seed_maps = parse_seed_maps(&num_re, &rows);

    for seed_map in seed_maps.iter() {
        seeds = apply_seed_map(&mut seeds, seed_map);
    }

    println!("{:?}", seeds.iter().map(|s| s.start).min().unwrap_or(0));
}

fn apply_seed_map(
    seeds_range: &mut Vec<SeedRange>,
    seed_map: &Vec<SeedMapEntry>,
) -> Vec<SeedRange> {
    let mut result = Vec::new();

    let mut i = 0;

    loop {
        if i >= seeds_range.len() {
            break;
        }

        let seed = SeedRange {
            start: seeds_range[i].start,
            end: seeds_range[i].end,
        };

        i += 1;

        let mut mapped = false;

        for e in seed_map.iter() {
            if e.end < seed.start || seed.end < e.start {
                continue;
            }

            let max_start = cmp::max(seed.start, e.start);
            let min_end = cmp::min(seed.end, e.end);

            if seed.start < max_start {
                seeds_range.push(SeedRange {
                    start: seed.start,
                    end: max_start - 1,
                });
            }

            result.push(SeedRange {
                start: max_start + e.offset,
                end: min_end + e.offset,
            });

            if seed.end > min_end {
                seeds_range.push(SeedRange {
                    start: min_end + 1,
                    end: seed.end,
                });
            }

            mapped = true;
            break;
        }

        if !mapped {
            result.push(SeedRange {
                start: seed.start,
                end: seed.end,
            });
        }
    }

    result
}

fn parse_seed_maps(num_re: &Regex, rows: &Vec<&str>) -> Vec<Vec<SeedMapEntry>> {
    let mut seed_maps = Vec::new();
    let mut seed_map = Vec::new();

    for (i, row) in rows.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if row.contains("map") && seed_map.len() > 0 {
            seed_maps.push(seed_map);
            seed_map = Vec::new();
        }

        let nums = parse_nums(&num_re, row);

        if nums.len() != 3 {
            continue;
        }

        seed_map.push(SeedMapEntry {
            offset: nums[0] - nums[1],
            start: nums[1],
            end: nums[1] + nums[2] - 1,
        })
    }

    if seed_map.len() > 0 {
        seed_maps.push(seed_map);
    }

    seed_maps
}

fn parse_seeds(num_re: &Regex, row: &str) -> Vec<SeedRange> {
    let seeds_ranges = parse_nums(&num_re, row);
    let mut result = Vec::new();

    for i in (0..seeds_ranges.len()).step_by(2) {
        result.push(SeedRange {
            start: seeds_ranges[i],
            end: seeds_ranges[i] + seeds_ranges[i + 1] - 1,
        });
    }

    result
}

fn parse_nums(num_re: &Regex, row: &str) -> Vec<i64> {
    let mut nums = Vec::new();

    num_re.captures_iter(row).for_each(|v| {
        let num = v["num"].parse::<i64>().unwrap_or(0);
        nums.push(num);
    });

    nums
}
