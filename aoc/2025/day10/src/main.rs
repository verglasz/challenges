use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = Vec<Problem>;

struct Problem {
    target: u64,
    buttons: Vec<u64>,
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Problem {
    fn new(target: u64, buttons: Vec<u64>) -> Self {
        Self { target, buttons }
    }
}

fn parse<'a>(mut lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve1((start, chart): &Input) -> usize {
    let mut beams = HashSet::from([*start]);
    let mut next = HashSet::new();
    let mut total = 0;
    for line in chart {
        for &b in &beams {
            if line[b] != b'^' {
                next.insert(b);
                continue;
            }
            total += 1;
            next.insert(b - 1);
            next.insert(b + 1);
        }
        (beams, next) = (next, beams);
        next.clear();
    }
    total
}

fn solve2((start, chart): &Input) -> usize {
    let mut beams = HashMap::from([(*start, 1_usize)]);
    let mut next = HashMap::new();
    let mut total = 1;
    for line in chart {
        for (&b, &times) in beams.iter() {
            if line[b] != b'^' {
                *next.entry(b).or_default() += times;
                continue;
            }
            total += times;
            *next.entry(b - 1).or_default() += times;
            *next.entry(b + 1).or_default() += times;
        }
        (beams, next) = (next, beams);
        next.clear();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 21);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 40);
    }
}
