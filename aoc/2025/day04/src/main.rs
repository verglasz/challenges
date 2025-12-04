#![feature(ascii_char)]
#![feature(ascii_char_variants)]
use std::ascii::Char as AChar;
use utils::{
    get_stdinput,
    grid::{Dir, Point, VecMat},
};

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = VecMat<AChar>;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    let content = lines
        .filter(|l| !l.is_empty())
        .map(|x| x.as_ascii().expect("ascii").to_vec())
        .collect();
    VecMat::new(content).expect("matrix")
}

fn solve1(input: &Input) -> usize {
    let mut total = 0;
    for (p, c) in input.iter_pos() {
        if *c != AChar::CommercialAt {
            continue;
        }
        let mut close_rolls = 0;
        for dir in Dir::ALL {
            let nd = p.neighbour(dir);
            let Some(neigh) = input.get(nd) else {
                continue;
            };
            if *neigh == AChar::CommercialAt {
                close_rolls += 1;
            }
        }
        total += if close_rolls < 4 { 1 } else { 0 };
    }
    total
}

pub fn find_accessible(map: &Input) -> impl Iterator<Item = Point<usize>> + '_ {
    map.iter_pos().filter_map(|(p, c)| {
        if *c != AChar::CommercialAt {
            None?;
        }
        let mut close_rolls = 0;
        for dir in Dir::ALL {
            let nd = p.neighbour(dir);
            let Some(neigh) = map.get(nd) else {
                continue;
            };
            if *neigh == AChar::CommercialAt {
                close_rolls += 1;
            }
        }
        (close_rolls < 4).then_some(p)
    })
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
        assert_eq!(solve1(&input), 13);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 0);
    }
}
