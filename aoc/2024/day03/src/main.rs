use regex::Regex;
use utils::get_stdinput;

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = String;

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let mut s = "".to_string();
    lines.for_each(|l| s.push_str(l.as_ref()));
    s
}

fn sum_muls(s: &str) -> isize {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .expect("valid regex")
        .captures_iter(s)
        .map(|mul| {
            let a = mul[1]
                .parse::<isize>()
                .expect("mul args should be valid numbers");
            let b = mul[2]
                .parse::<isize>()
                .expect("mul args should be valid numbers");
            (a, b)
        })
        .map(|(a, b)| a * b)
        .sum()
}

fn solve1(input: &Input) -> isize {
    sum_muls(input)
}

fn solve2(input: &Input) -> isize {
    let mut input = &input[..];
    let mut sum = 0;
    loop {
        let (slice, rest) = match input.find("don't()") {
            Some(i) => input.split_at(i),
            None => (input, ""),
        };
        sum += sum_muls(slice);
        match rest.find("do()") {
            Some(i) => input = &rest[i..],
            None => break,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 161);
    }

    #[test]
    fn test2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 48);
    }
}
