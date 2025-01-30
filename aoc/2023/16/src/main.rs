#!/usr/bin/env rust-script

use std::{collections::VecDeque, iter::once, ops::Add};

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_input() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .filter_map(|line| match line.unwrap().into_bytes() {
            l if l.is_empty() => None,
            l => Some(l),
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn inbounds(&self, grid: &Vec<Vec<u8>>) -> bool {
        (self.0 < grid.len()) & (self.1 < grid[0].len())
    }
}

#[derive(Debug, Clone, Copy)]
struct Dir(i8, i8);

impl Dir {
    fn new(x: i8, y: i8) -> Self {
        if x.abs() + y.abs() != 1 {
            panic!("Invalid direction: ({}, {})", x, y);
        }
        Dir(x, y)
    }
    fn idx(&self) -> usize {
        match self {
            Dir(0, 1) => 0,
            Dir(1, 0) => 1,
            Dir(0, -1) => 2,
            Dir(-1, 0) => 3,
            _ => panic!("Invalid direction: {:?}", self),
        }
    }
}

impl Add<Dir> for Point {
    type Output = Point;

    fn add(self, rhs: Dir) -> Self::Output {
        Point(
            self.0.wrapping_add_signed(rhs.0 as isize),
            self.1.wrapping_add_signed(rhs.1 as isize),
        )
    }
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    solve(grid, Point(0, 0), Dir(0, 1))
}

fn part2(grid: &Vec<Vec<u8>>) -> usize {
    let mut max = 0;
    let h = grid.len();
    let w = grid[0].len();
    for i in 0..h {
        max = max.max(solve(grid, Point(i, 0), Dir(0, 1)));
        max = max.max(solve(grid, Point(i, w - 1), Dir(0, -1)));
    }
    for j in 0..w {
        max = max.max(solve(grid, Point(0, j), Dir(1, 0)));
        max = max.max(solve(grid, Point(h - 1, j), Dir(-1, 0)));
    }
    max
}

fn hilight(grid: &Vec<Vec<u8>>, state: &Vec<Vec<[bool; 4]>>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if state[i][j].iter().any(|&b| b) {
                print!("\x1b[1;31m{}\x1b[0m", c as char);
            } else {
                print!("{}", c as char);
            }
        }
        println!();
    }
}

fn solve(grid: &Vec<Vec<u8>>, startpos: Point, startdir: Dir) -> usize {
    let mut beams = VecDeque::from([(startpos, startdir)]);
    let mut state = vec![vec![[false; 4]; grid[0].len()]; grid.len()];

    while let Some((pos, dir)) = beams.pop_front() {
        state[pos.0][pos.1][dir.idx()] = true;
        let c = grid[pos.0][pos.1];
        let (da, db) = match (c, dir) {
            (b'\\', Dir(x, y)) => (Dir(y, x), None),
            (b'/', Dir(x, y)) => (Dir(-y, -x), None),
            (b'|', Dir(0, _)) => (Dir(-1, 0), Some(Dir(1, 0))),
            (b'-', Dir(_, 0)) => (Dir(0, -1), Some(Dir(0, 1))),
            _ => (dir, None),
        };
        // println!("{:?} {} -> {:?}", pos, c as char, new_dirs);
        for d in once(da).chain(db) {
            let new_pos = pos + d;
            // println!("{:?} {:?} -> {:?}", pos, d, new_pos);
            if new_pos.inbounds(grid) && !state[new_pos.0][new_pos.1][d.idx()] {
                // println!("pushing");
                beams.push_back((new_pos, d));
            }
        }
    }
    // hilight(grid, &state);
    state
        .iter()
        .flatten()
        .filter(|d| d.iter().any(|&x| x))
        .count()
}
