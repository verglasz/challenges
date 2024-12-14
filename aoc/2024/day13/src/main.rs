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

impl Problem {
    fn new(a: Delta<isize>, b: Delta<isize>, prize: Point<isize>) -> Self {
        Self { a, b, prize }
    }

    fn to_sys(&self) -> Sys {
        let eq1 = Eqn::new(self.a.dx, self.b.dx, self.prize.x);
        let eq2 = Eqn::new(self.a.dy, self.b.dy, self.prize.y);
        Sys::new(eq1, eq2)
    }
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

    fn diff(&self, other: Self) -> Self {
        Self {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
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
        let mut mul1 = e1.a / a_gcd;
        // if e1.a < 0 ^ e2.a < 0 {
        //     mul1 = -mul1;
        // }
        let mut mul2 = e2.a / a_gcd;
        // if e1.a < 0 ^ e2.a < 0 {
        //     mul2 = -mul2;
        // }
        // println!("e1: {:?}, e2: {:?}", e1, e2);
        let e1 = e1.scale(mul2);
        let e2 = e2.scale(mul1);
        let e = e1.diff(e2);
        // println!("e1: {:?}, e2: {:?}, gcd: {}, e: {:?}", e1, e2, a_gcd, e);
        if e.b == 0 {
            return None;
        }
        if e.c % e.b != 0 {
            return None;
        }
        let y = e.c / e.b;
        let k = (e1.c - e1.b * y);
        if k % e1.a != 0 {
            return None;
        }
        let x = k / e1.a;
        Some(Point::new(x, y))
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
    let mut total = 0;
    for problem in input {
        let sys = problem.to_sys();
        // println!("problem: {:?}", problem);
        if let Some(Point { x, y }) = sys.solve() {
            // println!("a: {x}, b: {y}");
            if x < 0 || y < 0 || x > 100 || y > 100 {
                continue;
            }
            total += 3 * x + 1 * y;
        }
    }
    total.try_into().expect("total is usize")
}

fn solve2(input: &Input) -> usize {
    let mut total = 0;
    let d = Delta::new(10000000000000, 10000000000000);
    for mut problem in input.iter().cloned() {
        problem.prize = problem.prize.add(d);
        let sys = problem.to_sys();
        // println!("problem: {:?}", problem);
        if let Some(Point { x, y }) = sys.solve() {
            // println!("a: {x}, b: {y}");
            if x < 0 || y < 0 {
                continue;
            }
            total += 3 * x + 1 * y;
        }
    }
    total.try_into().expect("total is usize")
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
        assert!(solve2(&input) > 0);
    }
}
