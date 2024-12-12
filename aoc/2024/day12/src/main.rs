use std::collections::{HashMap, HashSet};

use utils::{
    get_stdinput,
    grid::{Dir, VecMat},
    Counter,
};

fn main() {
    let input = get_stdinput();
    let parsed = parse(input);
    let p1 = solve1(&parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&parsed);
    println!("sol2: {p2:?}");
}
type Input = VecMat<u8>;

fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    lines
        .filter(|line| !line.as_ref().is_empty())
        .map(|line| line.as_ref().as_bytes().to_vec())
        .collect::<Vec<_>>()
        .try_into()
        .expect("should be a square matrix")
}

fn solve1(input: &Input) -> usize {
    // let mut perims = Counter::new();
    // let mut areas = Counter::new();
    let mut count = 0;
    let mut visited = VecMat::filled(input.shape(), &false);
    // println!("in: {:?} vis: {:?}", input.shape(), visited.shape());
    // println!("{}", input);

    for (start, &region) in input.iter_pos() {
        if *visited.get(start).expect("should be in bounds") {
            continue;
        }
        let mut perimeter = 0;
        let mut area = 0;
        let mut stack = vec![start];
        while let Some(p) = stack.pop() {
            let v = visited
                .get_mut(p)
                .expect("should be checked by the push condition");
            if *v {
                continue;
            }
            *v = true;
            area += 1;
            for n in p.neighbours() {
                if input.get(n).copied() == Some(region) {
                    // println!("pushing {:?}", n);
                    stack.push(n);
                } else {
                    perimeter += 1;
                }
            }
        }
        count += area * perimeter;
    }
    count
}

fn solve2(input: &Input) -> usize {
    let mut count = 0;
    let mut visited = VecMat::filled(input.shape(), &false);
    // println!("in: {:?} vis: {:?}", input.shape(), visited.shape());
    // println!("{}", input);

    for (start, &region) in input.iter_pos() {
        if *visited.get(start).expect("should be in bounds") {
            continue;
        }
        let mut sides = [0; 4].map(|_| HashMap::new());
        let mut area = 0;
        let mut stack = vec![start];
        while let Some(p) = stack.pop() {
            let v = visited
                .get_mut(p)
                .expect("should be checked by the push condition");
            if *v {
                continue;
            }
            *v = true;
            area += 1;
            for (i, d) in Dir::HORIZONTAL.into_iter().enumerate() {
                let n = p.neighbour(d);
                if input.get(n).copied() == Some(region) {
                    stack.push(n);
                } else {
                    sides[2 * i].entry(p.x).or_insert_with(Vec::new).push(p.y);
                }
            }
            for (i, d) in Dir::VERTICAL.into_iter().enumerate() {
                let n = p.neighbour(d);
                if input.get(n).copied() == Some(region) {
                    stack.push(n);
                } else {
                    sides[2 * i + 1]
                        .entry(p.y)
                        .or_insert_with(Vec::new)
                        .push(p.x);
                }
            }
        }
        println!(
            "region with {} has area: {} sides: {:?}",
            region as char, area, sides
        );
        let perimeter: usize = sides.into_iter().map(count_sides).sum();
        println!("perimeter: {}", perimeter);
        count += area * perimeter;
    }
    count
}

fn count_sides<T>(perim: HashMap<T, Vec<usize>>) -> usize {
    let mut sides = 0;
    for (_, mut segments) in perim.into_iter() {
        segments.sort();
        let mut runs = 1;
        println!("segments: {:?}", segments);
        let (current, rest) = segments.split_first().expect("should have at least one");
        let mut current = *current;
        for &next in rest {
            if next - current > 1 {
                runs += 1;
            }
            current = next;
        }
        sides += runs;
    }
    println!("sides: {}", sides);
    sides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve1(&input), 1930);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let input = parse(input.lines());
        assert_eq!(solve2(&input), 1206);
    }
}
