use std::error::Error;
use std::io::BufRead;

fn get_depths() -> Result<Vec<u32>, Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut out = Vec::new();
    for line in stdin.lock().lines() {
        out.push(line?.parse()?);
    }
    return Ok(out);
}

fn part1<I>(depths: I) -> usize
where
    I: Iterator<Item = u32> + Clone,
{
    depths
        .clone()
        .zip(depths.skip(1))
        .filter(|(a, b)| a < b)
        .collect::<Vec<_>>()
        .len()
}

fn part2(depths: Vec<u32>) -> usize {
    let foo = depths.windows(3).map(|w| w.iter().sum());
    part1(foo.into_iter())
}

fn main() -> Result<(), Box<dyn Error>> {
    let depths = get_depths()?;
    println!("{}", part1(depths.clone().into_iter()));
    println!("{}", part2(depths));
    Ok(())
}
