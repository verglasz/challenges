use utils::{
    get_stdinput,
    grid::{Delta, Point},
};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Vec<Vec<u8>>;

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let input: Input = lines
        .filter(|s| !s.as_ref().trim().is_empty())
        .map(|l| l.as_ref().trim().as_bytes().to_vec())
        .collect();
    debug_assert!(!input.is_empty());
    debug_assert!(!input[0].is_empty());
    debug_assert!(input.iter().all(|l| l.len() == input[0].len()));
    input
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    pub const ALL: [Dir; 8] = [
        Dir::N,
        Dir::NE,
        Dir::E,
        Dir::SE,
        Dir::S,
        Dir::SW,
        Dir::W,
        Dir::NW,
    ];

    pub const CROSS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

    fn to_offset(&self) -> (isize, isize) {
        match self {
            Dir::N => (0, -1),
            Dir::NE => (1, -1),
            Dir::E => (1, 0),
            Dir::SE => (1, 1),
            Dir::S => (0, 1),
            Dir::SW => (-1, 1),
            Dir::W => (-1, 0),
            Dir::NW => (-1, -1),
        }
    }

    fn to_delta(&self) -> Delta<isize> {
        let (dx, dy) = self.to_offset();
        Delta::new(dx, dy)
    }
}

mod locals {
    use super::Input;
    use utils::grid::{Delta, Point};
    pub type D = Delta<isize>;
    pub type P = Point<usize>;
    pub fn getp(input: &Input, p: P) -> Option<u8> {
        input.get(p.y).and_then(|l| l.get(p.x)).copied()
    }
}

fn solve1(input: &Input) -> isize {
    use locals::*;
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == b'X' {
                const TARGET: &[u8] = b"MAS";
                for dir in &Dir::ALL {
                    let mut found = true;
                    let mut p = P::new(x, y);
                    let d = dir.to_delta();
                    for c in TARGET.iter() {
                        p = p.wrapping_add_signed(d);
                        match getp(input, p) {
                            Some(c2) if c2 == *c => continue,
                            _ => {
                                found = false;
                                break;
                            }
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn solve2(input: &Input) -> isize {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == b'A' {
                if is_mas(input, x, y).is_some() {
                    count += 1;
                }
            }
        }
    }
    count
}

fn is_mas(input: &Input, x: usize, y: usize) -> Option<()> {
    use locals::*;
    let p = P::new(x, y);
    for rel in [1, -1] {
        // rel (sign) denotes the two diagonals
        // (ie, one of them has x,y changing in the same direction, the other in opposite)
        let d1 = D::new(1, rel);
        let d2 = D::new(-1, rel * -1);
        // find the two other elements on this diagonal
        let p1 = p.checked_add_signed(d1)?;
        let p2 = p.checked_add_signed(d2)?;
        // check that the diagonal is MAS in some way
        match (getp(input, p1)?, getp(input, p2)?) {
            (b'M', b'S') => (),
            (b'S', b'M') => (),
            _ => return None,
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 18);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 9);
    }
}
