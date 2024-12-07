use std::collections::HashSet;

use utils::{
    get_stdinput,
    grid::{Dir, Point, VecMat},
};

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
    let data = lines
        .filter(|s| !s.as_ref().trim().is_empty())
        .map(|l| l.as_ref().trim().as_bytes().to_vec())
        .collect();
    Input::new(data).expect("input should be grid map")
}

fn solve1(input: &Input) -> usize {
    let mut input = input.clone();
    let mut pos = input.find(&b'^').expect("should have a start point");
    *input.get_mut(pos).expect("start should be in bounds") = b'X';
    let mut dir = Dir::N;
    loop {
        let next = pos.wrapping_add_signed(dir.to_delta());
        match input.get(next) {
            None => {
                // oob
                break;
            }
            Some(b'#') => {
                // turn from obstacle
                dir = dir.clockwise_cross();
                continue;
            }
            Some(b'.') | Some(b'X') => {
                // advance
                pos = next;
                *input.get_mut(pos).expect("next should be in bounds") = b'X';
                continue;
            }
            _ => panic!("invalid input"),
        }
    }
    input.iter_all().filter(|&&c| c == b'X').count()
}

#[allow(dead_code)]
fn marker(dir: Dir) -> u8 {
    match dir {
        Dir::N => b'^',
        Dir::E => b'>',
        Dir::S => b'v',
        Dir::W => b'<',
        _ => panic!("invalid direction"),
    }
}

type ObstHits = HashSet<(Point<usize>, Dir)>;

fn is_loop(input: &Input, mut pos: Point<usize>, mut dir: Dir, mut hits: ObstHits) -> bool {
    // let mut hits = HashSet::new();
    loop {
        let next = pos.wrapping_add_signed(dir.to_delta());
        match input.get(next) {
            None => {
                // oob
                break;
            }
            Some(b'#') => {
                // obstacle, see if already hit
                if !hits.insert((next, dir)) {
                    // already hit, it's a loop
                    return true;
                }
                dir = dir.clockwise_cross();
                continue;
            }
            Some(b'.') | Some(b'X') => {
                // advance
                pos = next;
                continue;
            }
            _ => panic!("invalid input"),
        }
    }
    // we went OOB
    false
}

fn solve2(input: &Input) -> usize {
    solve2_a(input)
}

#[allow(dead_code)]
fn solve2_b(input: &Input) -> usize {
    let mut input = input.clone();
    let start = input.find(&b'^').expect("should have a start point");
    *input.get_mut(start).expect("start should be in bounds") = b'.';
    let mut loops = 0;
    for i in 0..input.rows() {
        for j in 0..input.cols() {
            let cell = input.get_mut(Point::new(j, i)).unwrap();
            if *cell != b'#' {
                // if not an obstacle, try putting an obstacle and checking for loops
                *cell = b'#';
                if is_loop(&input, start, Dir::N, Default::default()) {
                    loops += 1;
                }
                // reset
                *input.get_mut(Point::new(j, i)).unwrap() = b'.';
            }
        }
    }
    loops
}

fn solve2_a(input: &Input) -> usize {
    let mut input = input.clone();
    let mut pos = input.find(&b'^').expect("should have a start point");
    *input.get_mut(pos).expect("start should be in bounds") = b'X';
    let mut dir = Dir::N;
    let mut loops = HashSet::new();
    let mut collisions = HashSet::new(); // keep the collisions so far
                                         // to shorten the time we need to spend
                                         // retracing the path while checking for loops
                                         // (it does improve perf)
    loop {
        let next = pos.wrapping_add_signed(dir.to_delta());
        match input.get(next) {
            None => {
                // oob
                break;
            }
            Some(b'#') => {
                // turn from obstacle and record hit
                collisions.insert((next, dir));
                dir = dir.clockwise_cross();
            }
            Some(b'X') => {
                // already visited (and presumably tested), advance
                pos = next;
                *input.get_mut(pos).expect("next should be in bounds") = b'X';
            }
            Some(b'.') => {
                // we could walk forward, see if we can turn this into a loop
                // by putting an obstacle there
                *input.get_mut(next).expect("next should be in bounds") = b'#';
                if is_loop(&input, pos, dir, collisions.clone()) {
                    loops.insert(next);
                }
                // after we checked if it's a loop, we go forward and continue normally
                pos = next;
                *input.get_mut(next).expect("next should be in bounds") = b'X';
                continue;
            }
            _ => panic!("invalid input"),
        }
    }
    // for p in &loops {
    //     *input.get_mut(*p).expect("loop should be in bounds") = b'O';
    // }
    loops.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 41);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 6);
    }

    #[test]
    fn testinput1() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 5318);
    }

    #[test]
    fn testinput2() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 1831);
    }
}
