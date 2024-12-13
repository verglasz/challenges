use utils::{
    get_stdinput,
    grid::{Delta, Point},
    maths::{gcd, lcm},
};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Vec<Problem>;

#[derive(Debug, Clone, Copy)]
struct Problem {
    a: Delta<isize>,
    b: Delta<isize>,
    prize: Point<isize>,
}

/// represents the linear equation `a*x + b*y = c`
#[derive(Debug, Clone, Copy)]
struct Eqn {
    a: isize,
    b: isize,
    c: isize,
}

impl Eqn {
    fn new(a: isize, b: isize, c: isize) -> Self {
        Self { a, b, c }
    }

    fn scale(&self, factor: isize) -> Self {
        Self {
            a: self.a * factor,
            b: self.b * factor,
            c: self.c * factor,
        }
    }
}

/// represents a system of two linear equations
#[derive(Debug, Clone, Copy)]
struct Sys {
    eqns: (Eqn, Eqn),
}

impl Sys {
    fn new(eq1: Eqn, eq2: Eqn) -> Self {
        Self { eqns: (eq1, eq2) }
    }

    fn solve(&self) -> Option<Point<isize>> {
        let (e1, e2) = self.eqns;
        let a_gcd = gcd(e1.a, e2.a);
        let mul1 = e1.a / a_gcd;
        let mul2 = e2.a / a_gcd;
    }
}

fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let lines = &mut lines;
    let mut problems = vec![];
    loop {
        let a = {
            let Some(aline) = lines.next() else {
                break;
            };
            let aline = aline.as_ref();
            if aline.is_empty() {
                continue;
            }
            let aline = aline.strip_prefix("Button A: ").expect("has A prefix");
            let mut parts = aline.split(", ");
            let x = parts
                .next()
                .expect("a has x")
                .strip_prefix("X+")
                .expect("a has X+ prefix")
                .parse()
                .expect("a X is number");
            let y = parts
                .next()
                .expect("a has y")
                .strip_prefix("Y+")
                .expect("a has Y+ prefix")
                .parse()
                .expect("a Y is number");
            Delta::new(x, y)
        };
        let b = {
            let _bline = lines.next().expect("has line B");
            let bline = _bline.as_ref();
            let bline = bline.strip_prefix("Button B: ").expect("has B prefix");
            let mut parts = bline.split(", ");
            let x = parts
                .next()
                .expect("b has x")
                .strip_prefix("X+")
                .expect("b has X+ prefix")
                .parse()
                .expect("b X is number");
            let y = parts
                .next()
                .expect("b has y")
                .strip_prefix("Y+")
                .expect("b has Y+ prefix")
                .parse()
                .expect("b Y is number");
            Delta::new(x, y)
        };
        let prize = {
            let _prizeline = lines.next().expect("has line prize");
            let prizeline = _prizeline.as_ref();
            let prizeline = prizeline.strip_prefix("Prize: ").expect("has prize prefix");
            let mut parts = prizeline.split(", ");
            let x = parts
                .next()
                .expect("prize prize has x")
                .strip_prefix("X=")
                .expect("prize has X= prefix")
                .parse()
                .expect("prize X is number");
            let y = parts
                .next()
                .expect("prize has y")
                .strip_prefix("Y=")
                .expect("prize has Y= prefix")
                .parse()
                .expect("prize Y is number");
            Point::new(x, y)
        };
        problems.push(Problem { a, b, prize });
    }
    problems
}

fn solve1(input: &Input) -> usize {
    println!("{:?}", input);
    0
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
        assert_eq!(solve1(&input), 480);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 0);
    }
}
