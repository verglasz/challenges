mod utils;
use std::ops::RangeInclusive;

use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}

type Ru = RangeInclusive<usize>;
type Input = (Vec<Ru>, Vec<usize>);

fn parse<'a>(mut lines: impl Iterator<Item = &'a str>) -> Input {
    let mut ranges: Vec<_> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').expect("pair with -");
            let start: usize = a.parse().expect("number range start");
            let end = b.parse().expect("number range end");
            start..=end
        })
        .collect();
    let mut items: Vec<_> = lines
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect("number item"))
        .collect();
    ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()).then(a.end().cmp(b.end())));
    items.sort_unstable();
    (merge_overlapping(ranges), items)
}

/// Merge consecutive overlapping ranges into the union of the ranges.
/// Assumes the input is sorted lexicographically (by start then end)
fn merge_overlapping(ranges: Vec<Ru>) -> Vec<Ru> {
    let mut merged = vec![];
    let mut current = ranges[0].clone();
    for next in ranges.into_iter().skip(1) {
        if next.start() <= current.end() {
            // mergeable
            let end = current.end().max(next.end());
            current = *current.start()..=*end;
        } else {
            // end of mergeable window
            merged.push(current);
            current = next;
        }
    }
    merged.push(current);
    merged
}

/// merge two ranges if they're overlapping
fn merge(a: &Ru, b: &Ru) -> Option<Ru> {
    (b.start() <= a.end() && a.start() <= b.end())
        .then_some(*a.start().min(b.start())..=*a.end().max(b.end()))
}

fn solve1((ranges, items): &Input) -> usize {
    items
        .iter()
        .filter(|&i| ranges.iter().any(|r| r.contains(i)))
        .count()
}

fn solve1better((ranges, items): &Input) -> usize {
    let mut ranges = &ranges[..];
    let mut nfresh = 0;
    for item in items {
        // this could also be a binsearch
        let Some(r) = ranges.iter().position(|r| r.start() >= item) else {
            break;
        };
        ranges = &ranges[r..];
        nfresh += if ranges[0].end() >= item { 1 } else { 0 };
    }
    nfresh
}

fn solve2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 3);
    }

    #[test]
    fn test1b() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1better(&input), 3);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 0);
    }
}
