use anyhow::anyhow;
use std::{io::BufRead, str::FromStr};

enum Direction {
    Forward,
    Down,
    Up,
}
use Direction::*;

struct Command {
    direction: Direction,
    amount: u32,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = line
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid command: {}", line))?;
        let direction = match direction {
            "forward" => Forward,
            "down" => Down,
            "up" => Up,
            _ => return Err(anyhow!("Bad direction: {}", direction)),
        };
        let amount = amount.parse()?;
        Ok(Command { direction, amount })
    }
}

pub fn part1(input: impl BufRead) -> anyhow::Result<String> {
    let commands: Box<[Command]> = input
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<_, _>>()?;
    let mut horizontal = 0u32;
    let mut depth = 0u32;
    for command in commands.iter() {
        match command.direction {
            Forward => horizontal += command.amount,
            Down => depth += command.amount,
            Up => depth -= command.amount,
        }
    }
    Ok(format!("{}", horizontal * depth))
}

pub fn part2(input: impl BufRead) -> anyhow::Result<String> {
    let commands: Box<[Command]> = input
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<_, _>>()?;
    let mut horizontal = 0u32;
    let mut depth = 0u32;
    let mut aim = 0i32;
    for command in commands.iter() {
        match command.direction {
            Down => aim += command.amount as i32,
            Up => aim -= command.amount as i32,
            Forward => {
                horizontal += command.amount;
                depth += (command.amount as i32 * aim) as u32;
            }
        }
    }
    Ok(format!("{}", horizontal * depth))
}
