use std::collections::HashSet;

use utils::get_stdinput;

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Vec<Calib>;
type Calib = (usize, Vec<usize>);

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    lines
        .filter(|s| !s.as_ref().trim().is_empty())
        .map(parse_calib)
        .collect()
}

fn parse_calib(s: impl AsRef<str>) -> Calib {
    let (target, operands) = s.as_ref().split_once(":").expect("should have a space");
    let target = target.trim().parse().expect("should be a number");
    let operands = operands
        .trim()
        .split_whitespace()
        .map(|n| n.parse().expect("should be a number"))
        .collect();
    (target, operands)
}

fn solve1(input: &Input) -> usize {
    let maps = [
        |target, n| (target >= n).then_some(target - n),
        |target, n| (target % n == 0).then_some(target / n),
    ];
    solve(input, maps)
}

fn solve<const N: usize>(input: &Input, maps: [fn(usize, usize) -> Option<usize>; N]) -> usize {
    input
        .iter()
        .filter_map(|(target, ops)| reachable(*target, ops, maps).then_some(target))
        .sum()
}

fn reachable<F, const N: usize>(target: usize, ops: &[usize], maps: [F; N]) -> bool
where
    F: Fn(usize, usize) -> Option<usize>,
{
    let (first, ops) = ops.split_first().expect("should have at least one operand");
    ops.iter()
        .rev()
        .copied()
        .fold(HashSet::from([target]), |set, n| {
            set.into_iter()
                // m is a target we want to reach, see if it works with sums or muls etc
                .flat_map(|m| maps.iter().flat_map(move |f| f(m, n)))
                .collect()
        })
        .contains(&first)
}

fn possibilities(ops: &[usize]) -> HashSet<usize> {
    let mut ops = ops.iter().copied();
    let fst = ops.next().unwrap();
    ops.fold(HashSet::from([fst]), |set, n| {
        set.into_iter()
            .flat_map(|m| [m + n, m * n].into_iter())
            .collect()
    })
}

fn concate(a: usize, b: usize) -> usize {
    let mut x = b;
    let mut pow = 1;
    while x > 0 {
        pow *= 10;
        x /= 10;
    }
    a * pow + b
}

fn more_possibilities(ops: &[usize]) -> HashSet<usize> {
    let mut ops = ops.iter().copied();
    let fst = ops.next().unwrap();
    ops.fold(HashSet::from([fst]), |set, n| {
        set.into_iter()
            .flat_map(|m| [m + n, m * n, concate(m, n)].into_iter())
            .collect()
    })
}

fn solve2(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|(target, ops)| more_possibilities(ops).contains(target).then_some(target))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 3749);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 11387);
    }

    #[test]
    fn input1() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 1298300076754);
    }

    #[test]
    fn input2() {
        let input = include_str!("../input");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 248427118972289);
    }
}
