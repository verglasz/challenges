use std::collections::HashSet;

use pathfinding::directed::{astar, dijkstra};
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
type Input = (VecMat<bool>, Point<usize>, Point<usize>);

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let mut start = None;
    let mut end = None;
    let grid = lines
        .filter(|l| !l.as_ref().is_empty())
        .enumerate()
        .map(|(row, l)| {
            l.as_ref()
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(col, c)| match c {
                    b'.' => true,
                    b'#' => false,
                    b'S' => {
                        if start.replace(Point::new(col, row)).is_some() {
                            panic!("start already set");
                        }
                        true
                    }
                    b'E' => {
                        if end.replace(Point::new(col, row)).is_some() {
                            panic!("end already set");
                        }
                        true
                    }
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .try_into()
        .expect("should be a rectangular grid");
    (
        grid,
        start.expect("start not found"),
        end.expect("end not found"),
    )
}

fn successors(
    (p, d): &(Point<usize>, Dir),
    grid: &VecMat<bool>,
) -> impl IntoIterator<Item = ((Point<usize>, Dir), usize)> {
    let next = p.wrapping_add_signed(d.to_delta());
    [
        ((*p, d.clockwise_cross()), 1000),
        ((*p, d.counterclockwise_cross()), 1000),
    ]
    .into_iter()
    .chain(grid[next].then(|| ((next, *d), 1)))
}

fn solve1(input: &Input) -> usize {
    let (grid, start, end) = input;
    dijkstra::dijkstra(
        &(*start, Dir::E),
        |n| successors(n, grid),
        |(p, _)| p == end,
    )
    .expect("no path found")
    .1
}

fn solve2(input: &Input) -> usize {
    let (grid, start, end) = input;
    let mut paths = astar::astar_bag(
        &(*start, Dir::E),
        |n| successors(n, grid),
        |n| n.0.delta_to(*end).unwrap().manhattan(),
        |(p, _)| p == end,
    )
    .expect("no path found")
    .0;
    paths
        .into_iter()
        .flatten()
        .map(|(p, _)| p)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 7036);
    }
    #[test]
    fn test1b() {
        let input = include_str!("../test2");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 11048);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 45);
    }

    #[test]
    fn test2b() {
        let input = include_str!("../test2");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 64);
    }
}
