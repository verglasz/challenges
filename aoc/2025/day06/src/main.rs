#![feature(ascii_char)]
use utils::{decimals::from_ascii_digit_skipping, get_stdinput, grid::VecMat};

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()), input.len());
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(input);
    println!("sol2: {p2}");
}
type Input = (VecMat<usize>, Vec<u8>);

fn parse<'a>(mut lines: impl Iterator<Item = &'a str>, total: usize) -> Input {
    let nums = (1..total)
        .map(|_| {
            lines
                .next()
                .expect("numlines")
                .split_whitespace()
                .map(|x| x.parse().expect("numbers in numlines"))
                .collect()
        })
        .collect();
    let ops = lines
        .next()
        .expect("opsline")
        .split_whitespace()
        .map(|x| x.as_bytes()[0])
        .collect();
    (VecMat::new(nums).expect("mat"), ops)
}

fn solve1((nums, ops): &Input) -> usize {
    let mut total = 0;
    let nums = nums.transpose();
    for (i, op) in ops.iter().enumerate() {
        total += match op {
            b'+' => nums[i].iter().sum::<usize>(),
            b'*' => nums[i].iter().product::<usize>(),
            _ => panic!("unknown op {}", *op as char),
        };
    }
    total
}

fn solve2(mut input: Vec<String>) -> usize {
    let ops = input.pop().expect("ops line");
    // println!("{ops:?}");
    let nums = input
        .into_iter()
        .map(|line| {
            line.into_bytes()
                .into_iter()
                .map(|x| x.as_ascii().expect("all ascii"))
                .collect()
        })
        .collect();
    let nums = VecMat::new(nums).expect("actually matrix").transpose();
    for r in nums.iter_rows() {
        // println!("{}", r.as_str());
    }
    let mut nums = nums
        .iter_rows()
        .map(|row| row.as_str().trim().parse::<usize>());
    let mut total = 0;
    for op in ops.split_whitespace() {
        let iter = nums.by_ref().map_while(|x| x.ok()); // until empty
        total += match op {
            "+" => iter.sum::<usize>(),
            "*" => iter.product(),
            _ => panic!("unrecognised op '{op}'"),
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines(), 4);
        assert_eq!(solve1(&input), 4277556);
    }
    #[test]
    fn testinput() {
        let input: Vec<_> = include_str!("../input")
            .lines()
            .map(str::to_string)
            .collect();
        let parsed = parse(input.iter().map(|x| x.as_str()), input.len());
        let p1 = solve1(&parsed);
        assert_eq!(p1, 4719804927602);
        let p2 = solve2(input);
        assert_eq!(p2, 9608327000261);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = input.lines().map(|x| x.to_string()).collect();
        assert_eq!(solve2(input), 3263827);
    }
}
