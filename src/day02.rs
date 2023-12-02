use std::{str::FromStr, sync::OnceLock};

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use regex::Regex;

pub fn part1(input: &str) -> Result<String> {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let games = parse_games(input)?;
    let res = games.iter().fold(0, |acc, game| {
        let playable = game
            .rounds
            .iter()
            .all(|&(r, g, b)| r <= MAX_RED && g <= MAX_GREEN && b <= MAX_BLUE);

        if playable {
            acc + game.id
        } else {
            acc
        }
    });

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let games = parse_games(input)?;
    let res = games.iter().fold(0, |acc, game| {
        let (r, g, b) = game
            .rounds
            .iter()
            .fold((0, 0, 0), |(zr, zg, zb), &(r, g, b)| {
                (zr.max(r), zg.max(g), zb.max(b))
            });

        (r * g * b) + acc
    });

    Ok(res.to_string())
}

fn parse_games(input: &str) -> Result<Vec<Game>> {
    input.lines().map(Game::from_str).collect()
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

type Round = (usize, usize, usize);

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^Game (?P<id>\d+):").unwrap());

        let caps = re
            .captures(s)
            .ok_or_else(|| anyhow!("invalid game: {}", s))?;

        let id = caps["id"].parse()?;

        let mut rounds = vec![];
        for v1 in re.replace(s, "").split(';') {
            let mut round = Round::default();
            for v2 in v1.split(',') {
                let (num, color) = v2
                    .trim()
                    .split_once(' ')
                    .context(format!("invalid game: {s}"))?;
                let num = num.parse()?;
                match color {
                    "red" => round.0 = num,
                    "green" => round.1 = num,
                    "blue" => round.2 = num,
                    _ => bail!("invalid game: {}", s),
                }
            }

            rounds.push(round);
        }

        Ok(Game { id, rounds })
    }
}
