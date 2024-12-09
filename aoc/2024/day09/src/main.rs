use std::collections::VecDeque;

use utils::get_stdinput;

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = Diskmap;

#[derive(Debug, Clone)]
struct Diskmap {
    base_id: usize,
    data: VecDeque<u8>,
}

#[derive(Debug, Clone)]
struct IdMap {
    // (id, start_pos, size)
    blocks: Vec<(usize, usize, u8)>,
    // (start_pos, size)
    free_space: Vec<(usize, u8)>,
}

impl IdMap {
    fn new(blocks: Vec<(usize, usize, u8)>, free_space: Vec<(usize, u8)>) -> Self {
        Self { blocks, free_space }
    }
}

impl Diskmap {
    fn to_idmap(&self) -> IdMap {
        let data: (Vec<_>, Vec<_>) = self
            .data
            .iter()
            .enumerate()
            .scan(0, |&mut pos, (i, &b)| {
                let id = (i % 2 == 0).then_some(self.base_id + i / 2);
                pos += b as usize;
                id.map(|id| (id, pos, b)).ok_or((pos, b))
            })
            .collect();
        IdMap::new(data)
    }

    fn new(mut data: VecDeque<u8>) -> Self {
        if (data.len() + 1) % 2 != 0 {
            data.pop_back();
        }
        if data.iter().any(|&b| b > 9) {
            panic!("invalid input");
        }
        Self { base_id: 0, data }
    }

    fn compact_checksum(mut self) -> usize {
        fn sum_for(id: usize, start_pos: usize, size: usize) -> usize {
            id * (2 * start_pos + size - 1) * size / 2
        }
        let mut sum = 0;
        let mut pos = 0;
        while let Some((block_len, id)) = self.pop_first() {
            if let Some(id) = id {
                // data block, count it
                println!("id: {}, start_pos: {}, len: {}", id, pos, block_len);
                sum += sum_for(id, pos, block_len as usize);
                pos += block_len as usize;
            } else {
                // empty block, let's fill it
                println!("id: ., start_pos: {}, len: {}, backfilling", pos, block_len);
                let (popped, remaining) = self.pop_blocks(block_len as usize);
                for (block_len, id) in popped {
                    println!("   id: {}, start_pos: {}, len: {}", id, pos, block_len);
                    sum += sum_for(id, pos, block_len as usize);
                    pos += block_len as usize;
                }
                if remaining > 0 {
                    println!("early exiting cause empty vector: {}", remaining);
                }
            }
        }
        sum
    }

    // pop `count` blocks,
    // returning a block length and file id per each file touched by the pop
    // (in reverse order of popping, as in, last in the original data returned first)
    // and the count of non-popped blocks if we ran out
    fn pop_blocks(&mut self, mut count: usize) -> (Vec<(u8, usize)>, usize) {
        if self.data.is_empty() {
            return (vec![], count);
        }
        let mut last = self.data.len() - 1;
        debug_assert!(last % 2 == 0);
        let mut popped = vec![];
        while self.data[last] as usize <= count {
            count -= self.data[last] as usize;
            popped.push((self.data[last], self.base_id + last / 2));
            self.data.pop_back();
            self.data.pop_back();
            match last.checked_sub(2) {
                Some(l) => last = l,
                None => break,
            };
        }
        if count > 0 && self.data.len() > 0 {
            // conversion is fine because we know that data[last] was >= count
            popped.push((count as u8, self.base_id + last / 2));
            self.data[last] -= count as u8;
            count = 0;
        }

        (popped, count)
    }

    fn pop_first(&mut self) -> Option<(u8, Option<usize>)> {
        let block_len = self.data.pop_front()?;
        let mut id = None;
        if self.data.len() % 2 == 0 {
            // we popped a data block
            id = Some(self.base_id);
            self.base_id += 1;
        }
        // else we popped an empty-data block
        Some((block_len, id))
    }
}

fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let data = lines
        .next()
        .expect("should have a single line")
        .as_ref()
        .as_bytes()
        .iter()
        .map(|b| *b - b'0')
        .collect();
    Input::new(data)
}

fn solve1(input: &Input) -> usize {
    let input = input.clone();
    input.compact_checksum()
}

fn solve2(input: &Input) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockpop() {
        let mut disk = Diskmap::new([4, 0, 3, 0, 3, 0, 4].into_iter().collect());
        let (popped, remaining) = disk.pop_blocks(3);
        assert_eq!(popped, vec![(3, 3)]);
        assert_eq!(remaining, 0);
        assert_eq!(&disk.data, &[4, 0, 3, 0, 3, 0, 1]);
        let (popped, remaining) = disk.pop_blocks(0);
        assert_eq!(popped, vec![]);
        assert_eq!(remaining, 0);
        assert_eq!(&disk.data, &[4, 0, 3, 0, 3, 0, 1]);
        let (popped, remaining) = disk.pop_blocks(5);
        assert_eq!(popped, vec![(1, 3), (3, 2), (1, 1)]);
        assert_eq!(remaining, 0);
        assert_eq!(&disk.data, &[4, 0, 2]);
        let (popped, remaining) = disk.pop_blocks(2);
        assert_eq!(popped, vec![(2, 1)]);
        assert_eq!(remaining, 0);
        assert_eq!(&disk.data, &[4]);
        let (popped, remaining) = disk.pop_blocks(6);
        assert_eq!(popped, vec![(4, 0)]);
        assert_eq!(remaining, 2);
        assert_eq!(&disk.data, &[]);
        let (popped, remaining) = disk.pop_blocks(2);
        assert_eq!(popped, vec![]);
        assert_eq!(remaining, 2);
        assert_eq!(&disk.data, &[]);
        let (popped, remaining) = disk.pop_blocks(0);
        assert_eq!(popped, vec![]);
        assert_eq!(remaining, 0);
        assert_eq!(&disk.data, &[]);
    }

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 1928);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), ());
    }
}
