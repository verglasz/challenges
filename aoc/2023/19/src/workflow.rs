use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, RangeInclusive},
    ops::{Index, IndexMut},
};

// type Bound = RangeInclusive<u16>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Bound {
    Upper(u16),
    Lower(u16),
}

impl Bound {
    fn contains(&self, value: u16) -> bool {
        match *self {
            Self::Upper(n) => value < n,
            Self::Lower(n) => value > n,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Step {
    prop: Prop,
    bound: Bound,
    dest: Label,
}

type Prop = u8;

fn parse_cond(s: &str) -> (Prop, u8, u16) {
    let (pt, n) = s.split_at(2);
    let &[prop, typ] = pt.as_bytes() else {
        panic!("Invalid type in condition");
    };
    let n = n.parse().expect("Invalid number in condition");
    (prop, typ, n)
}

impl Step {
    fn parse(s: &str) -> Self {
        let (cond, target) = s.split_once(':').expect("No : in step input");
        let (prop, typ, n) = parse_cond(cond);
        use Bound::*;
        let bound = if typ == b'<' {
            Upper(n)
        } else if typ == b'>' {
            Lower(n)
        } else {
            panic!("Invalid type in step input");
        };
        Self {
            prop,
            bound,
            dest: parse_label(target),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Label([u8; 4]);

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zero = self.0.iter().position(|&b| b == 0).unwrap_or(4);
        let s = std::str::from_utf8(&self.0[..zero]).unwrap();
        write!(f, "'{}'", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Steps {
    conds: Vec<Step>,
}

impl Steps {
    fn next(&self, entry: Entry) -> Label {
        for step in &self.conds {
            if step.bound.contains(entry[step.prop]) {
                return step.dest;
            }
        }
        panic!("No matching step in workflow, not even the end");
    }
}

impl From<Vec<Step>> for Steps {
    fn from(value: Vec<Step>) -> Self {
        Steps { conds: value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Workflows {
    inner: HashMap<Label, Steps>,
}

impl Workflows {
    fn accept(&self, entry: Entry) -> Option<usize> {
        let mut workflow = START;
        loop {
            if workflow == ACCEPT {
                break Some(entry.sum());
            } else if workflow == REJECT {
                break None;
            } else {
                workflow = self.inner[&workflow].next(entry);
            }
        }
    }

    fn tree(&self, start: Label, block: Block) -> usize {
        let workflow = &self.inner[&start];
        let mut total = 0;
        for (part, dest) in block.fragments(workflow) {
            let Some(part) = part else { continue };
            if dest == REJECT {
                continue;
            } else if dest == ACCEPT {
                total += part.size();
            } else {
                total += self.tree(dest, part);
            }
        }
        total
    }
}

impl From<HashMap<Label, Steps>> for Workflows {
    fn from(value: HashMap<Label, Steps>) -> Self {
        Workflows { inner: value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RawEntry<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T> Index<Prop> for RawEntry<T> {
    type Output = T;

    fn index(&self, prop: Prop) -> &Self::Output {
        match prop {
            b'x' => &self.x,
            b'm' => &self.m,
            b'a' => &self.a,
            b's' => &self.s,
            _ => panic!("Invalid property in step input"),
        }
    }
}

impl<T> IndexMut<Prop> for RawEntry<T> {
    fn index_mut(&mut self, prop: Prop) -> &mut Self::Output {
        match prop {
            b'x' => &mut self.x,
            b'm' => &mut self.m,
            b'a' => &mut self.a,
            b's' => &mut self.s,
            _ => panic!("Invalid property in step input"),
        }
    }
}

type Entry = RawEntry<u16>;
type SingleBlock = RangeInclusive<u16>;
type Block = RawEntry<SingleBlock>;

type OneTwo<T, P = T> = (Option<T>, Option<P>);

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// enum OneTwo<T, P = T> {
//     Fst(T),
//     Snd(P),
//     Both(T, P),
// }

// impl<T, P> OneTwo<T, P> {
//     fn fst(self) -> Option<T> {
//         match self {
//             Self::Fst(t) | Self::Both(t, _) => Some(t),
//             Self::Snd(_) => None,
//         }
//     }

//     fn snd(self) -> Option<P> {
//         match self {
//             Self::Snd(p) | Self::Both(_, p) => Some(p),
//             Self::Fst(_) => None,
//         }
//     }
// }

impl Block {
    /// Split the block into fragments that are taken by successive steps of the workflow.
    fn fragments(self, workflow: &Steps) -> impl Iterator<Item = (Option<Block>, Label)> + '_ {
        let mut current = Some(self);
        workflow.conds.iter().map_while(move |step| {
            let (taken, rest) = current.take()?.split_off_taken(step.prop, step.bound);
            current = rest;
            Some((taken, step.dest))
        })
    }

    /// Split off the part of the block that is taken by the condition.
    /// Returns the part that is inside as fst, and the part that is outside as snd.
    /// If only one of them is non-empty, it is returned as the corresponding OneTwo variant.
    fn split_off(input: SingleBlock, cond: Bound) -> OneTwo<SingleBlock> {
        let (start, end) = input.into_inner();
        match cond {
            Bound::Upper(n) => {
                if end < n {
                    (Some(start..=end), None)
                } else if start > n {
                    (None, Some(start..=end))
                } else {
                    let fst = start..=n - 1;
                    let snd = n..=end;
                    (Some(fst), Some(snd))
                }
            }
            Bound::Lower(n) => {
                if start > n {
                    (Some(start..=end), None)
                } else if end < n {
                    (None, Some(start..=end))
                } else {
                    let fst = n..=end;
                    let snd = start..=n - 1;
                    (Some(fst), Some(snd))
                }
            }
        }
    }

    fn split_off_taken(self, prop: u8, bound: Bound) -> OneTwo<Self> {
        match Self::split_off(self[prop].clone(), bound) {
            (Some(t), Some(l)) => {
                let mut taken = self.clone();
                let mut left = self;
                taken[prop] = t;
                left[prop] = l;
                (Some(taken), Some(left))
            }
            (Some(_), None) => (Some(self), None),
            (None, Some(_)) => (None, Some(self)),
            (None, None) => (None, None),
        }
    }

    fn size(&self) -> usize {
        let x = self.x.end() - self.x.start() + 1;
        let m = self.m.end() - self.m.start() + 1;
        let a = self.a.end() - self.a.start() + 1;
        let s = self.s.end() - self.s.start() + 1;
        x as usize * m as usize * a as usize * s as usize
    }
}

impl Entry {
    fn parse(line: &str) -> Self {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        let items = line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(parse_cond);
        for (prop, typ, n) in items {
            match prop {
                b'x' => x = n,
                b'm' => m = n,
                b'a' => a = n,
                b's' => s = n,
                _ => panic!("Invalid property in entry input"),
            }
            assert_eq!(typ, b'=', "Invalid type in entry {:?} in input", &[typ]);
        }
        Self { x, m, a, s }
    }

    fn sum(&self) -> usize {
        self.x as usize + self.m as usize + self.a as usize + self.s as usize
    }
}

struct KeepLast<I, T> {
    inner: I,
    last: T,
}

impl<I: Iterator> KeepLast<I, I::Item> {
    fn new(iter: impl IntoIterator<IntoIter = I>) -> Option<Self> {
        let mut inner = iter.into_iter();
        let last = inner.next()?;
        Some(Self { inner, last })
    }
}

impl<I: Iterator> Iterator for KeepLast<I, I::Item> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.inner.next()?;
        std::mem::swap(&mut self.last, &mut next);
        Some(next)
    }
}

fn parse_label(s: &str) -> Label {
    let mut label = [0; 4];
    let s = s.as_bytes();
    label[..s.len()].copy_from_slice(s);
    Label(label)
}

const START: Label = Label(*b"in\0\0");
const ACCEPT: Label = Label(*b"A\0\0\0");
const REJECT: Label = Label(*b"R\0\0\0");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split() {
        let block = super::Block {
            x: 1..=5,
            m: 1..=5,
            a: 1..=5,
            s: 1..=1,
        };
    }
}
