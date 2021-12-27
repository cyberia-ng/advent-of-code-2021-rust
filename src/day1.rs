pub fn part1(input: &str) -> anyhow::Result<String> {
    let depths: Box<[u32]> = input
        .lines()
        .map(|line| line.parse::<_>())
        .collect::<Result<_, _>>()?;
    let increases = depths
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    Ok(format!("{}", increases))
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let depths: Box<[u32]> = input
        .lines()
        .map(|line| line.parse::<_>())
        .collect::<Result<_, _>>()?;
    let window_sums: Box<[u32]> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let increases = window_sums
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    Ok(format!("{}", increases))
}
