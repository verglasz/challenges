use std::collections::HashSet;

use pathfinding::{
    directed::{bfs, dijkstra},
    prelude::astar,
};
use utils::{
    get_stdinput,
    grid::{Dir, Point},
};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = (Vec<Point<usize>>, Point<usize>);

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let falling = lines
        .filter(|l| !l.as_ref().is_empty())
        .map(|l| {
            let mut parts = l.as_ref().split(",");
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            Point::new(x, y)
        })
        .collect();
    let bounds = Point::new(71, 71);
    (falling, bounds)
}

fn successors<'a>(
    p: &Point<usize>,
    bounds: Point<usize>,
    blocks: &'a HashSet<Point<usize>>,
) -> impl Iterator<Item = Point<usize>> + 'a {
    p.neighbours()
        .filter(move |p| !blocks.contains(p) && p.in_bounds(bounds.into_both().into()))
}

fn solve1(input: &Input) -> usize {
    let (falling, bounds) = input;
    let fallen: HashSet<_> = falling.iter().copied().take(1024).collect();
    let start = Point::new(0, 0);
    let end = bounds.neighbour(Dir::NW);
    bfs::bfs(&start, |p| successors(p, *bounds, &fallen), |p| *p == end)
        .expect("no path found")
        .len()
        - 1
}

fn solve2(input: &Input) -> String {
    let (falling, bounds) = input;
    let mut fallen = HashSet::new();
    let start = Point::new(0, 0);
    let end = bounds.neighbour(Dir::NW);
    let mut blocker = None;
    for b in falling {
        fallen.insert(*b);
        let path = astar(
            &start,
            |&current| successors(&current, *bounds, &fallen).map(|p| (p, 1)),
            |p| p.delta_to(end).unwrap().manhattan(),
            |p| *p == end,
        );
        if path.is_none() {
            blocker = Some(*b);
            break;
        }
    }
    let blocker = blocker.expect("no blocker found");

    format!("{},{}", blocker.x, blocker.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let mut input = parse(input.lines());
        input.1 = Point::new(7, 7);
        let n = 12;
        input.0.drain(n..);
        assert_eq!(solve1(&input), 22);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let mut input = parse(input.lines());
        input.1 = Point::new(7, 7);
        assert_eq!(solve2(&input), "6,1");
    }
}
