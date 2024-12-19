use std::collections::{HashMap, HashSet};

use bstr::{BStr, BString};
use utils::get_stdinput;

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = (Patterns, Designs);
type Patterns = Vec<Design>;
type Designs = Vec<Design>;
type Design = Vec<u8>;

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

fn solve1(input: &Input) -> usize {
    let (patterns, designs) = input;
    let mut feasible = HashMap::from([(b"".to_vec(), true)]);
    let mut count = 0;
    for design in designs {
        if is_feasible(&mut feasible, design, patterns) {
            count += 1;
        }
    }
    count
}

fn is_feasible(feasible: &mut HashMap<Design, bool>, design: &[u8], patterns: &Patterns) -> bool {
    if let Some(&is_feasible) = feasible.get(design) {
        return is_feasible;
    }
    for pattern in patterns {
        let Some(rest) = design.strip_prefix(pattern.as_slice()) else {
            continue;
        };
        if is_feasible(feasible, rest, patterns) {
            feasible.insert(design.to_vec(), true);
            return true;
        }
    }
    // checked all possible prefixes and no luck :(
    feasible.insert(design.to_vec(), false);
    false
}

fn solve2(input: &Input) -> usize {
    let (patterns, designs) = input;
    let mut counts = HashMap::from([(b"".to_vec(), 1)]);
    let mut total = 0;
    for design in designs {
        total += find_counts(&mut counts, design, patterns);
    }
    total
}

fn find_counts(counts: &mut HashMap<Design, usize>, design: &[u8], patterns: &Patterns) -> usize {
    if let Some(&count) = counts.get(design) {
        return count;
    }
    let count = patterns
        .iter()
        .map(|pattern| {
            design
                .strip_prefix(pattern.as_slice())
                .map(|rest| find_counts(counts, rest, patterns))
                .unwrap_or(0)
        })
        .sum();
    counts.insert(design.to_vec(), count);
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
