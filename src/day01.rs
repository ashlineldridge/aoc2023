use anyhow::{bail, Result};
use std::sync::OnceLock;

pub fn part1(input: &str) -> Result<String> {
    let res = input.lines().try_fold(0, |z, line| -> Result<u32> {
        let h = parse_num(line, ParseDirection::LeftRight, true)?;
        let t = parse_num(line, ParseDirection::RightLeft, true)?;

        Ok(z + h * 10 + t)
    })?;

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let res = input.lines().try_fold(0, |z, line| -> Result<u32> {
        let h = parse_num(line, ParseDirection::LeftRight, false)?;
        let t = parse_num(line, ParseDirection::RightLeft, false)?;

        Ok(z + h * 10 + t)
    })?;

    Ok(res.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ParseDirection {
    LeftRight,
    RightLeft,
}

fn parse_num(line: &str, dir: ParseDirection, use_digit_names: bool) -> Result<u32> {
    if line.is_empty() {
        bail!("invalid input")
    }

    for &(digit, digit_name) in digit_names() {
        let found = match dir {
            ParseDirection::LeftRight => {
                if line.starts_with(digit.to_string().as_str()) {
                    true
                } else if use_digit_names {
                    line.starts_with(digit_name)
                } else {
                    false
                }
            }
            ParseDirection::RightLeft => {
                if line.ends_with(digit.to_string().as_str()) {
                    true
                } else if use_digit_names {
                    line.ends_with(digit_name)
                } else {
                    false
                }
            }
        };

        if found {
            return Ok(digit);
        }
    }

    let line = if dir == ParseDirection::LeftRight {
        &line[1..]
    } else {
        &line[..line.len() - 1]
    };

    parse_num(line, dir, use_digit_names)
}

// This doesn't actually require `OnceLock` - I was gonna use a regex but CBF.
fn digit_names() -> &'static [(u32, &'static str); 9] {
    static DIGITS: OnceLock<[(u32, &str); 9]> = OnceLock::new();
    DIGITS.get_or_init(|| {
        [
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
        ]
    })
}
