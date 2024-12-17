use core::fmt;
use std::{
    collections::HashSet,
    ops::{ControlFlow, Deref, Index, IndexMut},
    thread,
};

use utils::{
    get_stdinput,
    prettyprinting::DisplayAsDebug,
    types::{Both, Either},
};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = (Regs, Vec<ThreeBit>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    regs: Regs,
    pc: usize,
    insts: Vec<ThreeBit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpResult {
    Out(ThreeBit),
    Jump(usize),
    None,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ThreeBit(u8);

impl fmt::Display for ThreeBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    fn op_type(self) -> OpType {
        use OpCode::*;
        match self {
            Adv | Bst | Out | Bdv | Cdv => OpType::Combo,
            Bxl | Jnz | Bxc => OpType::Lit,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Regs {
    a: usize,
    b: usize,
    c: usize,
}

impl IndexMut<Reg> for Regs {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        match index {
            Reg::A => &mut self.a,
            Reg::B => &mut self.b,
            Reg::C => &mut self.c,
        }
    }
}

impl Index<Reg> for Regs {
    type Output = usize;

    fn index(&self, index: Reg) -> &Self::Output {
        match index {
            Reg::A => &self.a,
            Reg::B => &self.b,
            Reg::C => &self.c,
        }
    }
}

impl State {
    fn new(regs: Regs, pc: usize, insts: Vec<ThreeBit>) -> Self {
        Self { regs, pc, insts }
    }

    fn init(a: usize, b: usize, c: usize, insts: &[u8]) -> Self {
        Self::new(
            Regs { a, b, c },
            0,
            insts
                .iter()
                .map(|&n| ThreeBit::new(n).expect("valid"))
                .collect(),
        )
    }

    fn run(&mut self) -> Vec<ThreeBit> {
        let mut output = vec![];
        while let ControlFlow::Continue(out) = self.step() {
            if let Some(out) = out {
                output.push(out);
            }
        }
        output
    }

    fn step(&mut self) -> ControlFlow<(), Option<ThreeBit>> {
        step(&mut self.regs, &mut self.pc, &self.insts)
    }
}

fn step(regs: &mut Regs, pc: &mut usize, insts: &[ThreeBit]) -> ControlFlow<(), Option<ThreeBit>> {
    let Some(inst) = insts.get(*pc) else {
        return ControlFlow::Break(());
    };
    let op = inst.op();
    let operand = insts[*pc + 1];
    *pc += 2;
    let out = match regs.perform(op, operand) {
        OpResult::Out(out) => Some(out),
        OpResult::Jump(jump) => {
            *pc = jump;
            None
        }
        OpResult::None => None,
    };
    ControlFlow::Continue(out)
}

impl Regs {
    fn perform(&mut self, op: OpCode, operand: ThreeBit) -> OpResult {
        let op_type = op.op_type();
        let val = match op_type {
            OpType::Lit => operand.lit(),
            OpType::Combo => operand.combo(self).expect("combo valid"),
        };
        use OpCode::*;
        use OpResult::{Jump, None};
        match op {
            Adv => self.a = self.a >> val,
            Bdv => self.b = self.a >> val,
            Cdv => self.c = self.a >> val,
            Bxl => self.b ^= val,
            Bst => self.b = val % 8,
            Bxc => self.b ^= self.c,
            Jnz => {
                if self.a != 0 {
                    return Jump(val);
                }
            }
            Out => {
                return OpResult::Out(ThreeBit(val as u8 % 8));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpType {
    Lit,
    Combo,
}

impl From<ThreeBit> for OpCode {
    fn from(n: ThreeBit) -> Self {
        match n.0 {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

impl ThreeBit {
    fn new(n: u8) -> Option<Self> {
        (n < 8).then_some(Self(n))
    }

    fn op(self) -> OpCode {
        self.into()
    }

    fn lit(self) -> usize {
        self.0 as usize
    }

    fn combo(self, state: &Regs) -> Option<usize> {
        Some(match self.0 {
            0..=3 => self.lit(),
            4 => state.a,
            5 => state.b,
            6 => state.c,
            7 => None?,
            _ => unreachable!(),
        })
    }

    fn comb(self) -> ComboType {
        use ComboType::*;
        use Reg::*;
        match self.0 {
            n @ 0..=3 => Lit(n),
            4 => R(A),
            5 => R(B),
            6 => R(C),
            7 => Reserved,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComboType {
    Lit(u8),
    R(Reg),
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Reg {
    A,
    B,
    C,
}

fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let lines = &mut lines;
    let a = lines
        .next()
        .expect("A")
        .as_ref()
        .strip_prefix("Register A: ")
        .expect("A")
        .parse()
        .expect("A number");
    let b = lines
        .next()
        .expect("B")
        .as_ref()
        .strip_prefix("Register B: ")
        .expect("B")
        .parse()
        .expect("B number");
    let c = lines
        .next()
        .expect("C")
        .as_ref()
        .strip_prefix("Register C: ")
        .expect("C")
        .parse()
        .expect("C number");
    assert!(lines.next().expect("empty").as_ref().is_empty());
    let code = lines
        .next()
        .expect("code")
        .as_ref()
        .strip_prefix("Program: ")
        .expect("code")
        .split(",")
        .map(|s| ThreeBit::new(s.parse().expect("code is nums")))
        .collect::<Option<Vec<_>>>()
        .expect("code is valid");
    (Regs { a, b, c }, code)
}

fn solve1(input: &Input) -> String {
    let mut state = State::new(input.0, 0, input.1.clone());
    state
        .run()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn run_check(mut regs: Regs, insts: &[ThreeBit]) -> bool {
    let mut pc = 0;
    let mut left = insts;
    let mut seen: HashSet<(Regs, usize)> = [(regs, pc)].into_iter().collect();
    while let ControlFlow::Continue(out) = step(&mut regs, &mut pc, insts) {
        if !seen.insert((regs, pc)) {
            // loop
            return false;
        }
        let Some(out) = out else {
            // no output, keep going
            continue;
        };
        let Some((fst, rest)) = left.split_first() else {
            // we got output but was expecting none
            return false;
        };
        if *fst != out {
            // output doesn't match
            return false;
        }
        left = rest;
    }
    left.is_empty()
}

fn bad_solve2((orig_regs, insts): &Input) -> usize {
    let orig_regs = *orig_regs;
    thread::scope(|s| {
        let mut threads = vec![];
        for t in 0..6 {
            let jh = s.spawn(move || {
                for a in 0.. {
                    let mut regs = orig_regs;
                    regs.a ^= a * 6 + t;
                    if run_check(regs, insts) {
                        return Some(regs.a);
                    }
                    if a % 1_000_000 == 0 {
                        println!("{:5}M values of a checked on thread {}", a / 1_000_000, t);
                    }
                }
                None
            });
            threads.push(jh);
        }
        loop {
            let Both(done, waiting): Both<Vec<_>, Vec<_>> = threads
                .into_iter()
                .map(|jh| {
                    if jh.is_finished() {
                        Either::Left(jh)
                    } else {
                        Either::Right(jh)
                    }
                })
                .collect();
            for d in done {
                let t = d.thread().id();
                let res = d.join().unwrap_or_else(|e| {
                    panic!("thread {:?} panicked: {:?}", t, e);
                });
                if let Some(d) = res {
                    // we need to cancel the threads since we don't want to wait...
                    // this design is problematic, should have each thread send back the result on a mpsc
                    // channel instead probably
                    return d;
                }
            }
            threads = waiting;
        }
    })
}

impl fmt::Display for ComboType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ComboType::*;
        match self {
            Lit(n) => write!(f, "{}", n),
            R(r) => write!(f, "{:?}", r),
            Reserved => write!(f, "!"),
        }
    }
}

struct DisplayInsts<'a>(&'a [ThreeBit], usize);

impl fmt::Display for DisplayInsts<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.chunks(2).enumerate().map(|(i, w)| {
                let inst = w[0].op();
                let operand = w[1];
                let pc = self.1 + i * 2;
                let s = if inst.op_type() == OpType::Lit {
                    format!("{pc}: {:?}({:?})", inst, operand.lit())
                } else {
                    let val = operand.comb();
                    format!("{pc}: {:?}({})", inst, val)
                };
                DisplayAsDebug(s)
            }))
            .finish()
    }
}

fn solve2((orig_regs, insts): &Input) -> usize {
    // let orig_regs = *orig_regs;
    println!("instructions:");
    println!("aligned:");
    println!("{:#}", DisplayInsts(&insts[..], 0));
    println!("offset:");
    println!("{:#}", DisplayInsts(&insts[1..insts.len() - 1], 1));
    0
}

macro_rules! bits {
    ($($n:expr),*) => {
        vec![$(ThreeBit::new($n).expect("valid")),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut state = State::init(0, 0, 9, &[2, 6]);
        state.run();
        assert_eq!(state.regs.b, 1);
    }

    #[test]
    fn test_case2() {
        let mut state = State::init(10, 0, 0, &[5, 0, 5, 1, 5, 4]);
        let out = state.run();
        assert_eq!(out, bits![0, 1, 2]);
    }

    #[test]
    fn test_case3() {
        let mut state = State::init(2024, 0, 0, &[0, 1, 5, 4, 3, 0]);
        let out = state.run();
        assert_eq!(state.regs.a, 0);
        assert_eq!(out, bits![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_case4() {
        let mut state = State::init(0, 29, 0, &[1, 7]);
        state.run();
        assert_eq!(state.regs.b, 26);
    }

    #[test]
    fn test_case5() {
        let mut state = State::init(0, 2024, 43690, &[4, 0]);
        state.run();
        assert_eq!(state.regs.b, 44354);
    }

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test2() {
        let input = include_str!("../test2");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 117440);
    }
}
