use anyhow::anyhow;
use std::{
    env::args,
    io::{stdin, StdinLock},
};

mod day1;
mod day2;
mod day3;

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
        (1, 1) => day1::part1,
        (1, 2) => day1::part2,
        (2, 1) => day2::part1,
        (2, 2) => day2::part2,
        (3, 1) => day3::part1,
        _ => return Err(anyhow!("Invalid day/part")),
    });

    let stdin = Box::leak(Box::new(stdin()));
    let handle = stdin.lock();
    let output = part_fn(handle)?;
    println!("{}", output);
    Ok(())
}
