use pathfinding::directed::*;
use std::ops::{Index, IndexMut};

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone)]
struct Mat<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap()
    }
}

impl<T> Mat<T> {
    fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        let data = vec![T::default(); width * height];
        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<&T> {
        let idx = self.idx(i, j)?;
        Some(&self.data[idx])
    }

    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        let idx = self.idx(j, i)?;
        Some(&mut self.data[idx])
    }

    fn idx(&self, i: usize, j: usize) -> Option<usize> {
        if j >= self.width || i >= self.height {
            return None;
        }
        Some(i * self.width + j)
    }
}

fn parse_input() -> Mat<u8> {
    let v: Vec<_> = std::io::stdin()
        .lines()
        .filter_map(|line| match line.ok()? {
            l if l.is_empty() => None,
            l => Some(l.into_bytes()),
        })
        .collect();
    let width = v[0].len();
    let height = v.len();
    let mut mat = Mat::new(width, height);
    for (y, line) in v.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            mat[(x, y)] = c - b'0';
        }
    }
    mat
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn as_num(&self) -> usize {
        match self {
            Dir::Up => 0,
            Dir::Right => 1,
            Dir::Down => 2,
            Dir::Left => 3,
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
    const fn all() -> [Self; 4] {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    }
    fn from_num(n: usize) -> Self {
        match n {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            3 => Dir::Left,
            _ => panic!("Invalid direction number"),
        }
    }
    fn as_offset(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }

    fn offset(&self, i: usize, j: usize) -> (usize, usize) {
        let (di, dj) = self.as_offset();
        let i = i.wrapping_add_signed(di);
        let j = j.wrapping_add_signed(dj);
        (i, j)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    i: usize,
    j: usize,
    going: (Dir, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeNeighbours {
    node: Node,
    min: usize,
    max: usize,
    dir: usize,
}

impl NodeNeighbours {
    fn advance(&mut self) {
        // print!("Skipping, dir: {}", self.dir);
        self.dir += 1;
        // println!(" -> {}", self.dir);
    }
}

impl Iterator for NodeNeighbours {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("Next: {:?}", self);
        if self.dir >= 4 {
            return None;
        }
        let dir = Dir::from_num(self.dir);
        self.advance();
        if dir == self.node.going.0.opposite() {
            // println!("Skipping opposite");
            return self.next();
        }
        let mut count = 1;
        if dir == self.node.going.0 {
            if self.node.going.1 >= self.max {
                // println!("Can't keep going straight");
                return self.next();
            }
            count += self.node.going.1;
        } else if self.node.going.1 < self.min {
            // println!("Must keep going straight");
            return self.next();
        }

        let (i, j) = dir.offset(self.node.i, self.node.j);
        let going = (dir, count);
        // println!("Next: {:?} -> {:?}", (i, j), going);
        Some(Node { i, j, going })
    }
}

impl Node {
    fn neighbours(&self, min: usize, max: usize) -> NodeNeighbours {
        NodeNeighbours {
            node: *self,
            min,
            max,
            dir: 0,
        }
    }
}

fn part1(grid: &Mat<u8>) -> usize {
    solve::<0, 3>(grid)
}

fn part2(grid: &Mat<u8>) -> usize {
    solve::<4, 10>(grid)
}

fn neighbours<const MIN: usize, const MAX: usize>(
    grid: &Mat<u8>,
    node: Node,
) -> impl Iterator<Item = (Node, usize)> + '_ {
    // println!("Neighbours of: {:?}", node);
    node.neighbours(MIN, MAX)
        .filter_map(|n| Some((n, *grid.get(n.i, n.j)? as usize)))
}

fn solve<const MIN: usize, const MAX: usize>(grid: &Mat<u8>) -> usize {
    const START: Node = Node {
        i: 0,
        j: 0,
        going: (Dir::Right, 0),
    };
    let path = dijkstra::dijkstra(
        &START,
        |n| neighbours::<MIN, MAX>(grid, *n),
        |n| n.i == grid.height - 1 && n.j == grid.width - 1 && n.going.1 >= MIN,
    );
    // println!("Found path: {:?}", path);
    path.unwrap().1
}
