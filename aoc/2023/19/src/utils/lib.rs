use hashbrown::{Equivalent, HashMap};
use std::{borrow::Borrow, hash::Hash, io::stdin};

pub mod decimals;
pub mod grid;
pub mod maths;
pub mod prettyprinting;
pub mod types;

pub mod graphs;

pub fn get_stdinput() -> impl Iterator<Item = String> {
    stdin().lines().map(|s| s.expect("input string unparsable"))
}

pub fn pairs<'a, T: 'a>(
    mut iter: impl Iterator<Item = &'a T>,
) -> impl Iterator<Item = (&'a T, &'a T)> {
    let fst = iter.next();
    iter.scan(fst, |state, x| state.replace(x).map(|y| (y, x)))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Counter<V: Hash + Eq>(HashMap<V, usize>);

impl<V: Hash + Eq> Counter<V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // fn add_ref<'b, Q>(&mut self, el:&'b Q)
    //     where V: Borrow<Q> + From<&'b Q>, Q: Equivalent<V> + Hash
    // {
    //     *self.0.entry_ref(el).or_insert(0) += 1;
    // }

    pub fn add_one(&mut self, el: V) {
        self.add(el, 1)
    }

    pub fn add(&mut self, el: V, count: usize) {
        *self.0.entry(el).or_insert(0) += count;
    }

    pub fn get<'b, Q: ?Sized>(&self, el: &'b Q) -> Option<usize>
    where
        V: Borrow<Q>,
        Q: Equivalent<V> + Hash,
    {
        self.0.get(el).copied()
    }

    pub fn get_or_zero<'b, Q: ?Sized>(&self, el: &'b Q) -> usize
    where
        V: Borrow<Q>,
        Q: Equivalent<V> + Hash,
    {
        self.get(el).unwrap_or(0)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&V, &usize)> {
        self.0.iter()
    }

    pub fn counts(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter().map(|(_, &c)| c)
    }
}

// impl<'b, V: Hash +Eq, Q> FromIterator<&'b Q> for Counter<V>
//         where V: Borrow<Q> + From<&'b Q>, Q: Equivalent<V> + Hash
// {
//     fn from_iter<T: IntoIterator<Item = &'b Q>>(iter: T) -> Self {
//         let mut counts = Self::new();
//         iter.into_iter().for_each(|el| counts.add_ref(el));
//         counts
//     }
// }

impl<'b, V: Hash + Eq> FromIterator<V> for Counter<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut counts = Self::new();
        iter.into_iter().for_each(|el| counts.add_one(el));
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_nums() {
        let counts: Counter<_> = [1, 2, 3, 1, 2, 4, 1].iter().collect();
        assert_eq!(counts.get(&0), None);
        assert_eq!(counts.get(&1), Some(3));
        assert_eq!(counts.get(&2), Some(2));
        assert_eq!(counts.get(&3), Some(1));
        assert_eq!(counts.get(&4), Some(1));
    }

    #[test]
    fn for_strs() {
        // test for words which are string slices
        let words = "apple cusu mano apple mano cusu mano kuzu".to_string();
        let counts: Counter<_> = words.split_whitespace().collect();
        assert_eq!(counts.get("manu"), None);
        assert_eq!(counts.get("mano"), Some(3));
        assert_eq!(counts.get("apple"), Some(2));
        assert_eq!(counts.get("cusu"), Some(2));
        assert_eq!(counts.get("kuzu"), Some(1));
    }
}
