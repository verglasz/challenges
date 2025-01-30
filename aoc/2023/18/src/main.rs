use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::stdin,
    num::ParseIntError,
    str::{from_utf8, FromStr},
};

use pathfinding::grid::Grid;

fn main() {
    let input = parse_input();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for Rgb {
    type Err = Option<ParseIntError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[..1] != "#" {
            return Err(None);
        }
        let r = u8::from_str_radix(&s[1..3], 16)?;
        let g = u8::from_str_radix(&s[3..5], 16)?;
        let b = u8::from_str_radix(&s[5..7], 16)?;
        Ok(Rgb { r, g, b })
    }
}

fn parse_input() -> Vec<(u8, usize, [u8; 6])> {
    stdin()
        .lines()
        .filter_map(|l| match l.unwrap() {
            s if s.is_empty() => None,
            s => {
                let mut parts = s.split_whitespace();
                let &dir = parts
                    .next()
                    .expect("pos 0 of parsed line")
                    .as_bytes()
                    .first()
                    .expect("dir");
                let num = parts
                    .next()
                    .expect("pos 1 of parsed line")
                    .parse::<usize>()
                    .unwrap();
                let mut color = [0; 6];
                color
                    .copy_from_slice(&parts.next().expect("pos 2 of parsed line").as_bytes()[2..8]);
                Some((dir, num, color))
            }
        })
        .collect()
}

fn offset(dir: u8) -> (i32, i32) {
    match dir {
        b'U' => (-1, 0),
        b'D' => (1, 0),
        b'L' => (0, -1),
        b'R' => (0, 1),
        _ => panic!("Unknown direction"),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i32, i32);

impl Pos {
    fn new() -> Self {
        Pos(0, 0)
    }

    fn step(&mut self, dir: u8) {
        let (i, j) = offset(dir);
        self.0 += i;
        self.1 += j;
    }

    fn step_n(&mut self, dir: u8, n: usize) -> Self {
        let (i, j) = offset(dir);
        let a = self.0 + i * n as i32;
        let b = self.1 + j * n as i32;
        Pos(a, b)
    }

    fn coords(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}

fn neighbours(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let result = [
        (pos.0.wrapping_add(1), pos.1),
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0, pos.1.wrapping_add(1)),
        (pos.0, pos.1.wrapping_sub(1)),
    ];
    result.into_iter()
}

fn part1<T>(input: &Vec<(u8, usize, T)>) -> usize {
    let mut map = HashMap::new();
    let mut pos = Pos(0, 0);
    for &(dir, num, _) in input {
        for _ in 0..num {
            pos.step(dir);
            map.insert(pos.coords(), ());
        }
    }
    let places = map.keys().map(|&(i, j)| (j, i)).collect::<Vec<_>>();
    let grid = Grid::from_coordinates(&places).unwrap();
    let width = grid.width;
    let height = grid.height;
    println!("Grid ({height}x{width})");
    // println!("\n{:?}", grid);
    let mut visited = HashSet::new();
    let mut queue: Vec<_> = (0..width)
        .flat_map(|x| [(x, 0), (x, height - 1)])
        .chain((0..height).flat_map(|y| [(0, y), (width - 1, y)]))
        .collect();
    while let Some(pos) = queue.pop() {
        // println!("Pos: {:?}", pos);
        if visited.contains(&pos) || grid.has_vertex(pos) || !grid.is_inside(pos) {
            // println!("Skipping");
            continue;
        }
        visited.insert(pos);
        queue.extend(neighbours(pos));
    }
    // println!("Visited: {:?}", visited);

    height * width - visited.len()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Bbox(Pos, Pos);

impl Bbox {
    fn update(&mut self, pos: Pos) {
        let mut topleft = &mut self.0;
        let mut bottomright = &mut self.1;
        if pos.0 < topleft.0 {
            topleft.0 = pos.0;
        }
        if pos.1 < topleft.1 {
            topleft.1 = pos.1;
        }
        if pos.0 > bottomright.0 {
            bottomright.0 = pos.0;
        }
        if pos.1 > bottomright.1 {
            bottomright.1 = pos.1;
        }
    }
}

fn part2(input: &Vec<(u8, usize, [u8; 6])>) -> usize {
    let mut rearranged = input.iter().map(|&(_, _, color)| {
        // 0 means R, 1 means D, 2 means L, and 3 means U.
        let dir = match color[5] {
            b'3' => b'U',
            b'0' => b'R',
            b'1' => b'D',
            b'2' => b'L',
            _ => panic!("Unknown direction"),
        };
        let n = from_utf8(&color[..5]).unwrap();
        let num = usize::from_str_radix(n, 16).unwrap();
        (dir, num)
    });
    let mut pos = Pos(0, 0);
    let mut bb = Bbox(pos, pos);
    let mut vert_walls = BTreeMap::new();
    let mut horiz_walls = BTreeMap::new();
    for (dir, num) in rearranged {
        let next = pos.step_n(dir, num);
        bb.update(next);
        let fst;
        let last;
        let walls;
        let key;
        if next.0 == pos.0 {
            key = pos.0;
            fst = next.1.min(pos.1);
            last = next.1.max(pos.1);
            walls = &mut vert_walls;
        } else if next.1 == pos.1 {
            key = pos.1;
            fst = next.0.min(pos.0);
            last = next.0.max(pos.0);
            walls = &mut horiz_walls;
        } else {
            panic!("Invalid move");
        }
        use std::collections::btree_map::Entry::*;
        match walls.entry(key) {
            Vacant(e) => {
                e.insert(vec![(fst, last)]);
            }
            Occupied(mut e) => e.get_mut().push((fst, last)),
        }
    }
    println!("Bbox: {:?}", bb);
    0
}
