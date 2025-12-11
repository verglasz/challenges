use nom::{
    bytes::complete::is_not,
    character::{
        char,
        complete::{digit0, space0, space1},
    },
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    Parser,
};
use pathfinding::{directed::astar, prelude::bfs};
use rayon::prelude::*;
use std::{collections::HashMap, str::FromStr};
use utils::get_stdinput;

fn main() {
    let input: Vec<_> = get_stdinput().collect();
    let parsed = parse(input.iter().map(|x| x.as_str()));
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2}");
}
type Input = Vec<Problem>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Problem {
    target: u64,
    buttons: Vec<BT>,
    jolts: Vec<JT>,
}
type BT = Vec<u8>; //button type
type JT = u16; //jolt type

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    state: u64,
}

fn make_pattern(lights: impl Iterator<Item = u8>) -> u64 {
    let mut btn = 0;
    for l in lights {
        btn |= 1 << l;
    }
    btn
}

impl FromStr for Problem {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = delimited(char('['), is_not("]"), char(']'));
        let button = delimited(
            char('('),
            separated_list0(char(','), digit0.map(|x: &str| x.parse().unwrap())),
            char(')'),
        );
        // .map(|x| make_pattern(x.iter().copied()));
        let jolt = delimited(
            char('{'),
            separated_list0(char(','), digit0.map(|x: &str| x.parse().unwrap())),
            char('}'),
        );
        let mut line = separated_pair(
            separated_pair(pattern, space1, separated_list1(space1, button)),
            space0,
            jolt,
        );
        let (_, ((target, buttons), jolts)) = line.parse(s)?;
        let target = make_pattern(
            target
                .bytes()
                .enumerate()
                .filter_map(|(i, x)| (x == b'#').then_some(i as u8)),
        );
        Ok(Self {
            target,
            buttons,
            jolts,
        })
    }
}

impl Problem {
    fn solve1(&self) -> usize {
        let buttons: Vec<_> = self
            .buttons
            .iter()
            .map(|x| make_pattern(x.iter().copied()))
            .collect();

        let sol =
            bfs(&0, |x| toggle(*x, &buttons), |x| *x == self.target).expect("solution exists");
        sol.len() - 1
    }

    fn solve2(&self) -> usize {
        if self.jolts.iter().copied().max().unwrap() == 11256 {
            match self.jolts.len() {
                0..=8 => return solve2_u8::<8>(&self.jolts, &self.buttons),
                9..=16 => return solve2_u8::<16>(&self.jolts, &self.buttons),
                x => todo!("{x} jolts are too many"),
            }
        } else {
            match self.jolts.len() {
                0..=8 => return solve2_i16::<8>(&self.jolts, &self.buttons),
                9..=12 => return solve2_i16::<12>(&self.jolts, &self.buttons),
                x => todo!("{x} jolts are too many"),
            }
        }
        // let mut start = vec![0; self.jolts.len()];
        let mut target = self.jolts.clone();
        let mut buttons = self.buttons.clone();
        buttons.sort_by_key(|b| !b.len());
        s2(&buttons[..], &mut target, isize::MAX)
            .try_into()
            .expect("positive")
    }
}

fn solve2_i16<const N: usize>(target: &[JT], buttons: &[BT]) -> usize {
    let mut arr_target = [0i16; N];
    for i in 0..target.len() {
        arr_target[i] = target[i] as i16;
    }
    let arr_buttons: Vec<_> = buttons
        .iter()
        .map(|b| make_arr(b).map(|x| x as i16))
        .collect();
    s2_i16(
        arr_target,
        &arr_buttons,
        isize::MAX,
        //&mut HashMap::new(),
    )
    .try_into()
    .expect("nonnegative")
}

fn s2_i16<const N: usize>(
    mut target: [i16; N],
    buttons: &[[i16; N]],
    best: isize,
    // memo: &mut HashMap<[i16; N], isize>,
) -> isize {
    let mut presses = 1;
    if let Some(i) = buttons
        .iter()
        .copied()
        .filter(|x| *subarr16(target, x).iter().min().unwrap() >= 0)
        .reduce(|a, b| addarr16(a, &b))
        .and_then(|x| {
            x.into_iter()
                .enumerate()
                .find_map(|(i, x)| (x == 1).then_some(i))
        })
    {
        // there is only one button that increases something
        // we just press this one button as much as we need
        let tgt = target[i];
        let btn = buttons
            .iter()
            .filter(|x| *subarr16(target, x).iter().min().unwrap() >= 0)
            .find(|b| b[i] == 1)
            .unwrap();
        for _ in 0..tgt {
            target = subarr16(target, btn);
        }
        if target == [0; N] {
            return tgt as isize;
        }
        presses += tgt as isize;
    }
    let mut min = best - presses;
    // if let Some(x) = memo.get(&target) {
    //     // return *x;
    // }
    if target.iter().copied().max().unwrap() as isize >= min {
        return best;
    }

    for (i, b) in buttons.iter().enumerate() {
        let t = subarr16(target, b);
        if t == [0; N] {
            return presses;
        }
        if t.iter().any(|&x| x < 0) {
            continue;
        }
        let n = s2_i16(
            t,
            &buttons[i..],
            min,
            //memo
        );
        min = min.min(n);
    }
    // memo.insert(target, min + 1);
    min + presses
}

fn addarr16<const N: usize>(mut target: [i16; N], b: &[i16; N]) -> [i16; N] {
    for (i, &b) in b.iter().enumerate() {
        target[i] += b;
    }
    target
}

fn subarr16<const N: usize>(mut target: [i16; N], b: &[i16; N]) -> [i16; N] {
    for (i, &b) in b.iter().enumerate() {
        target[i] -= b;
    }
    target
}

fn solve2_u8<const N: usize>(target: &[JT], buttons: &[BT]) -> usize {
    let mut arr_target = [0u8; N];
    for i in 0..target.len() {
        arr_target[i] = target[i] as u8;
    }
    let arr_buttons: Vec<_> = buttons.iter().map(|b| make_arr(b)).collect();
    dbg!(arr_buttons
        .iter()
        .copied()
        .reduce(addarr)
        .unwrap()
        .iter()
        .filter(|x| **x != 0)
        .min()
        .unwrap());
    s2_u8(arr_target, &arr_buttons, isize::MAX)
        .try_into()
        .expect("nonnegative")
}

fn make_arr<const N: usize>(button: &[u8]) -> [u8; N] {
    let mut arr = [0; N];
    for b in button {
        arr[*b as usize] = 1;
    }
    arr
}

fn s2_u8<const N: usize>(target: [u8; N], buttons: &[[u8; N]], best: isize) -> isize {
    let mut min = best - 1;
    for (i, b) in buttons.iter().enumerate() {
        let Some(t) = subarr(target, *b) else {
            continue;
        };
        if t == [0; N] {
            return 1;
        }
        let n = s2_u8(t, &buttons[i..], min);
        min = min.min(n);
    }
    min + 1
}

fn addarr<const N: usize>(mut target: [u8; N], b: [u8; N]) -> [u8; N] {
    for (i, &b) in b.iter().enumerate() {
        target[i] += b;
    }
    target
}

fn subarr<const N: usize>(mut target: [u8; N], b: [u8; N]) -> Option<[u8; N]> {
    for (i, &b) in b.iter().enumerate() {
        if target[i] < b {
            return None;
        }
        target[i] -= b;
    }
    Some(target)
}

fn s2(buttons: &[BT], target: &mut [JT], mut best: isize) -> isize {
    if target.iter().all(|x| *x == 0) {
        return 0;
    }
    let mut min = best - 1;
    for (i, b) in buttons.iter().enumerate() {
        if min < 1 {
            break;
        }
        // println!("trying button {i}: {b:?}");
        if joggle(b, target).is_err() {
            // println!("can't do button {i}");
            continue;
        }
        // println!("new state {target:?}");
        let n = s2(&buttons[i..], target, min);
        min = min.min(n);
        // println!("new min {min:?}");
        unjoggle(b, target);
    }
    min + 1
}

fn unjoggle(b: &[u8], target: &mut [u16]) {
    for x in b {
        target[*x as usize] += 1;
    }
}

fn joggle(b: &[u8], target: &mut [JT]) -> Result<(), ()> {
    if b.iter().any(|x| target[*x as usize] == 0) {
        return Err(());
    }
    for x in b {
        target[*x as usize] -= 1;
    }
    Ok(())
}

// fn jog<'a, 'b>(thing: &'b (Vec<JT>, &'a [BT])) -> impl Iterator<Item = (Vec<JT>, usize)> + 'a {
//     None.into_iter()
// }

// fn joggle<'b>(
//     x: &'b [JT],
//     buttons: Rc<Vec<BT>>,
// ) -> Box<dyn Iterator<Item = (Vec<JT>, usize)> + 'static> {
//     let jolts = x.to_vec();

//     // Box::new((0..buttons.len()).map(move |i| (j1(buttons.as_ref()[i], jolts.clone()), 1)))
// }

fn heur(state: &[JT], target_jolts: &[JT]) -> usize {
    state
        .iter()
        .copied()
        .zip(target_jolts.iter().copied())
        .map(|(s, t)| t.checked_sub(s).unwrap_or(JT::MAX))
        .max()
        .unwrap() as usize
}

fn j1(mut bpos: u64, mut jolts: Vec<JT>) -> Vec<JT> {
    for p in jolts.iter_mut() {
        if (bpos & 1) != 0 {
            *p += 1;
        }
        bpos >>= 1;
    }
    jolts
}

fn toggle(x: u64, buttons: &[u64]) -> impl Iterator<Item = u64> {
    buttons.iter().map(move |b| x ^ b)
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Input {
    lines
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve1(input: &Input) -> usize {
    input.iter().map(|x| x.solve1()).sum()
}

fn solve2(input: &Input) -> usize {
    println!(
        "max jolts {} (max val {}) max btn len {} ",
        input.iter().map(|i| i.jolts.len()).max().unwrap(),
        input
            .iter()
            .map(|i| i.jolts.iter().max().unwrap())
            .max()
            .unwrap(),
        input
            .iter()
            .map(|i| i.buttons.iter().map(|bs| bs.len()).max().unwrap())
            .max()
            .unwrap()
    );
    // 14767
    // input
    //     .iter()
    //     .map(|x| x.jolts.iter().copied().max().unwrap() as usize)
    //     .sum()
    input
        .par_iter()
        .enumerate()
        .map(|(i, p)| {
            let s = p.solve2();
            println!("{i}: {s}");
            s
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 7);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 33);
    }
}
