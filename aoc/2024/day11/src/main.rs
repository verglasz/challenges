use utils::{decimals::digits, get_stdinput, Counter};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Counter<usize>;

fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    lines
        .next()
        .expect("should have one line")
        .as_ref()
        .split_whitespace()
        .map(|s| s.parse().expect("should be a number"))
        .collect()
}

fn solve1(input: &Input) -> usize {
    solve(input.clone(), 25)
}

fn solve(mut stones: Counter<usize>, steps: usize) -> usize {
    for _ in 0..steps {
        stones = step(stones);
    }
    stones.counts().sum()
}

fn step(stones: Counter<usize>) -> Counter<usize> {
    let mut new = Counter::new();
    for (&stone, &count) in stones.iter() {
        let (a, b) = evolve(stone);
        new.add(a, count);
        if let Some(b) = b {
            new.add(b, count);
        }
    }
    new
}

fn evolve(stone: usize) -> (usize, Option<usize>) {
    if stone == 0 {
        return (1, None);
    }
    let d = digits(stone);
    if d % 2 == 0 {
        let mask = 10usize.pow(d as u32 / 2);
        return (stone / mask, Some(stone % mask));
    }
    return (stone * 2024, None);
}

fn solve2(input: &Input) -> usize {
    solve(input.clone(), 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 55312);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 0);
    }
}
