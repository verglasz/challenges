#![feature(ascii_char)]
use std::{ascii::Char, collections::HashMap, ops::Rem};

use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();

    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = HashMap<Name, Vec<Name>>;

type Name = [Char; 3];
fn parse_name(name: &[Char]) -> Name {
    match name {
        &[a, b, c] => [a, b, c],
        x => todo!("'{}' as a name", name.as_str()),
    }
}

fn parse_line(line: &str) -> (Name, Vec<Name>) {
    let (name, targets) = line.split_once(':').expect("colon");
    let n = parse_name(name.trim().as_ascii().unwrap());

    let tgts = targets
        .split_whitespace()
        .map(|x| parse_name(x.trim().as_ascii().unwrap()))
        .collect();
    (n, tgts)
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines.filter(|x| !x.is_empty()).map(parse_line).collect()
}

fn solve1(input: &Input) -> usize {
    0
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
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 6);
    }
}
