use std::collections::HashSet;

use utils::get_stdinput;

fn main() {
    let input = get_stdinput();
    let mut parsed = parse(input);
    let p1 = solve1(&mut parsed);
    println!("sol1: {p1:?}");
    let p2 = solve2(&mut parsed);
    println!("sol2: {p2:?}");
}

type PageNo = u8;
type Rule = (PageNo, PageNo);
type Rules = HashSet<Rule>;
type Update = Vec<PageNo>;
type Updates = Vec<Update>;
type Input = (Rules, Updates);

fn parse_rule(s: impl AsRef<str>) -> Rule {
    let (a, b) = s.as_ref().split_once("|").expect("rule should have a |");
    let a = a.parse().expect("rule should be a number");
    let b = b.parse().expect("rule should be a number");
    (a, b)
}

fn parse_update(s: impl AsRef<str>) -> Update {
    s.as_ref()
        .split(',')
        .map(|n| n.parse().expect("update should be a number"))
        .collect()
}

fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Input {
    let rules = (&mut lines)
        .take_while(|l| !l.as_ref().is_empty())
        .map(parse_rule)
        .collect();
    let updates = lines
        .take_while(|l| !l.as_ref().is_empty())
        .map(parse_update)
        .collect();
    (rules, updates)
}

fn solve1(input: &mut Input) -> usize {
    let (rules, updates) = input;
    updates
        .iter()
        .filter(|u| check(u, rules))
        .map(takemid)
        .sum()
}

fn check(update: &Update, rules: &Rules) -> bool {
    for (fst, snd) in rules {
        let Some(i) = update.iter().position(|&n| n == *fst) else {
            // rule does not apply
            continue;
        };
        let Some(j) = update.iter().position(|&n| n == *snd) else {
            // rule does not apply
            continue;
        };
        if i > j {
            // rule violated
            return false;
        }
    }
    true
}

fn takemid(u: &Update) -> usize {
    u[u.len() / 2] as usize
}

fn solve2(input: &mut Input) -> usize {
    let (rules, updates) = input;
    updates
        .iter_mut()
        .filter(|u| !check(u, rules))
        .map(|u| {
            fix(u, rules);
            &*u
        })
        .map(takemid)
        .sum()
}

fn fix(u: &mut Update, rules: &Rules) {
    u.sort_by(|&a, &b| {
        if rules.contains(&(a, b)) {
            std::cmp::Ordering::Less
        } else if rules.contains(&(b, a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("../test");
        let mut input = parse(input.lines());
        assert_eq!(solve1(&mut input), 143);
    }

    #[test]
    fn test2() {
        let input = include_str!("../test");
        let mut input = parse(input.lines());
        assert_eq!(solve2(&mut input), 123);
    }

    #[test]
    fn input1() {
        let input = include_str!("../input");
        let mut input = parse(input.lines());
        assert_eq!(solve1(&mut input), 3608);
    }

    #[test]
    fn input2() {
        let input = include_str!("../input");
        let mut input = parse(input.lines());
        assert_eq!(solve2(&mut input), 4922);
    }
}
