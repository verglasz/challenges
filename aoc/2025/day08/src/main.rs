use std::collections::HashMap;

use union_find::{QuickFindUf, QuickUnionUf, Union, UnionByRank, UnionBySize, UnionFind};
use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed, 1000);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = Vec<El>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct El {
    x: isize,
    y: isize,
    z: isize,
}

impl El {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
    fn delta(&self, other: &Self) -> Self {
        El {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn sqmod(&self) -> isize {
        self.x.abs() * self.x.abs() + self.y.abs() * self.y.abs() + self.z.abs() * self.z.abs()
    }

    fn sqdist(&self, other: &Self) -> isize {
        self.delta(other).sqmod()
    }
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|l| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            El { x, y, z }
        })
        .collect()
}

fn solve1(input: &Input, mut connect: usize) -> usize {
    let mut pairs: Vec<_> = (0..input.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .collect();
    pairs.sort_by_cached_key(|&(x, y)| input[x].sqdist(&input[y]));
    let mut i = 0;
    let mut uf = QuickFindUf::<UnionBySize>::new(pairs.len());
    while connect > 0 {
        let (x, y) = pairs[i];
        i += 1;
        connect -= 1;
        if uf.find(x) == uf.find(y) {
            continue;
        }
        // println!("connecting box {x} and {y}");
        uf.union(x, y);
    }
    let mut sizes: Vec<_> = group_sizes(&mut uf, input.len()).collect();
    sizes.sort();
    // println!("{sizes:?}");
    sizes.iter().rev().take(3).product()
}

fn group_sizes<T: Union>(uf: &mut QuickFindUf<T>, max: usize) -> impl Iterator<Item = usize> {
    let mut counts = HashMap::new();
    for i in 0..max {
        let root = uf.find(i);
        *counts.entry(root).or_default() += 1;
    }
    counts.into_values()
}

fn solve2(input: &Input) -> isize {
    let mut pairs: Vec<_> = (0..input.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .collect();
    pairs.sort_by_cached_key(|&(x, y)| input[x].sqdist(&input[y]));
    let mut uf = QuickUnionUf::<UnionByRank>::new(pairs.len());
    let mut last = (0, 0);
    let mut connect = 0;
    for (i, p) in pairs.iter().copied().enumerate() {
        let (x, y) = p;
        if uf.find(x) == uf.find(y) {
            continue;
        }
        // println!("connecting box {x} and {y}");
        uf.union(x, y);
        last = p;
        connect += 1;
        if connect == input.len() - 1 {
            println!("quickexit");
            println!("last {i}th of {}", pairs.len());
            break;
        }
    }
    input[last.0].x * input[last.1].x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input, 10), 40);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 25272);
    }
}
