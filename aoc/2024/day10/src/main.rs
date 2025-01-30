use std::collections::HashSet;

use utils::{get_stdinput, grid::VecMat};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = VecMat<u8>;

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let mut out: Input = lines
        .filter(|line| !line.as_ref().is_empty())
        .map(|line| line.as_ref().as_bytes().to_vec())
        .collect::<Vec<_>>()
        .try_into()
        .expect("should be a square matrix");

    out.for_each_mut(|_, c| {
        *c -= b'0';
    });
    out
}
fn solve1(input: &Input) -> usize {
    let starts: HashSet<_> = input
        .iter_pos()
        .filter_map(|(p, &c)| (c == 0).then_some(p))
        .collect();
    let mut count = 0;
    for s in starts {
        let mut layer: HashSet<_> = [s].into();
        for i in 1..=9 {
            let mut next_layer = HashSet::new();
            for p in layer.iter() {
                for n in p.neighbours() {
                    if input.get(n).copied() == Some(i) {
                        next_layer.insert(n);
                    }
                }
            }
            if next_layer.is_empty() {
                break;
            }
            if i == 9 {
                count += next_layer.len();
                break;
            }
            layer = next_layer;
        }
    }

    count
}

fn solve1x(input: &Input) -> usize {
    let mut starts = VecMat::filled(input.shape(), &0);
    let mut layer: HashSet<_> = (input
        .iter_pos()
        .filter_map(|(p, &c)| (c == 0).then_some(p))
        .collect());
    for p in layer.iter() {
        starts.set(*p, 1);
    }
    for i in 1..=9 {
        let mut next_layer = HashSet::new();
        for p in layer.iter() {
            let s = starts[*p];
            for n in p.neighbours() {
                if input.get(n).copied() == Some(i) {
                    starts[n] += s;
                    next_layer.insert(n);
                }
            }
        }
        if next_layer.is_empty() {
            break;
        }
        if i == 9 {
            break;
        }
        layer = next_layer;
    }
    layer.iter().map(|p| starts[*p]).sum()
}

fn solve2(input: &Input) -> usize {
    let starts: HashSet<_> = input
        .iter_pos()
        .filter_map(|(p, &c)| (c == 0).then_some(p))
        .collect();
    let mut count: usize = 0;
    for s in starts {
        let mut paths = VecMat::filled(input.shape(), &0);
        paths.set(s, 1);

        let mut layer: HashSet<_> = [s].into();
        for i in 1..=9 {
            let mut next_layer = HashSet::new();
            for p in layer.iter() {
                let s = paths[*p];
                for n in p.neighbours() {
                    if input.get(n).copied() == Some(i) {
                        paths[n] += s;
                        next_layer.insert(n);
                    }
                }
            }
            if next_layer.is_empty() {
                break;
            }
            if i == 9 {
                count += next_layer.iter().map(|p| paths[*p]).sum::<usize>();
                break;
            }
            layer = next_layer;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 36);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 81);
    }
}
