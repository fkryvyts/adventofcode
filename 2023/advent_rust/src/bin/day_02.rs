use std::error::Error;
use std::fs;

use regex::Regex;

struct Game {
    id: u64,
    sets: Vec<GameSet>,
}

struct GameSet {
    r: u64,
    g: u64,
    b: u64,
}

fn main() {
    let game_re = Regex::new(r"Game (?<id>\d+)").expect("Unable to compile regex");
    let color_re = Regex::new(r"(?<val>\d+) (?<color>\w+)").expect("Unable to compile regex");

    let data = fs::read_to_string("inputs/day_02.txt").expect("Unable to read file");
    let games: Result<Vec<Game>, _> = data
        .trim()
        .split("\n")
        .map(|s| parse_game(&game_re, &color_re, s))
        .collect();
    let res = games.expect("Unable to parse games");

    let s: u64 = res
        .iter()
        .map(|g| {
            let max_red = g.sets.iter().map(|gs| gs.r).max().unwrap_or(0);
            let max_green = g.sets.iter().map(|gs| gs.g).max().unwrap_or(0);
            let max_blue = g.sets.iter().map(|gs| gs.b).max().unwrap_or(0);

            max_red * max_green * max_blue
        })
        .sum();

    println!("{:?}", s);
}

fn parse_game(game_re: &Regex, color_re: &Regex, line: &str) -> Result<Game, Box<dyn Error>> {
    let mut game = Game {
        id: 0,
        sets: Vec::new(),
    };

    let game_parts: Vec<String> = line.split(":").map(str::to_string).collect();

    let Some(capt) = game_re.captures(game_parts[0].as_str()) else {
        return Ok(game);
    };

    game.id = capt["id"].parse::<u64>()?;
    let sets: Result<Vec<GameSet>, _> = game_parts[1]
        .split(";")
        .map(str::to_string)
        .map(|s| parse_gameset(color_re, s))
        .collect();

    game.sets = sets?;

    return Ok(game);
}

fn parse_gameset(color_re: &Regex, line: String) -> Result<GameSet, Box<dyn Error>> {
    let mut game_set = GameSet { r: 0, g: 0, b: 0 };

    color_re.captures_iter(line.as_str()).for_each(|v| {
        let val = v["val"].parse::<u64>();

        match val {
            Ok(val) => match &v["color"] {
                "red" => {
                    game_set.r = val;
                }
                "green" => {
                    game_set.g = val;
                }
                "blue" => {
                    game_set.b = val;
                }
                _ => {}
            },
            _ => {}
        }
    });

    return Ok(game_set);
}
