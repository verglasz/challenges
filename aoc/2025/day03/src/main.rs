#![feature(ascii_char)]
use std::{ascii::Char as AsciiChar, cmp::max_by_key, usize};
use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input<'a> = Vec<&'a [AsciiChar]>;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input<'a> {
    lines
        .filter(|x| !x.is_empty())
        .map(|s| s.as_ascii().expect("ascii"))
        .collect()
}

fn solve1(input: &Input) -> usize {
    input.iter().map(|x| get_max(x)).sum()
}

fn get_max(bank: &[AsciiChar]) -> usize {
    let (p1, n1) = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        .max_by_key(|x| (x.1, -(x.0 as isize)))
        .expect("nonempty bank");
    let n2 = bank[p1 + 1..].iter().max().expect("len > 2 bank");
    println!("{}: {} {}", bank.as_str(), n1.to_char(), n2.to_char());
    (n1.to_u8() - b'0') as usize * 10 + (n2.to_u8() - b'0') as usize
}

fn solve2(input: &Input) -> usize {
    input.iter().map(|x| get_max_n::<12>(x)).sum()
}

fn get_max_n<const N: usize>(mut bank: &[AsciiChar]) -> usize {
    let mut all = Vec::with_capacity(N);
    print!("{}: ", bank.as_str());
    for i in 0..N {
        let (p, n) = bank[..bank.len() + i + 1 - N]
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|x| (x.1, -(x.0 as isize)))
            .expect("bank fits");
        all.push(n);
        bank = &bank[p + 1..];
    }
    println!("{}", all.as_str());
    all.as_str().parse().expect("number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 357);
    }

    #[test]
    fn test_max() {
        assert_eq!(get_max(b"008480003".as_ascii().unwrap()), 88);
        assert_eq!(get_max(b"007430087".as_ascii().unwrap()), 87);
        assert_eq!(get_max(b"983220000".as_ascii().unwrap()), 98);
        assert_eq!(get_max(b"983220000".as_ascii().unwrap()), 98);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 3121910778619);
    }
}
