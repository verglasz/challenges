use std::{io::stdin, ops::Deref};

fn main() {
    let lines = stdin().lines().map(|line| line.unwrap());
    let input = parse_input(lines);
    println!("input: {:?}", input);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

type Input = ();

fn parse_input<T: Deref<Target = str>>(mut lines: impl Iterator<Item = T>) -> Input {}

fn part1(input: &Input) -> usize {
    0
}

fn part2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test");
        let input = parse_input(input.lines());
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test");
        let input = parse_input(input.lines());
        assert_eq!(part1(&input), 0);
    }
}
