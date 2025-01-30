use utils::{get_stdinput, Counter};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}

type Input = (Vec<isize>, Vec<isize>);

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    lines
        .filter(|s| !s.as_ref().is_empty())
        .map(|s| {
            let mut ns = s.as_ref().split_whitespace();
            let a: isize = ns
                .next()
                .expect("lines must have element 1")
                .parse()
                .expect("element 1 must be a number");
            let b: isize = ns
                .next()
                .expect("lines must have element 2")
                .parse()
                .expect("element 2 must be a number");
            (a, b)
        })
        .collect()
}

fn solve1(input: &Input) -> isize {
    let mut a = input.0.clone();
    let mut b = input.1.clone();
    a.sort();
    b.sort();
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum()
}

fn solve2(input: &Input) -> usize {
    let a = &input.0;
    let counts: Counter<_> = input.1.iter().copied().collect();
    a.iter().map(|n| *n as usize * counts.get_or_zero(n)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 11);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 31);
    }
}
