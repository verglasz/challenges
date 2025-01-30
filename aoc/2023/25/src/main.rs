use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{self, Debug, Display, Formatter},
    io::stdin,
    ops::Deref,
};

use pathfinding::directed::bfs;

fn main() {
    let lines = stdin().lines().map(|line| line.unwrap());
    let input = parse_input(lines);
    eprintln!("input: {:?}", input);
    eprintln!("part 1: {}", part1(&input));
    eprintln!("part 2: {}", part2(&input));
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Label([u8; 3]);

impl Debug for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

type Input = HashMap<Label, HashSet<Label>>;

fn parse_input<T: Deref<Target = str>>(mut lines: impl Iterator<Item = T>) -> Input {
    let mut input: Input = HashMap::new();
    for line in &mut lines {
        if line.is_empty() {
            continue;
        }
        let (src, dest) = line.split_once(": ").expect("input doesn't contain :");
        let label = Label(src.as_bytes().try_into().expect("label is not 3 bytes"));
        let dests: HashSet<_> = dest
            .split(' ')
            .map(|dest| Label(dest.as_bytes().try_into().expect("label is not 3 bytes")))
            .collect();
        for d in &dests {
            match input.entry(*d) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(label);
                }
                Entry::Vacant(entry) => {
                    entry.insert(HashSet::from([label]));
                }
            }
        }
        match input.entry(label) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().extend(dests);
            }
            Entry::Vacant(entry) => {
                entry.insert(dests);
            }
        }
    }
    input
}

fn to_dot(input: &Input) -> String {
    let mut dot = String::new();
    dot.push_str("graph {\n");
    for (src, dests) in input {
        for dest in dests {
            dot.push_str(&format!("  {} -- {};\n", src, dest));
        }
    }
    dot.push_str("}\n");
    dot
}

fn part1(input: &Input) -> usize {
    let side1 = [Label(*b"fch"), Label(*b"vfj"), Label(*b"jbz")];
    let side2 = [Label(*b"fvh"), Label(*b"nvg"), Label(*b"sqh")];
    for l1 in &side1 {
        println!("{}: {:?}", l1, input.get(l1));
    }
    for l2 in &side2 {
        println!("{}: {:?}", l2, input.get(l2));
    }
    let fst = input.iter().next().unwrap().0;
    let graph = {
        let mut g = input.clone();
        for l1 in &side1 {
            for l2 in &side2 {
                println!("removing {} -- {}", l1, l2);
                dbg!(g.get_mut(l1)).unwrap().remove(l2);
                dbg!(g.get_mut(l2)).unwrap().remove(l1);
            }
        }
        g
    };
    let size = graph.len();
    let fst = *graph.iter().next().unwrap().0;
    let mut piece1 = bfs::bfs_reach(fst, |node| graph.get(node).unwrap().iter().copied()).count();
    let piece2 = size - piece1;

    println!("{piece1} x {piece2}");
    piece1 * piece2
}

fn part2(input: &Input) -> usize {
    // println!("{}", to_dot(input));

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test");
        let input = parse_input(input.lines());
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test");
        let input = parse_input(input.lines());
        assert_eq!(part1(&input), 0);
    }
}
