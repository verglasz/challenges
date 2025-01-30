use core::fmt;
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
type Input = (Warehouse, Movements, Point<usize>);
type Warehouse = VecMat<Obj>;
type Movements = Vec<Dir>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Obj {
    Wall,
    Empty,
    Box,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WideObj {
    Wall,
    Empty,
    BoxL,
    BoxR,
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Obj::*;
        write!(
            f,
            "{}",
            match self {
                Wall => '#',
                Empty => '.',
                Box => 'O',
            }
        )
    }
}

impl fmt::Display for WideObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use WideObj::*;
        write!(
            f,
            "{}",
            match self {
                Wall => '#',
                Empty => '.',
                BoxL => '[',
                BoxR => ']',
            }
        )
    }
}

fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let mut pos = None;
    let warehouse = lines
        .by_ref()
        .take_while(|l| !l.as_ref().is_empty())
        .enumerate()
        .map(|(row, l)| {
            l.as_ref()
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(col, &c)| match c {
                    b'#' => Obj::Wall,
                    b'.' => Obj::Empty,
                    b'O' => Obj::Box,
                    b'@' => {
                        if pos.replace(Point::new(col, row)).is_some() {
                            panic!("multiple robots");
                        }
                        Obj::Empty
                    }
                    _ => panic!("unexpected char in warehouse"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("should be a square matrix");
    let moves = lines
        .filter(|l| !l.as_ref().is_empty())
        .flat_map(|l| {
            l.as_ref()
                .as_bytes()
                .iter()
                .map(|c| match c {
                    b'^' => Dir::N,
                    b'v' => Dir::S,
                    b'<' => Dir::W,
                    b'>' => Dir::E,
                    _ => panic!("unexpected char in moves"),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (warehouse, moves, pos.expect("should have a player"))
}

fn solve1(input: &Input) -> usize {
    let (warehouse, moves, mut pos) = input;

    // println!("starting position:");
    // println!("{}", warehouse.highlighted(&[pos].into_iter().collect()));
    // println!("moves: {:?}", moves);
    let mut warehouse = warehouse.clone();
    for dir in moves {
        pos = move_bot(&mut warehouse, *dir, pos);
        // println!("moved {} to {:?}", dir, pos);
        // println!("{}", warehouse.highlighted(&[pos].into_iter().collect()));
    }
    warehouse
        .iter_pos()
        .map(|(p, o)| match o {
            Obj::Box => p.y * 100 + p.x,
            _ => 0,
        })
        .sum()
}

fn move_bot(warehouse: &mut Warehouse, dir: Dir, pos: Point<usize>) -> Point<usize> {
    let new_pos = pos.wrapping_add_signed(dir.to_delta());
    let mut box_pos = new_pos;
    loop {
        match warehouse[box_pos] {
            Obj::Wall => break pos,
            Obj::Empty => {
                warehouse[box_pos] = Obj::Box;
                warehouse[new_pos] = Obj::Empty;
                break new_pos;
            }
            Obj::Box => {
                box_pos = box_pos.wrapping_add_signed(dir.to_delta());
                continue;
            }
        }
    }
}

fn solve2(input: &Input) -> usize {
    let (warehouse, moves, mut pos) = input;
    pos.x *= 2;
    let mut warehouse = widen(warehouse);
    // println!("starting position:");
    // println!("{}", warehouse.highlighted(&[pos].into_iter().collect()));
    for dir in moves {
        pos = move_wide(&mut warehouse, *dir, pos);
        // println!("moved {} to {:?}", dir, pos);
        // println!("{}", warehouse.highlighted(&[pos].into_iter().collect()));
    }
    warehouse
        .iter_pos()
        .map(|(p, o)| match o {
            WideObj::BoxL => p.y * 100 + p.x,
            _ => 0,
        })
        .sum()
}

fn move_wide(warehouse: &mut WideWarehouse, dir: Dir, pos: Point<usize>) -> Point<usize> {
    let mut pushers = HashSet::from_iter([pos]);
    let mut to_move = vec![];
    while !pushers.is_empty() {
        let mut new_pushers = HashSet::new();
        for pusher in &pushers {
            // to_move.push(*pusher); // this element needs to proceed
            let next = pusher.wrapping_add_signed(dir.to_delta());
            match warehouse[next] {
                WideObj::Wall => return pos, // if anything hits a wall, nothing moves
                WideObj::Empty => continue, // the thing that needed to move can move here, all good
                WideObj::BoxL => {
                    // this thing needs to move
                    // println!("found left half of box at {:?}", next);
                    new_pushers.insert(next);
                    if dir.is_vertical() {
                        // println!("moving right half of box too");
                        // the other half needs to move too
                        new_pushers.insert(next.wrapping_add_signed(Dir::E.to_delta()));
                    }
                }
                WideObj::BoxR => {
                    // this thing needs to move
                    new_pushers.insert(next);
                    // the other half needs to move too
                    if dir.is_vertical() {
                        new_pushers.insert(next.wrapping_add_signed(Dir::W.to_delta()));
                    }
                }
            }
        }
        // everything that needs to push also needs to move
        to_move.extend(new_pushers.iter().copied());
        pushers = new_pushers;
    }
    for &p in to_move.iter().rev() {
        let dest = p.wrapping_add_signed(dir.to_delta());
        // println!("moving {:?} to {:?}", p, dest);
        debug_assert!(matches!(warehouse[dest], WideObj::Empty));
        warehouse[dest] = warehouse[p];
        warehouse[p] = WideObj::Empty;
    }
    pos.wrapping_add_signed(dir.to_delta())
}

type WideWarehouse = VecMat<WideObj>;
fn widen(warehouse: &Warehouse) -> WideWarehouse {
    warehouse
        .iter_rows()
        .map(|row| {
            row.iter()
                .flat_map(|&o| match o {
                    Obj::Wall => [WideObj::Wall, WideObj::Wall],
                    Obj::Empty => [WideObj::Empty, WideObj::Empty],
                    Obj::Box => [WideObj::BoxL, WideObj::BoxR],
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .try_into()
        .expect("should be a matrix still")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 10092);
    }
    #[test]
    fn test1_small() {
        let input = include_str!("../smalltest");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 2028);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 9021);
    }
    #[test]
    fn test2_small() {
        let input = include_str!("../smalltest");
        let input = parse(input.lines());
        assert!(solve2(&input) > 0);
    }
}
