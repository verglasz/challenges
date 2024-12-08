use std::collections::{hash_map::Entry, HashMap, HashSet};

use utils::{
    get_stdinput,
    grid::{Point, VecMat},
};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = (Shape, Vec<Station>);
type Shape = (usize, usize);
type Station = (Point<usize>, u8);

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let map: VecMat<u8> = lines
        .filter(|s| !s.as_ref().trim().is_empty())
        .map(|l| l.as_ref().trim().as_bytes().to_vec())
        .collect::<Vec<_>>()
        .try_into()
        .expect("input should be grid map");
    (
        map.shape(),
        map.iter_pos()
            .filter_map(|(p, &s)| (s != b'.').then_some((p, s)))
            .collect(),
    )
}

fn solve1(input: &Input) -> usize {
    let &(shape, ref s_pos) = input;
    let mut stations = HashMap::<_, Vec<_>>::new();
    let mut antinodes = HashSet::new();
    for &(p, c) in s_pos {
        if c == b'.' {
            continue;
        }
        let mut entry = match stations.entry(c) {
            Entry::Occupied(e) => e,
            Entry::Vacant(e) => {
                e.insert(vec![p]);
                continue;
            }
        };
        let others = entry.get_mut();
        for &o in &*others {
            let delta = p.delta_to(o).expect("reasonable distances");
            let a1 = o.wrapping_add_signed(delta);
            if a1.in_bounds(shape) {
                antinodes.insert(a1);
            }
            let a2 = p.wrapping_add_signed(-delta);
            if a2.in_bounds(shape) {
                antinodes.insert(a2);
            }
        }
        others.push(p);
    }
    antinodes.len()
}

fn solve2(input: &Input) -> usize {
    let &(shape, ref s_pos) = input;
    let mut stations = HashMap::<_, Vec<_>>::new();
    let mut antinodes = HashSet::new();
    for &(p, c) in s_pos {
        if c == b'.' {
            continue;
        }
        let mut entry = match stations.entry(c) {
            Entry::Occupied(e) => e,
            Entry::Vacant(e) => {
                e.insert(vec![p]);
                continue;
            }
        };
        let others = entry.get_mut();
        for &o in &*others {
            let delta = p.delta_to(o).expect("reasonable distances");
            let mut antinode1 = p;
            while antinode1.in_bounds(shape) {
                antinodes.insert(antinode1);
                antinode1 = antinode1.wrapping_add_signed(delta);
            }
            let mut antinode2 = p;
            while antinode2.in_bounds(shape) {
                antinodes.insert(antinode2);
                antinode2 = antinode2.wrapping_add_signed(-delta);
            }
        }
        others.push(p);
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 14);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 34);
    }

    #[test]
    fn input1() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 413);
    }

    #[test]
    fn input2() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 1417);
    }
}
