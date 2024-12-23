use std::collections::{HashMap, HashSet};

use utils::get_stdinput;

fn main() {
    let input = get_stdinput().collect::<Vec<_>>();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Vec<usize>;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect("entries must be numbers"))
        .collect()
}

const PRUNE: usize = 16777216;

fn evolve(mut x: usize, n: usize) -> usize {
    for _ in 0..n {
        x = step(x);
    }
    x
}

fn step(mut x: usize) -> usize {
    x = ((x * 64) ^ x) % PRUNE;
    x = ((x / 32) ^ x) % PRUNE;
    x = ((x * 2048) ^ x) % PRUNE;
    x
}

fn solve1(input: &Input) -> usize {
    input.iter().copied().map(|x| evolve(x, 2000)).sum()
}

fn delta(n: usize, rounds: usize) -> HashMap<[i8; 4], u8> {
    let mut deltas = HashMap::new();
    let prices: Vec<_> = (0..=rounds)
        .scan(n, |x, _| {
            let price = *x % 10;
            *x = step(*x);
            Some(price as u8)
        })
        .collect();
    debug_assert_eq!(prices.len(), rounds + 1);
    let diffs: Vec<_> = prices.windows(2).map(|w| w[1] as i8 - w[0] as i8).collect();
    for (i, w) in diffs.windows(4).enumerate() {
        let key = [w[0], w[1], w[2], w[3]];
        deltas.entry(key).or_insert(prices[i + 4]);
    }
    deltas
}

fn solve2(input: &Input) -> usize {
    let deltas: Vec<_> = input.iter().copied().map(|n| delta(n, 2000)).collect();
    let ds: HashSet<_> = deltas.iter().flat_map(|d| d.keys().copied()).collect();
    let mut max = 0;
    for d in ds {
        let gain = deltas
            .iter()
            .map(|x| x.get(&d).map(|n| *n as usize).unwrap_or(0))
            .sum();
        max = max.max(gain);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 37327623);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test2");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 23);
    }
}
