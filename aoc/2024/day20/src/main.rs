use std::collections::HashSet;

use utils::{
    get_stdinput,
    grid::{Dir, Point, VecMat},
};

fn main() {
    let input = get_stdinput().collect::<Vec<_>>();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = (Point<usize>, VecMat<CellState>, Point<usize>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellState {
    Empty,
    Wall,
    Pathed,
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    use CellState::*;
    let mut start = None;
    let mut end = None;
    let map = lines
        .filter(|x| !x.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &c)| {
                    // println!("{} {}: {}", x, y, c as char);
                    match c {
                        b'#' => Wall,
                        b'.' => Empty,
                        b'S' => {
                            let p = Point::new(x, y);
                            assert!(start.replace(p).is_none(), "multiple start points");
                            Empty
                        }
                        b'E' => {
                            let p = Point::new(x, y);
                            assert!(end.replace(p).is_none(), "multiple end points");
                            Empty
                        }
                        _ => panic!("invalid char"),
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    (
        start.expect("no start point"),
        map.try_into().expect("map should be rectangular"),
        end.expect("no end point"),
    )
}

fn solve1(input: &Input) -> usize {
    solve1n(input, 100)
}

fn solve1n(input: &Input, threshold: usize) -> usize {
    let (start, map, end) = input;
    let mut map = map.clone();
    let mut p = *start;
    let mut cheat_times = VecMat::filled_with(map.shape(), |_| vec![]);
    let mut cheats = 0;
    let mut time = 0;
    while p != *end {
        // mark current cell as visited
        map[p] = CellState::Pathed;
        // check all directions
        let mut step = p;
        for next in p.neighbours() {
            match map.get(next) {
                None => continue,
                Some(CellState::Pathed) => continue,
                Some(CellState::Empty) => {
                    step = next;
                    continue;
                }
                Some(CellState::Wall) => {}
            };
            // now check if we can cheat from there...
            for cheat_dest in next.neighbours() {
                if map.get(cheat_dest).copied() == Some(CellState::Empty) {
                    // this cell is reachable by cheating in two ps
                    cheat_times[cheat_dest].push(time + 2);
                }
            }
        }
        // accumulate the cheats that got to this cell
        for t in cheat_times[p].iter().copied() {
            let saved = time - t;
            if saved >= threshold {
                cheats += 1;
            }
        }
        debug_assert_ne!(step, p, "no path found before end..");
        // move to the next cell
        time += 1;
        p = step;
    }
    // accumulate end cheats
    for t in cheat_times[p].iter().copied() {
        let saved = time - t;
        if saved >= threshold {
            cheats += 1;
        }
    }
    cheats
}

fn solve2(input: &Input) -> () {}

fn solve2n(input: &Input, threshold: usize) -> usize {
    let (start, map, end) = input;
    let mut map = map.clone();
    let mut p = *start;
    let mut cheat_times = VecMat::filled_with(map.shape(), |_| vec![]);
    let mut cheats = 0;
    let mut time = 0;
    while p != *end {
        // mark current cell as visited
        map[p] = CellState::Pathed;
        // check the next normal step
        let mut step = p;
        // now check what we can do by starting to cheat here!
        let mut visited = HashSet::from([p]);
        let mut queue = vec![p];
        while let Some(next) = queue.pop() {
            let xxx;
            match map.get(next) {
                None => continue,
                Some(CellState::Pathed) => continue,
                Some(CellState::Empty) => {
                    step = next;
                    continue;
                }
                Some(CellState::Wall) => {}
            };
        }
        // accumulate the cheats that got to this cell
        for t in cheat_times[p].iter().copied() {
            let saved = time - t;
            if saved >= threshold {
                cheats += 1;
            }
        }
        debug_assert_ne!(step, p, "no path found before end..");
        // move to the next cell
        time += 1;
        p = step;
    }
    // accumulate end cheats
    for t in cheat_times[p].iter().copied() {
        let saved = time - t;
        if saved >= threshold {
            cheats += 1;
        }
    }
    cheats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        // There are 14 cheats that save 2 picoseconds.
        // There are 14 cheats that save 4 picoseconds.
        // There are 2 cheats that save 6 picoseconds.
        // There are 4 cheats that save 8 picoseconds.
        // There are 2 cheats that save 10 picoseconds.
        // There are 3 cheats that save 12 picoseconds.
        // There is one cheat that saves 20 picoseconds.
        // There is one cheat that saves 36 picoseconds.
        // There is one cheat that saves 38 picoseconds.
        // There is one cheat that saves 40 picoseconds.
        // There is one cheat that saves 64 picoseconds.
        let mut n = 0;
        assert_eq!(solve1n(&input, 65), n);
        n += 1;
        assert_eq!(solve1n(&input, 64), n);
        assert_eq!(solve1n(&input, 41), n);
        n += 1;
        assert_eq!(solve1n(&input, 40), n);
        assert_eq!(solve1n(&input, 39), n);
        n += 1;
        assert_eq!(solve1n(&input, 38), n);
        assert_eq!(solve1n(&input, 37), n);
        n += 1;
        assert_eq!(solve1n(&input, 36), n);
        n += 1;
        assert_eq!(solve1n(&input, 20), n);
        n += 3;
        assert_eq!(solve1n(&input, 12), n);
        n += 2;
        assert_eq!(solve1n(&input, 10), n);
        n += 4;
        assert_eq!(solve1n(&input, 8), n);
        n += 2;
        assert_eq!(solve1n(&input, 6), n);
        n += 14;
        assert_eq!(solve1n(&input, 4), n);
        n += 14;
        assert_eq!(solve1n(&input, 2), n);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());

        //     There are 32 cheats that save 50 picoseconds.
        //     There are 31 cheats that save 52 picoseconds.
        //     There are 29 cheats that save 54 picoseconds.
        //     There are 39 cheats that save 56 picoseconds.
        //     There are 25 cheats that save 58 picoseconds.
        //     There are 23 cheats that save 60 picoseconds.
        //     There are 20 cheats that save 62 picoseconds.
        //     There are 19 cheats that save 64 picoseconds.
        //     There are 12 cheats that save 66 picoseconds.
        //     There are 14 cheats that save 68 picoseconds.
        //     There are 12 cheats that save 70 picoseconds.
        //     There are 22 cheats that save 72 picoseconds.
        //     There are 4 cheats that save 74 picoseconds.
        //     There are 3 cheats that save 76 picoseconds.
        assert_eq!(solve2(&input), ());
    }
}
