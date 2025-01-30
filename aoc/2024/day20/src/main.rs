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

fn solve2(input: &Input) -> usize {
    solve2n(input, 100)
}

// just find times to reach each cell
fn walk(input: &Input) -> VecMat<usize> {
    let (start, map, end) = input;
    let mut map = map.clone();
    let mut p = *start;
    let mut time = 0;
    let mut times = VecMat::filled(map.shape(), &usize::MAX);
    while p != *end {
        // mark current cell as visited
        map[p] = CellState::Pathed;
        times[p] = time;
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
        }
        debug_assert_ne!(step, p, "no path found before end..");
        // move to the next cell
        time += 1;
        p = step;
    }
    times[p] = time;
    times
}

fn solve2n(input: &Input, threshold: usize) -> usize {
    let (start, map, end) = input;
    let mut map = map.clone();
    let normal_times = walk(input);
    let mut p = *start;
    let mut cheats = 0;
    let mut time = 0;
    while p != *end {
        // mark current cell as visited
        map[p] = CellState::Pathed;
        // check the next normal step
        let mut step = p;
        // now check what we can do by starting to cheat here!
        let mut visited = HashSet::from([p]);
        let mut queue: Vec<_> = p.neighbours().collect();
        let mut cheat_t = 1;
        // println!("at step {}", time);
        while cheat_t <= 20 && !queue.is_empty() {
            // println!("cheat_t: {}, queue size: {}", cheat_t, queue.len());
            let mut next_queue = Vec::with_capacity(queue.len());
            for next in queue.drain(..) {
                match map.get(next) {
                    None => continue, // out of bounds
                    Some(CellState::Pathed) => {
                        // we've already been here walking normally before,
                        // no point in marking a cheat to here
                        // only thing we need to do is check neighbours and continue
                        // continue;
                    }
                    Some(CellState::Empty) => {
                        if cheat_t == 1 {
                            step = next;
                            // we'll walk here normally so the cheat saves no time...
                        } else {
                            // we can cheat to here
                            let cheat_time = cheat_t + time;
                            let normal_time = normal_times[next];
                            if cheat_time + threshold <= normal_time {
                                // we can cheat to here faster than walking normally
                                cheats += 1;
                            }
                        }
                    }
                    Some(CellState::Wall) => {
                        // check neighbours and continue
                    }
                };
                for n in next.neighbours() {
                    if visited.insert(n) {
                        next_queue.push(n);
                    }
                }
            }
            cheat_t += 1;
            queue = next_queue;
        }
        debug_assert_ne!(step, p, "no path found before end..");
        // move to the next cell
        time += 1;
        p = step;
    }
    // accumulate end cheats
    // for t in cheat_times[p].iter().copied() {
    //     let saved = time - t;
    //     if saved >= threshold {
    //         cheats += 1;
    //     }
    // }
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
        let mut n = 0;
        assert_eq!(solve2n(&input, 77), n);
        n += 3;
        assert_eq!(solve2n(&input, 76), n);
        assert_eq!(solve2n(&input, 75), n);
        n += 4;
        assert_eq!(solve2n(&input, 74), n);
        assert_eq!(solve2n(&input, 73), n);
        n += 22;
        assert_eq!(solve2n(&input, 72), n);
        assert_eq!(solve2n(&input, 71), n);
        n += 12;
        assert_eq!(solve2n(&input, 70), n);
        assert_eq!(solve2n(&input, 69), n);
        n += 14;
        assert_eq!(solve2n(&input, 68), n);
        assert_eq!(solve2n(&input, 67), n);
        n += 12;
        assert_eq!(solve2n(&input, 66), n);
        assert_eq!(solve2n(&input, 65), n);
        n += 19;
        assert_eq!(solve2n(&input, 64), n);
        assert_eq!(solve2n(&input, 63), n);
        n += 20;
        assert_eq!(solve2n(&input, 62), n);
        assert_eq!(solve2n(&input, 61), n);
        n += 23;
        assert_eq!(solve2n(&input, 60), n);
        assert_eq!(solve2n(&input, 59), n);
        n += 25;
        assert_eq!(solve2n(&input, 58), n);
        assert_eq!(solve2n(&input, 57), n);
        n += 39;
        assert_eq!(solve2n(&input, 56), n);
        assert_eq!(solve2n(&input, 55), n);
        n += 29;
        assert_eq!(solve2n(&input, 54), n);
        assert_eq!(solve2n(&input, 53), n);
        n += 31;
        assert_eq!(solve2n(&input, 52), n);
        assert_eq!(solve2n(&input, 51), n);
        n += 32;
        assert_eq!(solve2n(&input, 50), n);
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert!(solve2(&input) > 220679);
    }
}
