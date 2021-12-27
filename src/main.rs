use anyhow::anyhow;
use std::{
    env::args,
    io::{stdin, Read},
};

mod day1;

fn main() -> anyhow::Result<()> {
    let days: Vec<[Box<dyn Fn(&str) -> anyhow::Result<String>>; 2]> =
        vec![[Box::new(day1::part1), Box::new(day1::part2)]];

    let mut args = args();
    args.next();
    let day = args
        .next()
        .ok_or(anyhow!("Must provide day number argument"))?;
    let day = day.parse::<usize>()?;
    let day_index = day.checked_sub(1).ok_or(anyhow!("Days start at 1"))?;
    let part = args
        .next()
        .ok_or(anyhow!("Must provide part number argument"))?;
    let part = part.parse::<usize>()?;
    let part_index = part.checked_sub(1).ok_or(anyhow!("Parts start at 1"))?;

    let day_fn = days
        .get(day_index)
        .ok_or(anyhow!("No such day"))?
        .get(part_index)
        .ok_or(anyhow!("No such part"))?;

    eprintln!("Reading input");
    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;
    let input = String::from_utf8(input)?;

    let output = day_fn(&input)?;
    println!("{}", output);
    Ok(())
}
