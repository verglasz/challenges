use std::io::stdin;

use utils::{
    grid::{Delta, Point, VecMat},
    types::Either,
    Counter,
};

fn main() {
    let input = include_str!("../input");
    let parsed = parse(input.lines());
    let bounds = Point::new(101, 103);
    let p1 = solve1(&parsed, bounds);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed, bounds);
    println!("sol2: {p2:?}");
}
type Input = Vec<Robot>;

type Robot = (Point<usize>, Delta<isize>);

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    lines
        .filter(|l| !l.as_ref().is_empty())
        .map(|l| {
            let l = l.as_ref();
            let mut it = l.split_whitespace();
            let pos = it.next().unwrap();
            let vel = it.next().unwrap();
            let pt = {
                let mut pos = pos
                    .strip_prefix("p=")
                    .expect("p=")
                    .split(',')
                    .map(|s| s.parse().expect("positions are numbers"));
                Point::new(pos.next().expect("px"), pos.next().expect("py"))
            };
            let v = {
                let mut vel = vel
                    .strip_prefix("v=")
                    .expect("v=")
                    .split(',')
                    .map(|s| s.parse().expect("velocities are numbers"));
                Delta::new(vel.next().expect("vx"), vel.next().expect("vy"))
            };
            (pt, v)
        })
        .collect()
}

fn evolve(r: Robot, steps: usize, bounds: Point<usize>) -> Robot {
    let (pt, v) = r;
    let Delta { dx, dy } = v.scale(steps as isize);
    let dx = dx.rem_euclid(bounds.x as isize) as usize;
    let dy = dy.rem_euclid(bounds.y as isize) as usize;
    let pt = Point::new((pt.x + dx) % bounds.x, (pt.y + dy) % bounds.y);
    (pt, v)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cross {
    Horizontal,
    Vertical,
}

fn quadrant(p: Point<usize>, bounds: Point<usize>) -> Either<Quadrant, Cross> {
    let Point { x, y } = p;
    let Point { x: w, y: h } = bounds;
    let midh = h / 2;
    let midw = w / 2;
    if x < midw {
        if y < midh {
            Either::Left(Quadrant::TopLeft)
        } else if y > midh {
            Either::Left(Quadrant::BottomLeft)
        } else {
            Either::Right(Cross::Horizontal)
        }
    } else if x > midw {
        if y < midh {
            Either::Left(Quadrant::TopRight)
        } else if y > midh {
            Either::Left(Quadrant::BottomRight)
        } else {
            Either::Right(Cross::Horizontal)
        }
    } else {
        Either::Right(Cross::Vertical)
    }
}

fn solve1(input: &Input, bounds: Point<usize>) -> usize {
    let time = 100;
    let evolved: Input = input
        .iter()
        .copied()
        .map(|r| evolve(r, time, bounds))
        .collect();
    // let mut grid = VecMat::filled(bounds.into_both().swap().into(), &b'.');
    // show_robots(&mut grid, &evolved, b'.');
    // cleanup_robots(&mut grid, &evolved, b'.');
    quadrant_counts(&evolved, bounds).counts().product()
}

fn quadrant_counts(robots: &Input, bounds: Point<usize>) -> Counter<Quadrant> {
    robots
        .iter()
        .copied()
        .map(|(p, _)| quadrant(p, bounds))
        .fold(Counter::new(), |mut c, p| {
            if let Some(q) = p.left() {
                c.add_one(*q);
            }
            c
        })
}

fn show_robots(grid: &mut VecMat<u8>, robots: &Input, blank: u8) {
    for &(p, _) in robots {
        if grid[p] == blank {
            grid[p] = b'1';
        } else {
            grid[p] += 1;
        }
    }
}

fn cleanup_robots(grid: &mut VecMat<u8>, robots: &Input, blank: u8) {
    for &(p, _) in robots {
        grid[p] = blank;
    }
}

fn solve2(input: &Input, bounds: Point<usize>) -> usize {
    let blank = b' ';
    let mut grid = VecMat::filled(bounds.into_both().swap().into(), &blank);
    let mut robots = input.clone();
    // let mut reply = String::new();
    let mut symm = 0;
    for step in 1..bounds.x * bounds.y {
        for r in &mut robots {
            *r = evolve(*r, 1, bounds);
        }
        if special(&mut robots, bounds) {
            symm += 1;
            println!("special state ({symm} found) at step {step}:");
            show_robots(&mut grid, &robots, blank);
            println!("{}", grid);
            // cleanup_robots(&mut grid, &robots, blank);
            // println!("is this the right one?");
            // stdin().read_line(&mut reply).expect("error reading reply");
            // if reply.trim().starts_with('y') {
            //     return step;
            // }
            return step;
        }
    }
    panic!("no result found before looping, {symm} special states found");
}

fn special(robots: &mut Input, bounds: Point<usize>) -> bool {
    // let mut c = quadrant_counts(robots, bounds);
    // c.get(&Quadrant::TopLeft) == c.get(&Quadrant::TopRight)
    //     && c.get(&Quadrant::BottomLeft) == c.get(&Quadrant::BottomRight)
    robots.sort_unstable_by_key(|&(p, _)| (p.y, p.x));
    let mut last_line = 0;
    let mut streak = 0;
    let mut last_x = 0;
    for &mut (p, _) in robots {
        if p.y != last_line {
            streak = 0;
            last_x = 0;
            last_line = p.y;
            continue;
        }
        if p.x == last_x {
            streak += 1;
            if streak > 12 {
                return true;
            }
        } else {
            streak = 0;
        }
        last_x = p.x + 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        let bounds = Point::new(11, 7);
        assert_eq!(solve1(&input, bounds), 12);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        let bounds = Point::new(11, 7);
        // assert!(solve2(&input, bounds) > 0);
    }
}
