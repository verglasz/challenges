use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use regex::bytes::Regex;
use utils::get_stdinput;

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!(
        "sol1: {p1:?}, {} things checked",
        CNT.load(Ordering::Relaxed)
    );
    CNT.store(0, Ordering::Relaxed);
    let p2 = solve2(&parsed);
    println!(
        "sol2: {p2:?}, {} things checked",
        CNT.load(Ordering::Relaxed)
    );
}
type Input = (Patterns, Designs);
type Patterns = Vec<Design>;
type Designs = Vec<Design>;
type Design = Vec<u8>;

static CNT: AtomicUsize = AtomicUsize::new(0);

fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let patterns = lines
        .next()
        .expect("patterns on fst line")
        .as_ref()
        .split(", ")
        .map(|l| l.as_bytes().to_vec().into())
        .collect();
    // debug_assert!(lines.next().expect("empty line").as_ref().is_empty());
    let designs = lines
        .filter(|l| !l.as_ref().is_empty())
        .map(|l| l.as_ref().as_bytes().to_vec().into())
        .collect();
    (patterns, designs)
}

fn solve1dp(input: &Input) -> usize {
    let (patterns, designs) = input;
    let mut feasible = HashMap::from([(b"".as_ref(), true)]);
    designs
        .iter()
        .filter(|design| {/*println!("Checked {} things", CNT.swap(0, Ordering::Relaxed)); */false} | is_feasible(&mut feasible, design, patterns))
        .count()
}

const solve1: for<'a> fn(&'a (Vec<Vec<u8>>, Vec<Vec<u8>>)) -> usize = solve1dp;

fn solve1regex(input: &Input) -> usize {
    let (patterns, designs) = input;
    let r = Regex::new(&format!(
        "^({})*$",
        patterns
            .iter()
            .map(|p| String::from_utf8_lossy(p).to_string())
            .collect::<Vec<_>>()
            .join("|")
    ))
    .expect("regex");
    designs.iter().filter(|&d| r.is_match(d)).count()
}

fn is_feasible<'a>(
    feasible: &mut HashMap<&'a [u8], bool>,
    design: &'a [u8],
    patterns: &Patterns,
) -> bool {
    if let Some(&is_feasible) = feasible.get(design) {
        return is_feasible;
    }
    // let n = CNT.fetch_add(1, Ordering::Relaxed) + 1;
    // if n % 1000_000 == 0 {
    //     println!("Checked {n} things so far...");
    // }
    let res = patterns
        .iter()
        .find_map(|pattern| {
            let rest = design.strip_prefix(pattern.as_slice())?;
            is_feasible(feasible, rest, patterns).then_some(())
        })
        .is_some();
    feasible.insert(design, res);
    res
}

fn solve2(input: &Input) -> usize {
    return 0;
    let (patterns, designs) = input;
    let mut counts = HashMap::from([(b"".as_ref(), 1)]);
    designs
        .iter()
        .map(|design| find_counts(&mut counts, design, patterns))
        .sum()
}

fn find_counts<'a>(
    counts: &mut HashMap<&'a [u8], usize>,
    design: &'a [u8],
    patterns: &Patterns,
) -> usize {
    if let Some(&count) = counts.get(design) {
        return count;
    }
    CNT.fetch_add(1, Ordering::Relaxed);
    let count = patterns
        .iter()
        .map(|pattern| {
            design
                .strip_prefix(pattern.as_slice())
                .map(|rest| find_counts(counts, rest, patterns))
                .unwrap_or(0)
        })
        .sum();
    counts.insert(design, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 6);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 16);
    }
}
