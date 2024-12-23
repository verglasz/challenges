use utils::get_stdinput;

fn main() {
    let input = get_stdinput().collect::<Vec<_>>();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Vec<usize>;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect("entries must be numbers"))
        .collect()
}

const PRUNE: usize = 16777216;

fn evolve(mut x: usize, n: usize) -> usize {
    for _ in 0..n {
        x = ((x * 64) ^ x) % PRUNE;
        x = ((x / 32) ^ x) % PRUNE;
        x = ((x * 2048) ^ x) % PRUNE;
    }
    x
}

fn solve1(input: &Input) -> usize {
    input.iter().copied().map(|x| evolve(x, 2000)).sum()
}

fn solve2(input: &Input) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 37327623);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), ());
    }
}
