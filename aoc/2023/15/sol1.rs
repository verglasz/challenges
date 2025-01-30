#!/usr/bin/env rust-script

use std::io::stdin;

fn main() {
    let input = stdin()
        .lines()
        .next()
        .expect("input has one line")
        .expect("no error reading input line");

    println!("{}", sol1(input.as_bytes()));
    println!("{}", sol2(input.as_bytes()));
}

fn sol1(input: &[u8]) -> usize {
    let mut tot: usize = 0;
    let mut hash: u8 = 0;
    for &c in input {
        match c {
            b',' => {
                tot += hash as usize;
                hash = 0;
            }
            c => hash = hash.wrapping_add(c).wrapping_mul(17),
        }
    }
    tot + hash as usize
}

enum Op {
    Pop,
    Insert(usize),
}

fn parsenum(num: &[u8]) -> usize {
    num.iter().fold(0, |n, &c| {
        n.wrapping_mul(10).wrapping_add((c - b'0') as usize)
    })
}

fn parsecmd(cmd: &[u8]) -> (u8, &[u8], Op) {
    let mut hash = 0;
    let mut chrs = cmd.iter().enumerate();
    loop {
        match chrs.next().expect("cmd has - or = before end") {
            (i, b'-') => break (hash, &cmd[..i], Op::Pop),
            (i, b'=') => {
                let (part, num) = cmd.split_at(i);
                break (hash, part, Op::Insert(parsenum(&num[1..])));
            }
            (_, &c) => hash = hash.wrapping_add(c).wrapping_mul(17),
        }
    }
}

fn sol2(input: &[u8]) -> usize {
    let mut boxes = vec![Vec::new(); 256];
    for cmd in input.split(|&c| c == b',') {
        let (hash, part, op) = parsecmd(cmd);
        let mut _box = &mut boxes[hash as usize];
        let idx = _box
            .iter()
            .enumerate()
            .find_map(|(i, &(label, _))| (label == part).then_some(i));
        match (idx, op) {
            (None, Op::Insert(n)) => _box.push((part, n)),
            (Some(i), Op::Pop) => {
                _box.remove(i);
            }
            (Some(i), Op::Insert(n)) => _box[i].1 = n,
            (None, Op::Pop) => continue,
        }
    }
    // dbg!(&boxes);
    boxes.iter().enumerate().fold(0, |tot, (hash, _box)| {
        tot + _box
            .iter()
            .enumerate()
            .map(|(i, &(_, n))| (hash + 1) * (i + 1) * n)
            .sum::<usize>()
    })
}
