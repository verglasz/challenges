use core::fmt;
use std::collections::{HashMap, HashSet};

use utils::{
    get_stdinput,
    graphs::{bron_kerbosch, max_bron_kerbosch},
};

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = ConnectionGraph;

type ConnectionGraph = HashMap<Name, HashSet<Name>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Name([u8; 2]);

impl Name {
    fn as_bytes(&self) -> &[u8; 2] {
        &self.0
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("invalid utf8")
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<[u8; 2]> for Name {
    fn from(bytes: [u8; 2]) -> Self {
        Name(bytes)
    }
}

fn parse_line(line: &str) -> (Name, Name) {
    let mut parts = line.split("-");
    let a = parts.next().expect("missing a").as_bytes();
    let b = parts.next().expect("missing b").as_bytes();
    ([a[0], a[1]].into(), [b[0], b[1]].into())
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    let mut graph = ConnectionGraph::new();
    for line in lines.filter(|s| !s.is_empty()) {
        let (a, b) = parse_line(line);
        if a == b {
            continue;
        }
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }
    graph
}

fn solve1(input: &Input) -> usize {
    // println!("{:?}", input);
    input
        .iter()
        .filter(|(name, _)| name.0[0] == b't')
        .flat_map(|(name, neighbours)| {
            neighbours.iter().enumerate().flat_map(move |(i, a)| {
                neighbours.iter().skip(i + 1).filter_map(move |b| {
                    input
                        .get(a)
                        .expect("element should be in map")
                        .contains(b)
                        .then_some({
                            let mut set = [*name, *a, *b];
                            set.sort();
                            set
                        })
                })
            })
        })
        .collect::<HashSet<_>>()
        .len()
}

fn solve2(input: &Input) -> String {
    let p = input.keys().copied().collect();
    let mut max_clique = max_bron_kerbosch(p, |n| &input[n]).expect("no cliques");
    max_clique.sort();
    max_clique
        .iter()
        .map(Name::as_str)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 7);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), "co,de,ka,ta");
    }
}
