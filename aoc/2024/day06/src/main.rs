use utils::{get_stdinput, grid::VecMat};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = VecMat<u8>;

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let data = lines
        .filter(|s| !s.as_ref().trim().is_empty())
        .map(|l| l.as_ref().trim().as_bytes().to_vec())
        .collect();
    Input::new(data).expect("input should be grid map")
}

fn solve1(input: &Input) -> () {}

fn solve2(input: &Input) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), ());
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), ());
    }
}
