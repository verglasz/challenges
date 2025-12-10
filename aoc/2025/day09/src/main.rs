use std::{collections::HashSet, iter::once};

use utils::{
    decimals::mask10,
    get_stdinput,
    grid::{Dir, Point, VecMat},
};

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = Vec<Point<usize>>;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|l| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            Point::new(x, y)
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, &p)| (0..i).map(move |j| rectangle(p, input[j])))
        .max()
        .unwrap()
}

fn rectangle(p: Point<usize>, q: Point<usize>) -> usize {
    let d = p.delta_to(q).unwrap();
    (d.dx.abs() as usize + 1) * (d.dy.abs() as usize + 1)
}

fn solve2(input: &Input) -> usize {
    let x = input.iter().map(|p| p.x).max().unwrap();
    let y = input.iter().map(|p| p.y).max().unwrap();
    let factor = (mask10(x.max(y)) / 100).max(1);
    dbg!(factor);
    let map = input
        .iter()
        .map(|p| Point::new(p.x / factor, p.y / factor))
        .collect::<HashSet<_>>();
    dbg!(&map);
    let arena = VecMat::filled((x / factor + 1, y / factor + 1), &'.');
    println!(
        "{}",
        arena.formatter_with(|y, x, c| if map.contains(&Point::new(x, y)) {
            '#'
        } else {
            *c
        })
    );
    let cor = &corners(input);
    input
        .iter()
        .enumerate()
        .flat_map(|(i, &p)| {
            (0..i).filter_map(move |j| allowed(p, input[j], cor).then_some(rectangle(p, input[j])))
        })
        .max()
        .unwrap()
}

fn allowed(p: Point<usize>, q: Point<usize>, cor: &[Corner]) -> bool {
    cor.iter().all(|c| c.allows(p, q))
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Corner {
    p: Point<usize>,
    inside: Dir,
}
impl Corner {
    fn allows(self, p: Point<usize>, q: Point<usize>) -> bool {
        let x = Point::new(p.x, q.y);
        let y = Point::new(p.y, q.x);
        [p, q, x, y].into_iter().all(|k| {
            let d = self.p.delta_to(k).unwrap().dir().unwrap_or(self.inside);
            let aa = (self.inside.id() as i8 - d.id() as i8).abs();
            let aw = aa <= 1 || aa >= 7;
            if !aw {
                println!("corner {self:?} disallows {p:?}{q:?}");
            }
            aw
        })
    }

    /// make new corner assuming interior is on the right side of the path
    fn new(x: Point<usize>, y: Point<usize>, z: Point<usize>) -> (Self, isize) {
        let d1 = y.delta_to(x).unwrap().dir().unwrap();
        let d2 = y.delta_to(z).unwrap().dir().unwrap();
        let (inside, turn) = between(d1, d2);
        let c = Self { p: y, inside };
        println!("corner {c:?} from xyz={:?}", (x, y, z));
        (c, turn)
    }
}

fn between(d1: Dir, d2: Dir) -> (Dir, isize) {
    use Dir::*;
    match (d1, d2) {
        (N, E) => (SW, -1),
        (N, W) => (NW, 1),
        (E, N) => (NE, -1),
        (W, N) => (SE, 1),
        (W, S) => (SW, 1),
        (E, S) => (NW, -1),
        (S, W) => (NE, -1),
        (S, E) => (SE, 1),
        _ => todo!("{d1} -> {d2} wtf"),
    }
}

fn corners(input: &[Point<usize>]) -> Vec<Corner> {
    let l = input.len();
    // let mut current = if input[0].x == input[1].x {
    //     Dir::W
    // } else if input[0].y == input[1].y {
    //     Dir::N
    // } else {
    //     panic!("the fluff")
    // };
    let mut turns = 0;
    let mut output = vec![];
    for p in input.windows(3).chain([
        [input[l - 2], input[l - 1], input[0]].as_slice(),
        [input[l - 1], input[0], input[1]].as_slice(),
    ]) {
        let [x, y, z] = p else { panic!() };
        let (c, t) = Corner::new(*x, *y, *z);
        turns += t;
        output.push(c);
    }
    assert!(dbg!(turns) % 4 == 0);
    assert!(turns / 4 == 1);
    if turns > 0 {
        output
            .iter_mut()
            .for_each(|c| c.inside = c.inside.opposite());
    }

    output
}

// fn turn(prev: Point<usize>, current: Point<usize>) -> isize {
//     current.id() as isize - prev.id() as isize
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 50);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 24);
    }
    #[test]
    fn test2b() {
        let input = include_str!("../test2");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 24);
    }
}
