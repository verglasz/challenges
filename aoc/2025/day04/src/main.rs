#![feature(ascii_char)]
#![feature(ascii_char_variants)]
use std::ascii::Char as AChar;
use utils::{
    get_stdinput,
    grid::{Dir, Point, VecMat},
};

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let mut parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&mut parsed);
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
    find_accessible(input).count()
}

pub fn find_accessible(map: &Input) -> impl Iterator<Item = Point<usize>> + '_ {
    map.iter_pos().filter_map(|(p, c)| {
        if *c != AChar::CommercialAt {
            None?;
        }
        let close = p
            .all_neighbours()
            .into_iter()
            .filter_map(|n| map.get(n))
            .filter(|&&n| n == AChar::CommercialAt)
            .count();
        (close < 4).then_some(p)
    })
}

fn solve2(input: &mut Input) -> usize {
    let mut modified = true;
    let mut total = 0;
    while modified {
        let free: Vec<_> = find_accessible(input).collect();
        modified = !free.is_empty();
        total += free.len();
        for p in free {
            input[p] = AChar::FullStop;
        }
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
        assert_eq!(solve1(&input), 13);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let mut input = parse(input.lines());
        assert_eq!(solve2(&mut input), 43);
    }
}
