use log::*;
use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    mem,
    ops::Deref,
};

fn main() {
    let lines = stdin().lines().map(|line| line.unwrap());
    let input = parse_input(lines);
    println!("input: {:?}", input);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

type Coord = isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(Coord, Coord);

impl Point {
    fn neighbours(&self) -> [Point; 4] {
        [
            Point(self.0.wrapping_sub(1), self.1),
            Point(self.0, self.1.wrapping_add(1)),
            Point(self.0.wrapping_add(1), self.1),
            Point(self.0, self.1.wrapping_sub(1)),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map<T> {
    inner: Vec<Vec<T>>,
}

type MapInitError = &'static str;
impl<T> Map<T> {
    fn new(inner: Vec<Vec<T>>) -> Result<Self, MapInitError> {
        if inner.is_empty() {
            return Err("empty map");
        }
        let width = inner[0].len();
        if !inner.iter().all(|row| row.len() == width) {
            return Err("inconsistent row length");
        }
        Ok(Self { inner })
    }
    fn get(&self, p: Point) -> Option<&T> {
        self.inner.get(p.0 as usize)?.get(p.1 as usize)
    }

    fn height(&self) -> usize {
        self.inner.len()
    }

    fn width(&self) -> usize {
        self.inner.get(0).expect("Map can't be empty").len()
    }

    fn iter_coords(&self) -> impl Iterator<Item = (Point, &T)> {
        self.inner.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, x)| (Point(i as isize, j as isize), x))
        })
    }

    fn get_wrapping(&self, p: Point) -> &T {
        let i = p.0.rem_euclid(self.height() as isize);
        let j = p.1.rem_euclid(self.width() as isize);
        &self.inner[i as usize][j as usize]
    }

    fn in_bounds(&self, p: Point) -> bool {
        self.get(p).is_some()
    }

    fn valid_neighbours(&self, p: Point) -> impl Iterator<Item = (Point, &T)> + '_ {
        p.neighbours()
            .into_iter()
            .filter_map(|x| self.get(x).map(|v| (x, v)))
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Map<T> {
    type Error = MapInitError;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

type Input = (Point, Map<u8>);

fn parse_input<T: Deref<Target = str>>(lines: impl Iterator<Item = T>) -> Input {
    let map: Vec<Vec<_>> = lines
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.bytes().collect())
            }
        })
        .collect();
    let map: Map<u8> = map.try_into().expect("not a valid map");
    let start = map
        .iter_coords()
        .find_map(|(p, x)| (*x == b'S').then_some(p))
        .expect("no start found");
    (start, map)
}

fn count2(map: &Map<u8>, start: Point, steps: usize) -> usize {
    let mut prev = Vec::<HashSet<Point>>::with_capacity(steps);
    prev.push(HashSet::new());
    let mut step = HashSet::new();
    step.insert(start);
    for i in 0..steps {
        let next = step
            .iter()
            .flat_map(|s| {
                s.neighbours()
                    .into_iter()
                    .filter(|p| (!prev[i].contains(p) && *map.get_wrapping(*p) != b'#'))
            })
            .collect();
        prev.push(step);
        step = next;
    }
    let mut count = step.len();
    for (i, s) in prev.iter().rev().enumerate() {
        if i % 2 == 0 {
            continue;
        }
        count += s.len();
    }
    count
}

fn count_var(map: &Map<u8>, start: Point, steps: usize) -> usize {
    // let at_step = Vec::with_capacity(STEPS);
    let mut step = HashSet::new();
    step.insert(start);
    for _ in 0..steps {
        step = step
            .into_iter()
            .flat_map(|s| {
                map.valid_neighbours(s)
                    .filter_map(|(p, x)| (*x != b'#').then_some(p))
            })
            .collect();
    }

    step.len()
}

fn count_reachable<const STEPS: usize>(map: &Map<u8>, start: Point) -> usize {
    count_var(map, start, STEPS)
}

fn part1(input: &Input) -> usize {
    let (start, map) = input;
    count_reachable::<64>(map, *start)
}

fn part2(input: &Input) -> usize {
    // tp2();
    count2(&input.1, input.0, 26501365)
}

fn tp2() {
    let input = include_str!("../test");
    let input = parse_input(input.lines());
    let ps = [
        (6, 16),
        (10, 50),
        (50, 1594),
        (100, 6536),
        (500, 167004),
        (1000, 668697),
        (5000, 16733044),
    ];
    for (steps, count) in ps {
        error!("steps: {}", steps);
        assert_eq!(dbg!(count2(&input.1, input.0, steps)), count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test");
        let input = parse_input(input.lines());
        assert_eq!(count_reachable::<6>(&input.1, input.0), 16);
    }

    #[test]
    fn test_part2() {
        tp2();
    }
}
