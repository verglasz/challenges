use std::str::FromStr;

use nom::{
    bytes::complete::is_not,
    character::{
        char,
        complete::{digit0, space0, space1},
    },
    multi::{separated_list0, separated_list1},
    number,
    sequence::{delimited, separated_pair},
    Parser,
};
use pathfinding::{directed::astar, prelude::bfs};
use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = Vec<Problem>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Problem {
    target: u64,
    buttons: Vec<BT>,
    jolts: Vec<JT>,
}
type BT = u64; //button type
type JT = u16; //jolt type

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    state: u64,
}

fn make_pattern(lights: impl Iterator<Item = u8>) -> u64 {
    let mut btn = 0;
    for l in lights {
        btn |= 1 << l;
    }
    btn
}

impl FromStr for Problem {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = delimited(char('['), is_not("]"), char(']'));
        let button = delimited(
            char('('),
            separated_list0(char(','), digit0.map(|x: &str| x.parse().unwrap())),
            char(')'),
        )
        .map(|x| make_pattern(x.iter().copied()));
        let jolt = delimited(
            char('{'),
            separated_list0(char(','), digit0.map(|x: &str| x.parse().unwrap())),
            char('}'),
        );
        let mut line = separated_pair(
            separated_pair(pattern, space1, separated_list1(space1, button)),
            space0,
            jolt,
        );
        let (_, ((target, buttons), jolts)) = line.parse(s)?;
        let target = make_pattern(
            target
                .bytes()
                .enumerate()
                .filter_map(|(i, x)| (x == b'#').then_some(i as u8)),
        );
        Ok(Self {
            target,
            buttons,
            jolts,
        })
    }
}

impl Problem {
    fn solve1(&self) -> usize {
        // let buttons: Vec<_> = self
        //     .buttons
        //     .iter()
        //     .map(|x| make_pattern(x.iter().copied()))
        //     .collect();

        let sol =
            bfs(&0, |x| toggle(*x, &self.buttons), |x| *x == self.target).expect("solution exists");
        sol.len() - 1
    }

    fn solve2(&self) -> usize {
        let sol = astar::astar(
            &vec![0; self.jolts.len()],
            move |x| joggle(x, &self.buttons),
            |x| heur(x, &self.jolts),
            |x| x == &self.jolts,
        )
        .expect("sol exists");
        sol.1
    }
}

fn joggle<'a, 'b: 'a>(
    x: &'b [JT],
    buttons: &'a [BT],
) -> impl Iterator<Item = (Vec<JT>, usize)> + 'a {
    let jolts = x.to_vec();
    buttons.iter().map(move |&b| (j1(b, jolts.clone()), 1))
}

fn heur(state: &[JT], target_jolts: &[JT]) -> usize {
    state
        .iter()
        .copied()
        .zip(target_jolts.iter().copied())
        .map(|(s, t)| t.checked_sub(s).unwrap_or(JT::MAX))
        .max()
        .unwrap() as usize
}

fn j1(mut bpos: u64, mut jolts: Vec<JT>) -> Vec<JT> {
    for p in jolts.iter_mut() {
        if (bpos & 1) != 0 {
            *p += 1;
        }
        bpos >>= 1;
    }
    jolts
}

fn toggle(x: u64, buttons: &[u64]) -> impl Iterator<Item = u64> {
    buttons.iter().map(move |b| x ^ b)
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve1(input: &Input) -> usize {
    input.iter().map(|x| x.solve1()).sum()
}

fn solve2(input: &Input) -> usize {
    input.iter().map(|x| x.solve2()).sum()
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
        assert_eq!(solve2(&input), 33);
    }
}
