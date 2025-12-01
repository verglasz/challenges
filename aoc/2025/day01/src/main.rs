use std::ops::Rem;

use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();

    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = Vec<isize>;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|l| {
            let sign = match l.as_bytes()[0] {
                b'L' => -1,
                b'R' => 1,
                _ => panic!("line {l} starts with weird char"),
            };
            sign * l[1..].parse::<isize>().expect("number in thingy")
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut dial = 50;
    let mut total = 0;
    for i in input {
        dial = (dial + i).rem_euclid(100);
        total += if dial == 0 { 1 } else { 0 };
    }
    total
}

fn solve2(input: &Input) -> usize {
    let mut dial = 50;
    let mut total = 0;
    for i in input {
        let (i, r) = (i.rem(100), i / 100);
        let ends_past_0 = dial != 0 && i != 0 && (dial + i >= 100 || dial + i <= 0);
        total += r.abs() as usize + if ends_past_0 { 1 } else { 0 };
        dial = (dial + i).rem_euclid(100);
    }
    total
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
