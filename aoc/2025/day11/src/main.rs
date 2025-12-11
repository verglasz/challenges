#![feature(ascii_char)]
use std::{ascii::Char, collections::HashMap, ops::Rem};

use topo_sort::TopoSort;
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

fn get_paths(input: &Input, start: Name, end: Name, skip: &[Name]) -> usize {
    let mut ts = TopoSort::with_capacity(input.len());
    for (s, t) in input.iter() {
        ts.insert_from_slice(*s, t);
    }
    let mut values = HashMap::with_capacity(input.len());
    values.insert(start, 1);
    let ordered = ts.try_into_vec_nodes().expect("no cycle");
    for node in ordered.into_iter().rev() {
        for tgt in &input[&node] {
            if skip.contains(tgt) {
                continue;
            }
            *values.entry(*tgt).or_default() += values.get(&node).copied().unwrap_or(0);
        }
    }

    values.get(&end).copied().unwrap_or(0)
}

fn solve1(input: &Input) -> usize {
    const YOU: Name = *b"you".as_ascii().unwrap();
    const OUT: Name = *b"out".as_ascii().unwrap();
    get_paths(input, YOU, OUT, &[])
}

fn solve2(input: &Input) -> usize {
    const SVR: Name = *b"svr".as_ascii().unwrap();
    const FFT: Name = *b"fft".as_ascii().unwrap();
    const DAC: Name = *b"dac".as_ascii().unwrap();
    const OUT: Name = *b"out".as_ascii().unwrap();
    let a = {
        get_paths(input, SVR, FFT, &[DAC, OUT])
            * get_paths(input, FFT, DAC, &[SVR, OUT])
            * get_paths(input, DAC, OUT, &[SVR, FFT])
    };
    let b = {
        get_paths(input, SVR, DAC, &[FFT, OUT])
            * get_paths(input, DAC, FFT, &[SVR, DAC])
            * get_paths(input, DAC, OUT, &[SVR, FFT])
    };
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 5);
    }
    #[test]
    fn test1i() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 782);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test2");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 2);
    }
}
