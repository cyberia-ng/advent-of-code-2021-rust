use anyhow::anyhow;
use std::{
    env::args,
    io::{stdin, Read},
};

mod day1;
mod day2;

fn main() -> anyhow::Result<()> {
    let mut args = args();
    args.next();
    let day = args
        .next()
        .ok_or(anyhow!("Must provide day number argument"))?;
    let day = day.parse::<usize>()?;
    let part = args
        .next()
        .ok_or(anyhow!("Must provide part number argument"))?;
    let part = part.parse::<usize>()?;

    type PartFn = dyn Fn(&str) -> anyhow::Result<String>;
    let part_fn: Box<PartFn> = Box::new(match (day, part) {
        (1, 1) => day1::part1,
        (1, 2) => day1::part2,
        (2, 1) => day2::part1,
        (2, 2) => day2::part2,
        _ => return Err(anyhow!("Invalid day/part")),
    });
    eprintln!("Reading input");
    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;
    let input = String::from_utf8(input)?;

    let output = part_fn(&input)?;
    println!("{}", output);
    Ok(())
}
