use utils::{get_stdinput, pairs};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Vec<Vec<isize>>;

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    lines
        .filter(|s| !s.as_ref().is_empty())
        .map(|s| {
            s.as_ref()
                .split_whitespace()
                .map(|x| x.parse().expect("line elements should be numbers"))
                .collect()
        })
        .collect()
}
fn base_safe<'a>(rows: impl Iterator<Item = &'a isize>, min: isize, max: isize) -> Option<usize> {
    pairs(rows)
        .map(|w| w.1 - w.0)
        .enumerate()
        .find_map(|(i, d)| (d < min || max < d).then_some(i))
}

fn solve1(input: &Input) -> usize {
    fn is_safe(row: &[isize], min: isize, max: isize) -> bool {
        base_safe(row.iter(), min, max).is_none()
    }
    input
        .iter()
        .filter(|row| is_safe(row, 1, 3) || is_safe(row, -3, -1))
        .count()
}

fn solve2(input: &Input) -> usize {
    fn is_safe(row: &[isize], min: isize, max: isize) -> bool {
        let Some(i) = base_safe(row.iter(), min, max) else {
            return true;
        };
        // the i-th difference is wrong,
        // problem is with i-th element or i+1-th, we try removing both
        let (a, b) = row.split_at_checked(i + 1).expect("i is at most len-1");
        // skip(1) removes i+1-th element
        if base_safe(a.iter().chain(b.iter().skip(1)), min, max).is_none() {
            return true;
        }
        // split_last removes i-th element
        let (_, a) = a
            .split_last()
            .expect("a is not empty since we split at at least 1");
        base_safe(a.iter().chain(b.iter()), min, max).is_none()
    }
    input
        .iter()
        .filter(|row| is_safe(row, 1, 3) || is_safe(row, -3, -1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 2);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 4);
    }
}
