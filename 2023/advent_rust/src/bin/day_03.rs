use std::fs;

struct PartNumber {
    value: u64,
    is_part: bool,
    x_start: isize,
    x_end: isize,
}

fn build_part_number(digits: &str, j: usize) -> PartNumber {
    PartNumber {
        value: digits.parse().unwrap(),
        is_part: false,
        x_start: (j - digits.len()) as isize,
        x_end: (j - 1) as isize,
    }
}

impl PartNumber {
    fn is_adjacent(&self, j: isize) -> bool {
        self.x_start - 1 <= j && j <= self.x_end + 1
    }
}

fn main() {
    let data = fs::read_to_string("inputs/day_03.txt").expect("Unable to read file");
    let rows: Vec<&str> = data.trim().split("\n").collect();

    let mut part_numbers: Vec<Vec<PartNumber>> = Vec::new();
    let mut digits = String::new();

    for (i, row) in rows.iter().enumerate() {
        part_numbers.push(Vec::new());
        digits.clear();

        for (j, c) in row.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.push(c);

                if j < row.len() - 1 {
                    continue;
                }
            }

            if digits.len() > 0 {
                part_numbers[i].push(build_part_number(digits.as_str(), j));
                digits.clear()
            }
        }
    }

    let mut sum = 0;

    for (i, row) in rows.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c.is_ascii_digit() || c == '.' {
                continue;
            }

            let mut adj_count = 0;
            let mut adj_prod = 1;

            for ii in i - 1..=i + 1 {
                for k in 0..part_numbers[ii].len() {
                    let pn = &mut part_numbers[ii][k];
                    if pn.is_adjacent(j as isize) {
                        pn.is_part = true;
                        adj_count += 1;
                        adj_prod *= pn.value;
                    }
                }
            }

            if c == '*' && adj_count == 2 {
                sum += adj_prod;
            }
        }
    }

    println!("{:?}", sum);
}
