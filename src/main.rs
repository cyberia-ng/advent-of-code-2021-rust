use anyhow::anyhow;
use std::{
    env::args,
    io::{stdin, StdinLock},
};

use advent_of_code_2021_rust::*;

fn main() -> anyhow::Result<()> {
    let mut args = args();
    args.next();
    let day = args
        .next()
        .ok_or(anyhow!("Must provide day number argument"))?;
    let day = day.parse::<u8>()?;

    let part = args
        .next()
        .ok_or(anyhow!("Must provide part number argument"))?;
    let part = part.parse::<u8>()?;

    type PartFn = Box<dyn Fn(StdinLock<'static>) -> anyhow::Result<String>>;
    let part_fn: PartFn = Box::new(match (day, part) {
        (1, 1) => day01::part1,
        (1, 2) => day01::part2,
        (2, 1) => day02::part1,
        (2, 2) => day02::part2,
        (3, 1) => day03::part1,
        (3, 2) => day03::part2,
        (4, 1) => day04::part1,
        (4, 2) => day04::part2,
        (5, 1) => day05::part1,
        (5, 2) => day05::part2,
        (6, 1) => day06::part1,
        (6, 2) => day06::part2,
        (7, 1) => day07::part1,
        (7, 2) => day07::part2,
        (8, 1) => day08::part1,
        (8, 2) => day08::part2,
        (9, 1) => day09::part1,
        (9, 2) => day09::part2,
        (10, 1) => day10::part1,
        (10, 2) => day10::part2,
        (11, 1) => day11::part1,
        (11, 2) => day11::part2,
        _ => return Err(anyhow!("Invalid day/part")),
    });

    let stdin = Box::leak(Box::new(stdin()));
    let handle = stdin.lock();
    let output = part_fn(handle)?;
    println!("{}", output);
    Ok(())
}
