use utils::{
    get_stdinput,
    grid::{Delta, Point},
};

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = Vec<Code>;
type Code = [u8; 4];

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|x| {
            let c = x.as_bytes();
            [c[0], c[1], c[2], c[3]]
        })
        .collect()
}

fn getnum(code: &Code) -> usize {
    std::str::from_utf8(&code[..3])
        .expect("codes are utf8")
        .parse()
        .expect("codes are numbers")
}

fn numpad_pos(btn: u8) -> Point<u8> {
    match btn {
        b'7' => Point::new(0, 0),
        b'8' => Point::new(0, 1),
        b'9' => Point::new(0, 2),
        b'4' => Point::new(1, 0),
        b'5' => Point::new(1, 1),
        b'6' => Point::new(1, 2),
        b'1' => Point::new(2, 0),
        b'2' => Point::new(2, 1),
        b'3' => Point::new(2, 2),
        b'0' => Point::new(3, 1),
        b'A' => Point::new(3, 2),
        _ => panic!("invalid button"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dirpad {
    L,
    R,
    U,
    D,
    A,
}

impl Dirpad {
    fn pos(self) -> Point<i8> {
        match self {
            Dirpad::U => Point::new(0, 1),
            Dirpad::A => Point::new(0, 2),
            Dirpad::L => Point::new(1, 0),
            Dirpad::D => Point::new(1, 1),
            Dirpad::R => Point::new(1, 2),
        }
    }

    const MISSING: Point<u8> = Point::new(3, 0);
}

// type PathOptions = Vec<Vec<(usize, Dirpad)>>;
type Path = [(usize, Dirpad); 2];

enum PathOptions {
    Tap((usize, Dirpad)),
    Ordered(Path),
    Both(Path),
}

fn solve_step(pos: Point<u8>, dest: Point<u8>, missing: Point<u8>) -> PathOptions {
    let delta = pos.delta_to(dest).expect("everything in bounds");
    let l = if delta.dx < 0 { -delta.dx } else { 0 };
    let r = if delta.dx > 0 { delta.dx } else { 0 };
    let u = if delta.dy < 0 { -delta.dy } else { 0 };
    let d = if delta.dy > 0 { delta.dy } else { 0 };
    use PathOptions::*;
    use Dirpad::*;
    if pos.x == missing.x && dest.y == missing.y {
        // one path option collides with missing
        if



    } else if {
    } else {
    }
}

fn solve_seq(mut pos: Point<u8>, desired: &[Point<u8>], missing: Point<u8>) -> Vec<PathOptions> {
    let mut seq = vec![];
    for &next in desired {}
    seq
}

fn shortest_seq(desired: &[Point<u8>], depth: u8, missing: Point<u8>, start: Point<u8>) -> usize {
    let seq = solve_seq(start, desired, missing);
    0
}

fn shortest_numpad_seq(code: &Code, depth: u8) -> usize {
    shortest_seq(
        &code.map(numpad_pos),
        depth,
        Point::new(3, 0),
        numpad_pos(b'A'),
    )
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .map(|code| shortest_numpad_seq(code, 3) * getnum(code))
        .sum()
}

fn solve2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 126384);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 0);
    }
}
