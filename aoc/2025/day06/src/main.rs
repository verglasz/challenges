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
        match op {
            b'+' => {
                total += nums[i].iter().sum::<usize>();
            }
            b'*' => {
                total += nums[i].iter().product::<usize>();
            }
            _ => panic!("unknown op {}", *op as char),
        }
    }
    total
}

fn solve2(mut input: Vec<String>) -> usize {
    let ops = input.pop().expect("ops line");
    println!("{ops:?}");
    let ops = ops.as_ascii().expect("ops..");
    let mut next = Some(0);
    let mut total = 0;
    while let Some(current) = next {
        next = ops[current + 1..]
            .iter()
            .position(|x| !x.is_whitespace())
            .map(|x| x + current + 1);
        let end = next.unwrap_or(input.iter().map(|x| x.len()).max().unwrap() + 1) - 1;
        let nums: Vec<_> = (current..end)
            .map(|i| {
                from_ascii_digit_skipping(
                    input
                        .iter()
                        .map(|row| row.as_bytes().get(i).copied().unwrap_or(b' ')),
                )
            })
            .collect();
        for row in input.iter() {
            println!("{}", &row[current..end.min(row.len())]);
        }
        println!("nums: {nums:?}");
        match ops[current].to_u8() {
            b'+' => {
                total += nums.iter().sum::<usize>();
            }
            b'*' => {
                total += nums.iter().product::<usize>();
            }
            _ => panic!("unknown op '{}' at {current}", ops[current].to_char()),
        }
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
        println!("sol1: {p1}");
        let p2 = solve2(input);
        println!("sol2: {p2}");
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = input.lines().map(|x| x.to_string()).collect();
        assert_eq!(solve2(input), 3263827);
    }
}
