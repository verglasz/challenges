use core::ascii;
use std::{cmp::Ordering, collections::HashSet, ops::RangeInclusive};

use utils::{decimals::mask10, get_stdinput};

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(&input[0]);
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input<'a> = Vec<RangeInclusive<&'a str>>;

fn parse(line: &str) -> Input<'_> {
    line.split(',')
        .map(|range| range.split_once('-').expect("should contain -"))
        .map(|(a, b)| a..=b)
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut total = 0;
    for range in input {
        let first = find_rep2_above(range.start(), false);
        let next_last = find_rep2_above(range.end(), true);
        for id in first..next_last {
            total += make_id(id, 2);
        }
    }
    total
}

fn make_id(base: usize, reps: usize) -> usize {
    let digs = mask10(base);
    let mut total = 1;
    for _ in 0..(reps - 1) {
        total = total * digs + 1;
    }
    total * base
}

fn find_rep2_above(start: &str, strict: bool) -> usize {
    assert!(start.len() > 0);
    if start.len() % 2 != 0 {
        10_usize.pow((start.len() / 2) as u32)
    } else {
        let (top, bot) = start.split_at(start.len() / 2);
        let top = top.parse::<usize>().expect("numbers in ids");
        let bot = bot.parse::<usize>().expect("numbers in ids");
        if top > bot {
            top
        } else if top < bot {
            top + 1
        } else {
            // top == bot, if strict we still increment
            top + strict as usize
        }
    }
}

// fn find_last(end: &str) -> usize { todo!() }

fn solve2(input: &Input) -> usize {
    let mut total = 0;
    let mut ids = HashSet::new();
    for range in input {
        print!("range {range:?}: ");
        let first = find_rep_n_above(range.start(), false, range.end().len());
        let next_last = find_rep_n_above(range.end(), true, range.end().len());
        for (f, l) in first.zip(next_last) {
            // println!("f:{f:?} l:{l:?}");
            let fa = f.1;
            let la = l.1;
            assert!(f.0 == l.0);
            for id in fa..la {
                let made_id = make_id(id, f.0);
                print!("{made_id}, ");
                ids.insert(made_id);
            }
        }
        println!("");
        total += ids.drain().sum::<usize>();
    }
    total
}

/// return a vector of repeat_times, base_id
fn find_rep_n_above(
    start: &str,
    strict: bool,
    max_reps: usize,
) -> impl Iterator<Item = (usize, usize)> {
    assert!(start.len() > 0);
    (2..=max_reps).map(move |reps| {
        let digs = start.len() / reps;
        let num = if start.len() % reps != 0 {
            10_usize.pow(digs as u32)
        } else {
            let mut chks = start.as_bytes().chunks(digs).map(|c| {
                str::from_utf8(c)
                    .expect("was ascii nums so no broken chars")
                    .parse::<usize>()
                    .expect("number")
            });
            let fst = chks.next().expect("at least one piece");
            // we need to go above fst if the first different subsequent digit group
            // is above fst, or if they're all exactly equal (and we're strict)
            let mut increment = strict;
            for c in chks {
                increment = match c.cmp(&fst) {
                    Ordering::Greater => true,
                    Ordering::Equal => continue,
                    Ordering::Less => false,
                };
                break;
            }
            fst + if increment { 1 } else { 0 }
        };
        (reps, num)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines().next().unwrap());
        assert_eq!(solve1(&input), 1227775554);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines().next().unwrap());
        assert_eq!(solve2(&input), 4174379265);
    }
}
