use log::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Debug},
    io::stdin,
    ops::Deref,
};

use pathfinding::directed::strongly_connected_components::strongly_connected_components;

fn main() {
    let lines = stdin().lines().map(|line| line.unwrap());
    let input = parse_input(lines);
    info!("input: {:?}", input);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

type Input = Configuration;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct Label([u8; 2]);

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0[0] as char, self.0[1] as char))
    }
}

type ModuleMemory = HashMap<Label, bool>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    FlipFlop { state: bool },
    Memory { last: ModuleMemory },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    typ: ModuleType,
    outputs: Vec<Label>,
}

impl Module {
    fn new(typename: u8, outputs: Vec<Label>) -> Option<Self> {
        let typ = match typename {
            b'%' => ModuleType::FlipFlop { state: false },
            b'&' => ModuleType::Memory {
                last: Default::default(),
            },
            _ => return None,
        };
        Some(Self { typ, outputs })
    }

    fn flipflop(&self) -> Option<bool> {
        match &self.typ {
            ModuleType::FlipFlop { state } => Some(*state),
            _ => None,
        }
    }

    fn memory(&self) -> Option<&ModuleMemory> {
        match &self.typ {
            ModuleType::Memory { last } => Some(last),
            _ => None,
        }
    }

    fn is_memory(&self) -> bool {
        self.memory().is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Configuration {
    broadcast: Vec<Label>,
    modules: HashMap<Label, Module>,
}

const BROADCAST: Label = Label(*b"XX");

impl Configuration {
    fn pulse(&mut self) -> (usize, usize) {
        let mut next: VecDeque<_> = self
            .broadcast
            .iter()
            .map(|&l| (l, false, BROADCAST))
            .collect();
        let mut neg_pulses = 0;
        let mut pos_pulses = 0;
        while let Some((label, pulse, src)) = next.pop_front() {
            if pulse {
                pos_pulses += 1;
            } else {
                neg_pulses += 1;
            }
            debug!(
                "{:?} --{}--> {:?}",
                src,
                if pulse { "HIGH" } else { "LOW" },
                label
            );
            let Some(target) = self.modules.get_mut(&label) else {
                continue;
            };
            let generated = match &mut target.typ {
                ModuleType::FlipFlop { state } => {
                    if pulse {
                        continue;
                    }
                    *state = !*state;
                    *state
                }
                ModuleType::Memory { last } => {
                    last.insert(src, pulse);
                    !last.values().all(|&p| p)
                }
            };
            for &dest in &target.outputs {
                next.push_back((dest, generated, label));
            }
        }
        (pos_pulses, neg_pulses)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for (label, module) in &self.modules {
            s.push_str(&format!("{:?} -> {:?}\n", label, module));
        }
        todo!()
    }
}

fn parse_input<T: Deref<Target = str>>(mut lines: impl Iterator<Item = T>) -> Input {
    let mut broadcast = None;
    let mut modules = HashMap::new();
    let mut outputs = HashMap::new();
    for line in &mut lines {
        if line.is_empty() {
            continue;
        }
        let (module, dests) = line.split_once(" -> ").unwrap();
        let dests: Vec<_> = dests
            .split(", ")
            .map(|s| {
                let c = s.as_bytes();
                Label([c[0], c[1]])
            })
            .collect();
        if module == "broadcaster" {
            broadcast = Some(dests);
        } else {
            let cs = module.as_bytes();
            let module = Module::new(cs[0], dests.clone()).expect("invalid module type");
            let label = Label([cs[1], cs[2]]);
            modules.insert(label, module);
            outputs.insert(label, dests);
        }
    }
    let broadcast = broadcast.unwrap();
    for (&label, outs) in outputs.iter() {
        for dest in outs {
            let Some(module) = modules.get_mut(dest) else {
                continue;
            };
            match &mut module.typ {
                ModuleType::FlipFlop { .. } => (),
                ModuleType::Memory { last } => {
                    last.insert(label, false);
                }
            }
        }
    }

    Configuration { broadcast, modules }
}

fn part1(input: &Input) -> usize {
    let mut config = input.clone();
    let mut neg_pulses = 0;
    let mut pos_pulses = 0;
    for _ in 0..1000 {
        let (p, n) = config.pulse();
        pos_pulses += p;
        neg_pulses += n + 1;
    }
    neg_pulses * pos_pulses
}

fn part2(input: &Input) -> usize {
    let nodes: Vec<_> = input.modules.keys().copied().collect();
    let components = strongly_connected_components(&nodes, |n| {
        input.modules[n]
            .outputs
            .iter()
            .filter(|m| input.modules.contains_key(m))
            .copied()
    });
    for c in &components {
        println!("component of len ({}): {:?}", c.len(), c);
    }
    let schema = get_schema(input, components);
    println!("schema: {:?}", schema);
    let loops: Vec<_> = schema.values().map(|b| b.silly_loop()).collect();
    println!("loops: {:?}", loops);
    loops.into_iter().fold(1, |a, b| a * b / gcd(a, b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Block {
    output: Label,
    memfeed: Vec<usize>,
    ffchain: Vec<BlockFf>,
}

impl Block {
    fn construct<'a>(
        input: &Input,
        elements: impl Iterator<Item = &'a Label>,
    ) -> Option<(Label, Self)> {
        let mut mem = None;
        let mut flipflops = HashSet::new();
        for l in elements {
            if let Some(m) = input.modules[l].memory() {
                if mem.is_some() {
                    warn!("block has multiple memories");
                    return None;
                }
                mem = Some(*l);
            } else {
                flipflops.insert(*l);
            }
        }
        if flipflops.len() != 12 {
            return None;
        }
        let memlabel = mem?;
        let mem = &input.modules[&memlabel];
        let output = mem
            .outputs
            .iter()
            .find_map(|l| input.modules[l].memory().and(Some(*l)))?;
        let entry = *input.broadcast.iter().find(|&l| flipflops.contains(l))?;
        let mut memfeed = Vec::new();
        let mut ffchain = Vec::new();
        let mut next = Some(entry);
        while let Some(n) = next {
            let module = &input.modules[&n];
            let feeds = module.outputs.contains(&memlabel);
            let ff = BlockFf {
                label: n,
                state: false,
                feeds,
            };
            if mem.outputs.contains(&n) {
                memfeed.push(ffchain.len());
            }
            ffchain.push(ff);
            next = module
                .outputs
                .iter()
                .find(|&l| flipflops.contains(l))
                .copied();
        }
        let block = Block {
            memfeed,
            output,
            ffchain,
        };
        Some((entry, block))
    }

    fn reset(&mut self) {
        for ff in &mut self.ffchain {
            ff.state = false;
        }
    }

    fn silly_loop(&self) -> usize {
        let mut bit = 1;
        let mut magic = 0;
        for ff in &self.ffchain {
            if ff.feeds {
                magic |= bit;
            }
            bit <<= 1;
        }
        magic
    }

    fn find_loop(&mut self) -> LoopInfo {
        self.reset();
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        let mut low_pulses = Vec::new();
        loop {
            queue.push_back(0);
            while let Some(i) = queue.pop_front() {
                // mem pulses are irrelevant...
                // if i == 0xff {
                //     // process mem pulse
                //     let memout = !self.ffchain.iter().all(|ff| ff.state);
                //     out_pulses.push(memout);
                //     queue.extend(self.memfeed.iter().map(|&i| (i, memout)));
                //     continue;
                // }
                // pulse to ff
                let ff = &mut self.ffchain[i];
                if ff.state {}
                ff.state = true;
                low_pulses.push(todo!());
                visited.insert(ff.label, i);
            }
            break;
        }

        todo!()
    }
}

struct LoopInfo {
    start: usize,
    length: usize,
    low_pulses: Vec<(usize, usize)>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct BlockFf {
    label: Label,
    state: bool,
    feeds: bool,
}

impl Debug for BlockFf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let pre = if self.receives { "^" } else { "" };
        let pre = "";
        let post = if self.feeds { "^" } else { "" };
        write!(f, "{pre}{:?}{post}", self.label)
    }
}

type Schema = (HashMap<Label, Block>);

fn get_schema(input: &Input, components: Vec<Vec<Label>>) -> Schema {
    let mut blocks = HashMap::new();
    for c in components {
        if let Some((entry, block)) = Block::construct(input, c.iter()) {
            blocks.insert(entry, block);
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        let input = include_str!("../test");
        let mut input = parse_input(input.lines());
        println!("{:?}", input);
        assert_eq!(input.pulse(), (4, 7));
        println!("{:?}", input);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test");
        let input = parse_input(input.lines());
        assert_eq!(part1(&input), 32000000);
    }
    #[test]
    fn test_part1b() {
        let input = include_str!("../test2");
        let input = parse_input(input.lines());
        assert_eq!(part1(&input), 11687500);
    }
}
