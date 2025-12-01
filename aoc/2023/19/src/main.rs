use std::{collections::HashMap, io::stdin, ops::Deref};

use crate::workflow::{parse_label, Block, Bound, Entry, KeepLast, Step, Workflows, START};

mod entry {}
mod workflow;

fn main() {
    let lines = stdin().lines().map(|line| line.unwrap());
    let input = parse_input(lines);
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

type Input = (Workflows, Vec<Entry>);

fn parse_input<T: Deref<Target = str>>(mut lines: impl Iterator<Item = T>) -> Input {
    let mut workflows = HashMap::new();
    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        let (lab, rest) = l.split_once('{').expect("No { in workflow input");
        let label = parse_label(lab);
        let rest = rest.trim_end_matches('}');
        let pieces = &mut KeepLast::new(rest.split(',')).expect("No pieces in workflow input");
        let mut steps: Vec<_> = pieces.map(Step::parse).collect();
        let last = pieces.last().expect("last?");
        steps.push(Step {
            prop: b'x',
            bound: Bound::Upper(u16::MAX),
            dest: parse_label(last),
        });

        workflows.insert(label, steps.into());
    }
    let entries = lines
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(Entry::parse(&l))
            }
        })
        .collect();
    (workflows.into(), entries)
}

fn part1((workflows, entries): &(Workflows, Vec<Entry>)) -> usize {
    entries.iter().filter_map(|&e| workflows.accept(e)).sum()
}

fn part2<T>((workflows, _): &(Workflows, T)) -> usize {
    let possible = Block {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };
    workflows.tree(START, possible)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_p2() {
        let input = include_str!("../test");
        let parsed = super::parse_input(input.lines());
        let result = super::part2(&parsed);
        assert_eq!(result, 19114);
    }

    fn test_p1() {
        let input = include_str!("../test");
        let parsed = super::parse_input(input.lines());
        let result = super::part2(&parsed);
        assert_eq!(result, 167409079868000);
    }
}
